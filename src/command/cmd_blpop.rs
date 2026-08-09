use async_trait::async_trait;

use crate::{
    Resp::{self},
    command::Command,
    db::{Db, RedisType},
};

pub struct CommandBlpop {}

#[async_trait]
impl Command for CommandBlpop {
    async fn execute(&self, db: &dyn Db, args: &[Resp]) -> Resp {
        let mut arg_iter = args.iter();

        // Parse arguments
        let key = match arg_iter.next() {
            Some(Resp::BulkString(s)) => s.clone(),
            _ => return Resp::NullBulkString,
        };

        let timeout_secs = match arg_iter.next() {
            Some(Resp::BulkString(s)) => s.parse::<f64>().ok(),
            _ => None,
        };

        // Retrieve DB entry and extract list
        let mut db_entry = match db.get_blocking(&key, timeout_secs).await {
            Some(entry) => entry,
            None => return Resp::NullBulkString,
        };

        let mut list = match db_entry.value {
            RedisType::List(list) => list,
            _ => return Resp::NullBulkString,
        };

        if list.is_empty() {
            return Resp::NullBulkString;
        };

        // We have a list and can remove elements
        let result_resp = Resp::Array(vec![
            Resp::BulkString(key.clone()),
            Resp::BulkString(list.remove(0)),
        ]);

        // Finally, write the modified list and return the result
        db_entry.value = RedisType::List(list);
        match db.set(&key, &db_entry) {
            Ok(_) => result_resp,
            Err(_) => Resp::Error("Could not write modified list back".into()),
        }
    }
}
