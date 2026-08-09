use std::{
    collections::{HashMap, VecDeque},
    io::Error,
    sync::{Arc, Mutex, RwLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use tokio::{
    sync::oneshot::{self, Sender},
    time::timeout,
};

#[async_trait]
pub trait Db: Send + Sync {
    fn set(&self, key: &str, value: &DbEntry) -> Result<(), Error>;
    fn get(&self, key: &str) -> Option<DbEntry>;
    async fn get_blocking(&self, key: &str, timeout_secs: Option<f64>) -> Option<DbEntry>;
}

#[derive(Clone, PartialEq, Debug)]
pub enum RedisType {
    String(String),
    List(Vec<String>),
}

#[derive(Clone, PartialEq, Debug)]
pub struct DbEntry {
    pub value: RedisType,
    pub px: Option<u64>,
    pub created_at: u64,
}

impl DbEntry {
    pub fn new(value: RedisType) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("We seem to be before 1970-01-01")
            .as_millis() as u64;
        DbEntry {
            value,
            px: None,
            created_at: now,
        }
    }

    pub fn is_expired(&self) -> bool {
        // Is now later than px milliseconds after the creation time?
        match self.px {
            Some(px) => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("We seem to be before 1970-01-01")
                    .as_millis() as u64;
                now > self.created_at + px
            }
            None => false,
        }
    }
}

#[derive(Clone)]
pub struct MemoryDb {
    data: Arc<RwLock<HashMap<String, DbEntry>>>,
    waiters: Arc<Mutex<HashMap<String, VecDeque<Sender<DbEntry>>>>>,
}

impl MemoryDb {
    pub fn new() -> Self {
        let data = Arc::new(RwLock::new(HashMap::new()));
        let waiters = Arc::new(Mutex::new(HashMap::new()));
        MemoryDb { data, waiters }
    }
}

#[async_trait]
impl Db for MemoryDb {
    fn set(&self, key: &str, value: &DbEntry) -> Result<(), Error> {
        // Always expect potential waiters and lock the waiters structure when writing
        {
            let mut waiter_map = self.waiters.lock().unwrap();

            // Write data
            {
                let mut data = self.data.write().unwrap();
                // Special case: Empty array. That removes the key, if it exists
                // NOTE: This should probably be migrated to a remove/delete functionality
                match &value.value {
                    RedisType::List(items) if items.is_empty() => {
                        data.remove(key);
                        return Ok(());
                    }
                    _ => {
                        data.insert(key.to_string(), value.clone());
                    }
                }
            }

            // Notify waiters, if any
            {
                if let Some(waiter_q) = waiter_map.get_mut(&key.to_string()) {
                    while let Some(tx) = waiter_q.pop_front() {
                        if tx.send(value.clone()).is_ok() {
                            println!("Woke up waiter");
                            break;
                        }
                    }
                    // Remove waiter list if empty
                    if waiter_q.is_empty() {
                        waiter_map.remove(key);
                    }
                }
            }
        }
        Ok(())
    }

    fn get(&self, key: &str) -> Option<DbEntry> {
        let entry_opt;

        // Retrieve the entry, if any, from the Db
        {
            let data = self.data.read().unwrap();
            entry_opt = data.get(key).cloned();
        }

        // If the entry is expired, remove it from the Db
        match entry_opt {
            Some(entry) => match entry.is_expired() {
                false => Some(entry),
                true => {
                    self.data.write().unwrap().remove(key);
                    None
                }
            },
            None => None,
        }
    }

    async fn get_blocking(&self, key: &str, timeout_secs: Option<f64>) -> Option<DbEntry> {
        // Get an entry, possibly blocking until it exists

        // Prepare a one-shot channel for the waiter
        let (tx, rx) = oneshot::channel();

        // First, get a lock on the waiters to atomically check for an element, return it or
        // start waiting on the key
        {
            let mut map = self.waiters.lock().unwrap();

            if let Some(entry) = self.get(key) {
                return Some(entry);
            }

            // No entry was found, add a waiter into the table
            map.entry(key.into()).or_default().push_back(tx);
        }
        // Wait for the result and return it
        // Wait "forever" is interpreted as many seconds
        match timeout_secs {
            Some(0.0) | None => rx.await.ok(),
            Some(s) => match timeout(Duration::from_millis((s * 1_000.0) as u64), rx).await {
                Ok(Ok(entry)) => Some(entry),
                _ => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn test_get_set() {
        let db = MemoryDb::new();
        let key = "Key1".to_string();

        let entry1 = DbEntry::new(RedisType::String("Value1".into()));

        let entry2 = DbEntry::new(RedisType::String("Value2".into()));

        // Set and get a value
        db.set(&key, &entry1).unwrap();
        assert_eq!(db.get(&key), Some(entry1));

        // Overwrite a value
        db.set(&key, &entry2).unwrap();
        assert_eq!(db.get(&key), Some(entry2));
    }

    #[test]
    fn test_is_expired() {
        let mut entry = DbEntry::new(RedisType::String("Val".into()));
        // no PX set
        assert_eq!(entry.is_expired(), false);

        // 10s in the future, the entry should  not be expired yet
        entry.px = Some(10_000);
        assert_eq!(entry.is_expired(), false);

        // Let the entry expire immediately
        entry.px = Some(0);
        // after 2 milliseconds, it should be expired
        thread::sleep(Duration::from_millis(2));
        assert_eq!(entry.is_expired(), true);
    }
}
