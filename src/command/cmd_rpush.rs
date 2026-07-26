use crate::command::Command;

pub struct CommandRpush {}

impl Command for CommandRpush {
    fn execute(&self, db: &dyn crate::db::Db, args: &[crate::Resp]) -> crate::Resp {
        todo!()
    }
}
