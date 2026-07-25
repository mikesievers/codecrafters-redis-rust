use crate::{Resp, db::Db};

pub fn cmd_ping() -> Resp {
    Resp::Simple("PONG".into())
}

pub trait Command<DB: Db> {
    fn execute(&self, db: &DB, args: &Option<Resp>) -> Resp;
}

pub struct CommandPing {}

impl<DB: Db> Command<DB> for CommandPing {
    fn execute(&self, _db: &DB, _args: &Option<Resp>) -> Resp {
        Resp::Simple("PONG".into())
    }
}
