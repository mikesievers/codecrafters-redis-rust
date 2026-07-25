use crate::{Resp, db::Db};

pub trait Command<DB: Db> {
    fn execute(&self, db: &DB, args: &Option<Resp>) -> Resp;
}

#[derive(Clone)]
pub struct CommandRegistry {}

impl CommandRegistry {
    pub fn new() -> Self {
        CommandRegistry {}
    }
}
