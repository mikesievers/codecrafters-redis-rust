use crate::{Resp, command::Command};

pub struct CommandEcho {}

impl Command for CommandEcho {
    fn execute(&self, _db: &dyn crate::db::Db, args: &[Resp]) -> Resp {
        args.first().cloned().unwrap_or(Resp::Simple("".into()))
    }
}
