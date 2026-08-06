use crate::{
    Resp,
    command::Command,
    db::{Db, RedisType},
};

pub struct CommandLlen {}

impl Command for CommandLlen {
    fn execute(&self, db: &dyn Db, args: &[Resp]) -> Resp {
        let resp_zero = Resp::Int(0);

        if args.is_empty() {
            return resp_zero;
        }
        match &args[0] {
            Resp::BulkString(key) => match db.get(key) {
                Some(entry) => match entry.value {
                    RedisType::List(items) => Resp::Int(items.len() as i64),
                    _ => resp_zero,
                },
                None => resp_zero,
            },
            _ => resp_zero,
        }
    }
}
