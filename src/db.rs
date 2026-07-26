use std::{
    collections::HashMap,
    io::Error,
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

pub trait Db {
    fn set(&self, key: &str, value: &DbEntry) -> Result<(), Error>;
    fn get(&self, key: &str) -> Option<DbEntry>;
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
    pub fn new(value: &String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("We seem to be before 1970-01-01")
            .as_millis() as u64;
        DbEntry {
            value: RedisType::String(value.clone()),
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
}

impl MemoryDb {
    pub fn new() -> Self {
        let data = Arc::new(RwLock::new(HashMap::new()));
        MemoryDb { data }
    }
}

impl Db for MemoryDb {
    fn set(&self, key: &str, value: &DbEntry) -> Result<(), Error> {
        {
            let mut data = self.data.write().unwrap();
            data.insert(key.to_string(), value.clone());
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
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn test_get_set() {
        let db = MemoryDb::new();
        let key = "Key1".to_string();

        let entry1 = DbEntry::new(&"Value1".into());

        let entry2 = DbEntry::new(&"Value2".into());

        // Set and get a value
        db.set(&key, &entry1).unwrap();
        assert_eq!(db.get(&key), Some(entry1));

        // Overwrite a value
        db.set(&key, &entry2).unwrap();
        assert_eq!(db.get(&key), Some(entry2));
    }

    #[test]
    fn test_is_expired() {
        let mut entry = DbEntry::new(&"Val".into());
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
