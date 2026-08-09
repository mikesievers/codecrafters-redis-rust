use async_trait::async_trait;

use crate::{Resp, command::Command, db::Db};

pub struct CommandPing {}

#[async_trait]
impl Command for CommandPing {
    async fn execute(&self, _db: &dyn Db, _args: &[Resp]) -> Resp {
        Resp::Simple("PONG".into())
    }
}
