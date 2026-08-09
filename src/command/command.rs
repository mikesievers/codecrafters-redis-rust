use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::{
    Resp,
    command::{
        CommandBlpop, CommandEcho, CommandGet, CommandLlen, CommandLpop, CommandLpush,
        CommandLrange, CommandPing, CommandRpush, CommandSet,
    },
    db::Db,
};

#[async_trait]
pub trait Command {
    async fn execute(&self, db: &dyn Db, args: &[Resp]) -> Resp;
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
                "LPUSH",
                Box::new(CommandLpush {}) as Box<dyn Command + Send + Sync>,
            ),
            (
                "LLEN",
                Box::new(CommandLlen {}) as Box<dyn Command + Send + Sync>,
            ),
            (
                "BLPOP",
                Box::new(CommandBlpop {}) as Box<dyn Command + Send + Sync>,
            ),
            (
                "LPOP",
                Box::new(CommandLpop {}) as Box<dyn Command + Send + Sync>,
            ),
            (
                "LRANGE",
                Box::new(CommandLrange {}) as Box<dyn Command + Send + Sync>,
            ),
        ]));
        CommandRegistry { commands }
    }
}
