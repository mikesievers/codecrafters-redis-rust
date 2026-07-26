use crate::{
    Resp,
    command::Command,
    db::{Db, DbEntry},
};

// SET
pub struct CommandSet {}

impl Command for CommandSet {
    fn execute(&self, db: &dyn Db, args: &[Resp]) -> Resp {
        match parse_get_args(args) {
            Ok((key, entry)) => match db.set(key, &entry) {
                Ok(_) => Resp::Simple("OK".into()),
                Err(_) => Resp::Error("Could not set value".into()),
            },
            Err(s) => return Resp::Error(s),
        }
    }
}

// GET
pub struct CommandGet {}

impl Command for CommandGet {
    fn execute(&self, db: &dyn Db, args: &[Resp]) -> Resp {
        match args {
            [Resp::BulkString(key)] => match db.get(key) {
                Some(entry) => Resp::BulkString(entry.value),
                None => Resp::NullBulkString,
            },
            _ => Resp::Error("Wrong arguments for get".into()),
        }
    }
}

// Parse parameters for GET
fn parse_get_args(args: &[Resp]) -> Result<(&String, DbEntry), String> {
    // All Parameters are BulkStrings
    // The first two parameters must be a key and a value
    if args.len() == 0 {
        return Err("No key and value supplied for SET".into());
    }

    let (key, mut entry) = match (&args[0], &args[1]) {
        (Resp::BulkString(k), Resp::BulkString(v)) => (k, DbEntry::new(&v)),
        _ => {
            return Err("Key and value must exist and be strings".into());
        }
    };

    Ok((key, entry))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::MemoryDb;

    #[test]
    fn test_parse_get_params() {
        let args_set_with_px = vec![
            Resp::BulkString("set".into()),
            Resp::BulkString("k".into()),
            Resp::BulkString("v".into()),
            Resp::BulkString("Px".into()),
            Resp::BulkString("10".into()),
        ];

        match parse_get_args(&args_set_with_px) {
            Ok((key, parsed_entry)) => todo!(),
            Err(s) => todo!(),
        }

        // assert_eq!(parsed_entry.value, "V");
        // assert_eq!(parsed_entry.px, 10);
    }

    #[test]
    fn test_get_set() {
        let db = MemoryDb::new();
        let key = Resp::BulkString("key".into());
        let value = Resp::BulkString("value".into());
        let args = vec![key.clone(), value.clone()];

        let command_set = CommandSet {};
        let _ = command_set.execute(&db, &args);

        let command_get = CommandGet {};
        let result = command_get.execute(&db, &vec![key]);
        assert_eq!(result, value);
    }
}
