use crate::{
    Resp,
    command::Command,
    db::{Db, RedisType},
};

pub struct CommandLpop {}

impl Command for CommandLpop {
    fn execute(&self, db: &dyn Db, args: &[Resp]) -> Resp {
        // This needs to be initilized so that it can be borrowed
        // in a closure downstream.
        let mut db_key = "UNINITIALIZED_PLACEHOLDER_IN_LPOP_WAS_USED".into();

        args.first()
            .and_then(|arg| match arg {
                Resp::BulkString(key) => {
                    db_key = key.clone();
                    db.get(key)
                }
                _ => None,
            })
            .and_then(|mut entry| match entry.value {
                RedisType::List(mut items) => {
                    if items.is_empty() {
                        None
                    } else {
                        let first_element = Resp::BulkString(items.remove(0));
                        entry.value = RedisType::List(items);
                        // If removing the element from the list and then writing it fails, we will
                        // just return an empty result. That might not be what is intended.
                        // If explicit error messages are wanted, change the ok() to a match.
                        db.set(&db_key, &entry).ok();
                        Some(first_element)
                    }
                }
                _ => None,
            })
            .unwrap_or(Resp::NullBulkString)
    }
}
