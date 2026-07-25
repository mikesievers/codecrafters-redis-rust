use crate::{
    Resp,
    command::Command,
    db::{Db, DbEntry},
};

pub struct CommandSet {}

impl Command for CommandSet {
    fn execute(&self, db: &dyn Db, args: &[Resp]) -> Resp {
        match args {
            [Resp::BulkString(key), Resp::BulkString(val)] => {
                let entry = DbEntry { value: val.clone() };
                match db.set(key, &entry) {
                    Ok(_) => Resp::Simple("OK".into()),
                    Err(_) => Resp::Error("Could not set value".into()),
                }
            }
            _ => Resp::Error("Wrong arguments for set".into()),
        }
    }
}

pub struct CommandGet {}

impl Command for CommandGet {
    fn execute(&self, db: &dyn Db, args: &[Resp]) -> Resp {
        match args {
            [Resp::BulkString(key)] => match db.get(key) {
                Some(entry) => Resp::BulkString(entry.value),
                None => Resp::NullBulkString,
            },
            _ => Resp::Error("Wrong arguments for get".into()),
        }
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

        let command_set = CommandSet {};
        let _ = command_set.execute(&db, &args);

        let command_get = CommandGet {};
        let result = command_get.execute(&db, &vec![key]);
        assert_eq!(result, value);
    }
}
