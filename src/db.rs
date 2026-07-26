use std::{
    collections::HashMap,
    io::Error,
    sync::{Arc, RwLock},
};

pub trait Db {
    fn set(&self, key: &str, value: &DbEntry) -> Result<(), Error>;
    fn get(&self, key: &str) -> Option<DbEntry>;
}

#[derive(Clone, PartialEq, Debug)]
pub struct DbEntry {
    pub value: String,
    pub px: Option<u64>,
}

impl DbEntry {
    pub fn new(value: &String) -> Self {
        DbEntry {
            value: value.clone(),
            px: None,
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
        {
            let data = self.data.read().unwrap();
            data.get(key).cloned()
        }
    }
}

#[cfg(test)]
mod tests {
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
}
