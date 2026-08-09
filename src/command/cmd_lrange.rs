use async_trait::async_trait;

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

#[async_trait]
impl Command for CommandLrange {
    async fn execute(&self, db: &dyn Db, args: &[Resp]) -> Resp {
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
    let empty_array = Resp::Array(vec![]);

    let (slice_start, slice_end) = match determine_slice_indices(start, end, items.len() as i64) {
        Some(value) => value,
        None => return empty_array,
    };

    let slice = &items[(slice_start)..=(slice_end)];

    Resp::Array(slice.iter().map(|s| Resp::BulkString(s.clone())).collect())
}

fn determine_slice_indices(start: &i64, end: &i64, items_len: i64) -> Option<(usize, usize)> {
    // Ensure 0-length arrays do not lead to errors below
    if items_len == 0 {
        return None;
    }
    let slice_start = match *start {
        // negative
        // and bigger than the length of items
        s if s < -items_len => 0,
        s if s >= -items_len && s < 0 => items_len + s,
        s if s > items_len => return None,
        // By default, return as is
        s => s,
    };
    let slice_end = match *end {
        // negative
        // and bigger than the length of items
        s if s < -items_len => 0,
        s if s >= -items_len && s < 0 => items_len + s,
        s if s > items_len => items_len - 1,
        // By default, return as is
        s => s,
    };

    // The slice index must be before the end
    if slice_end < slice_start {
        return None;
    }

    Some((slice_start as usize, slice_end as usize))
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0, 1, Some((0, 0)))]
    #[case(-2, -1, 2, Some((0, 1)))]
    #[case(-2, -1, 1, Some((0, 0)))]
    #[case(0, 3, 7, Some((0, 3)))]
    fn test_determine_slice_indices(
        #[case] start: i64,
        #[case] end: i64,
        #[case] items_len: i64,
        #[case] expected_result: Option<(usize, usize)>,
    ) {
        let result = determine_slice_indices(&start, &end, items_len);
        assert_eq!(result, expected_result);
    }
}
