use async_trait::async_trait;

use crate::{
    Resp,
    command::Command,
    db::{Db, RedisType},
};

pub struct CommandLlen {}

#[async_trait]
impl Command for CommandLlen {
    async fn execute(&self, db: &dyn Db, args: &[Resp]) -> Resp {
        args.first()
            .and_then(|arg| match arg {
                Resp::BulkString(key) => Some(key),
                _ => None,
            })
            .and_then(|key| db.get(key))
            .and_then(|entry| match entry.value {
                RedisType::List(items) => Some(Resp::Int(items.len() as i64)),
                _ => None,
            })
            .unwrap_or(Resp::Int(0))
    }
}
