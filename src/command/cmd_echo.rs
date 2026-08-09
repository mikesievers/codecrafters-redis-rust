use async_trait::async_trait;

use crate::{Resp, command::Command};

pub struct CommandEcho {}

#[async_trait]
impl Command for CommandEcho {
    async fn execute(&self, _db: &dyn crate::db::Db, args: &[Resp]) -> Resp {
        args.first().cloned().unwrap_or(Resp::Simple("".into()))
    }
}
