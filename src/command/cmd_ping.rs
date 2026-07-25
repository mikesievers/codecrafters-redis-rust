use crate::{Resp, command::Command, db::Db};

pub struct CommandPing {}

impl<DB: Db> Command<DB> for CommandPing {
    fn execute(&self, _db: &DB, _args: &Option<Resp>) -> Resp {
        Resp::Simple("PONG".into())
    }
}
