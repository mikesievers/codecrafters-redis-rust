use itertools::Itertools;

use crate::{
    Resp::{self},
    command::Command,
    db::{Db, RedisType},
};

pub struct CommandLpop {}

impl Command for CommandLpop {
    fn execute(&self, db: &dyn Db, args: &[Resp]) -> Resp {
        let mut arg_iter = args.iter();

        // Parse arguments
        let key = match arg_iter.next() {
            Some(Resp::BulkString(s)) => s.clone(),
            _ => return Resp::NullBulkString,
        };

        let nr_elements = match arg_iter.next() {
            Some(Resp::BulkString(s)) => s.parse::<usize>().ok(),
            _ => None,
        };

        // Retrieve DB entry and extract list
        let mut db_entry = match db.get(&key) {
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
        let result_resp = match nr_elements {
            None => Resp::BulkString(list.remove(0)),
            Some(nr_elements) => Resp::Array(
                list.drain(..nr_elements.min(list.len()))
                    .map(|s| Resp::BulkString(s))
                    .collect_vec(),
            ),
        };

        // Finally, write the modified list and return the result
        db_entry.value = RedisType::List(list);
        match db.set(&key, &db_entry) {
            Ok(_) => result_resp,
            Err(_) => Resp::Error("Could not write modified list back".into()),
        }
    }
}
