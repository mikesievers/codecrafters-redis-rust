use crate::{Resp, db::Db};

pub fn cmd_set<T: Db>(mut db: T, args: &[Resp]) -> Resp {
    match args {
        [Resp::BulkString(key), Resp::BulkString(val)] => match db.set(key, val) {
            Ok(_) => Resp::Simple("OK".into()),
            Err(_) => Resp::Error("Could not set value".into()),
        },
        _ => Resp::Error("Wrong arguments for set".into()),
    }
}

pub fn cmd_get<T: Db>(mut db: T, args: &[Resp]) -> Resp {
    match args {
        [Resp::BulkString(key)] => match db.get(key) {
            Some(s) => Resp::BulkString(s),
            None => Resp::NullBulkString,
        },
        _ => Resp::Error("Wrong arguments for get".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::MemoryDb;

    #[test]
    fn test_get_set() {
        let db = MemoryDb::new();
        let key = Resp::BulkString("key".into());
        let value = Resp::BulkString("value".into());
        let args = vec![key.clone(), value.clone()];

        let _ = cmd_set(db.clone(), &args);

        let result = cmd_get(db, &vec![key]);
        assert_eq!(result, value);
    }
}
