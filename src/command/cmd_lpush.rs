use crate::{
    Resp,
    command::Command,
    db::{Db, DbEntry, RedisType},
};

pub struct CommandLpush {}

impl Command for CommandLpush {
    fn execute(&self, db: &dyn Db, args: &[crate::Resp]) -> Resp {
        // Identify the (new) items for the list
        let (key, mut new_elements) = match parse_list_items(args) {
            Ok((arg_key, arg_elements)) => (arg_key, arg_elements),
            Err(s) => return Resp::Error(s),
        };

        // LPUSH inserts in reverse order
        new_elements.reverse();

        // If an existing entry exists, modify that one
        // Or create a new one
        match db.get(key) {
            Some(existing_entry) => match existing_entry.value {
                RedisType::List(mut existing_vec) => {
                    new_elements.extend(existing_vec);
                    let nr_entries = new_elements.len() as i64;
                    let entry = DbEntry::new(RedisType::List(new_elements));
                    match db.set(key, &entry) {
                        Ok(()) => Resp::Int(nr_entries),
                        Err(_) => Resp::Error("Could not insert value into database".into()),
                    }
                }
                _ => return Resp::Error("Existing entry is not a list type".into()),
            },
            None => {
                let nr_entries = new_elements.len() as i64;
                let entry = DbEntry::new(RedisType::List(new_elements));
                match db.set(key, &entry) {
                    Ok(()) => Resp::Int(nr_entries),
                    Err(_) => Resp::Error("Could not insert value into database".to_string()),
                }
            }
        }
    }
}

fn parse_list_items(args: &[Resp]) -> Result<(&String, Vec<String>), String> {
    // All Parameters are BulkStrings
    // The first two parameters must be a key and a value
    if args.len() < 2 {
        return Err("No key and at least one value supplied for SET".into());
    }

    let key = match &args[0] {
        Resp::BulkString(s) => s,
        _ => return Err("Key must be a BulkString".into()),
    };

    let mut arg_strings = Vec::new();
    for arg in args.iter().skip(1) {
        match arg {
            Resp::BulkString(s) => arg_strings.push(s.clone()),
            _ => return Err("All values must be BulkStrings".into()),
        }
    }

    Ok((key, arg_strings))
}
