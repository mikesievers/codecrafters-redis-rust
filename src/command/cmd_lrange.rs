use crate::Resp;
use crate::command::Command;
use crate::db::Db;
use crate::db::RedisType;

pub struct CommandLrange {}

#[derive(PartialEq, Debug)]
struct LrangeArgs {
    key: String,
    start: i64,
    end: i64,
}

impl Command for CommandLrange {
    fn execute(&self, db: &dyn Db, args: &[Resp]) -> Resp {
        match parse_lrange_args(args) {
            Ok(lrange_args) => {
                match db.get(&lrange_args.key) {
                    Some(entry) => {
                        match entry.value {
                            RedisType::List(items) => extract_and_return_values(
                                items,
                                &lrange_args.start,
                                &lrange_args.end,
                            ),
                            // If it's not a list, return an empty array
                            // NOTE: This might seem like an error but was not specified that way
                            _ => Resp::Array(vec![]),
                        }
                    }
                    // No entry found
                    None => Resp::Array(vec![]),
                }
            }
            Err(e) => Resp::Error(e),
        }
    }
}

fn extract_and_return_values(items: Vec<String>, start: &i64, end: &i64) -> Resp {
    // Guard clauses
    // Ensure 0-length arrays do not lead to errors below
    if items.len() == 0 {
        return Resp::Array(vec![]);
    }

    // Start must be positive
    if *start < 0 {
        return Resp::Array(vec![]);
    };
    // The slice index must be before the end
    if *end < *start {
        return Resp::Array(vec![]);
    }
    // Check if start is not bigger then the length of the items
    if (items.len() as i64) - 1 < *start {
        return Resp::Array(vec![]);
    };

    let slice_start = *start as usize;
    let slice_end = (*end as usize + 1).min(items.len());

    let slice = &items[slice_start..slice_end];

    Resp::Array(slice.iter().map(|s| Resp::BulkString(s.clone())).collect())
}

fn parse_lrange_args(args: &[Resp]) -> Result<LrangeArgs, String> {
    if args.len() != 3 {
        return Err("Exactly 3 arguments needed: LRANGE <key> <start> <end>".into());
    }

    let key = match &args[0] {
        Resp::BulkString(s) => s.clone(),
        _ => return Err("The first parameter must be a BulkString".into()),
    };

    let start = match &args[1] {
        Resp::BulkString(i) => i
            .parse::<i64>()
            .map_err(|_| "The second parameter must be parseable as an i64".to_string())?,
        _ => return Err("The second parameter must be a BulkString".into()),
    };

    let end = match &args[2] {
        Resp::BulkString(i) => i
            .parse::<i64>()
            .map_err(|_| "The third parameter must be parseable as an i64".to_string())?,
        _ => return Err("The second parameter must be a BulkString".into()),
    };

    Ok(LrangeArgs { key, start, end })
}
