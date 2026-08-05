use std::{collections::HashMap, sync::Arc};

use crate::{
    Resp,
    command::{CommandEcho, CommandGet, CommandLrange, CommandPing, CommandRpush, CommandSet},
    db::Db,
};

pub trait Command {
    fn execute(&self, db: &dyn Db, args: &[Resp]) -> Resp;
}

#[derive(Clone)]
pub struct CommandRegistry {
    pub commands: Arc<HashMap<&'static str, Box<dyn Command + Send + Sync>>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let commands = Arc::new(HashMap::from([
            (
                "PING",
                Box::new(CommandPing {}) as Box<dyn Command + Send + Sync>,
            ),
            (
                "ECHO",
                Box::new(CommandEcho {}) as Box<dyn Command + Send + Sync>,
            ),
            (
                "SET",
                Box::new(CommandSet {}) as Box<dyn Command + Send + Sync>,
            ),
            (
                "GET",
                Box::new(CommandGet {}) as Box<dyn Command + Send + Sync>,
            ),
            (
                "RPUSH",
                Box::new(CommandRpush {}) as Box<dyn Command + Send + Sync>,
            ),
            (
                "LRANGE",
                Box::new(CommandLrange {}) as Box<dyn Command + Send + Sync>,
            ),
        ]));
        CommandRegistry { commands }
    }
}
