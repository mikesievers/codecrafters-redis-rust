use std::collections::HashMap;

pub trait Db {
    fn set(&mut self, key: &String, value: &String) -> ();
    fn get(&mut self, key: &String) -> Option<String>;
}

struct MemoryDb {
    data: HashMap<String, String>,
}

impl MemoryDb {
    pub fn new() -> Self {
        let data = HashMap::new();
        MemoryDb { data }
    }
}

impl Db for MemoryDb {
    fn set(&mut self, key: &String, value: &String) -> () {
        self.data.insert(key.clone(), value.clone());
    }

    fn get(&mut self, key: &String) -> Option<String> {
        self.data.get(key).cloned()
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
        db.set(&key, &value);
        assert_eq!(db.get(&key), Some(value));
    }
}
