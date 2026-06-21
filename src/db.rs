use std::{
    collections::HashMap,
    io::Error,
    sync::{Arc, Mutex},
};

pub trait Db {
    fn set(&mut self, key: &String, value: &String) -> Result<(), Error>;
    fn get(&mut self, key: &String) -> Option<String>;
}

#[derive(Clone)]
pub struct MemoryDb {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl MemoryDb {
    pub fn new() -> Self {
        let data = Arc::new(Mutex::new(HashMap::new()));
        MemoryDb { data }
    }
}

impl Db for MemoryDb {
    fn set(&mut self, key: &String, value: &String) -> Result<(), Error> {
        {
            let mut data = self.data.lock().unwrap();
            data.insert(key.clone(), value.clone());
        }
        Ok(())
    }

    fn get(&mut self, key: &String) -> Option<String> {
        {
            let data = self.data.lock().unwrap();
            data.get(key).cloned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set() {
        let mut db = MemoryDb::new();
        let key = "Key1".to_string();
        let value = "Value1".to_string();
        let value2 = "Value2".to_string();

        // Set and get a value
        db.set(&key, &value).unwrap();
        assert_eq!(db.get(&key), Some(value));

        // Overwrite a value
        db.set(&key, &value2).unwrap();
        assert_eq!(db.get(&key), Some(value2));
    }
}
