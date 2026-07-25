use crate::{Resp, command::Command, db::Db};

pub struct CommandPing {}

impl Command for CommandPing {
    fn execute(&self, _db: &dyn Db, _args: &[Resp]) -> Resp {
        Resp::Simple("PONG".into())
    }
}
