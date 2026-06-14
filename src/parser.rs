use nom::Parser;
use nom::branch::alt;
use nom::bytes::streaming::{tag, take, take_until};
use nom::{self, IResult};

use crate::Resp;

fn crlf(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("\r\n")(i)
}

fn parse_number(i: &[u8]) -> IResult<&[u8], i64> {
    let (new_i, num_bytes) = take_until("\r\n")(i)?;
    let (new_i, _) = crlf(new_i)?;
    let s = std::str::from_utf8(num_bytes)
        .map_err(|_| nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Char)))?;
    let n = s
        .parse::<i64>()
        .map_err(|_| nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Digit)))?;
    Ok((new_i, n))
}

fn parse_integer(i: &[u8]) -> IResult<&[u8], Resp> {
    let (i, _) = tag(":")(i)?;
    let (i, n) = parse_number(i)?;
    Ok((i, Resp::Int(n)))
}

fn parse_simple(i: &[u8]) -> IResult<&[u8], Resp> {
    let (i, _) = tag("+")(i)?;
    let (i, s) = take_until("\r\n")(i)?;
    let (i, _) = crlf(i)?;
    Ok((i, Resp::String(String::from_utf8_lossy(s).into())))
}

fn parse_bulk_string(i: &[u8]) -> IResult<&[u8], Resp> {
    let (i, _) = tag("$")(i)?;
    let (i, n) = parse_number(i)?;
    let (i, s) = take(n as usize)(i)?;
    let (i, _) = crlf(i)?;
    Ok((i, Resp::String(String::from_utf8_lossy(s).into())))
}

fn parse_error(i: &[u8]) -> IResult<&[u8], Resp> {
    let (i, _) = tag("-")(i)?;
    let (i, s) = take_until("\r\n")(i)?;
    let (i, _) = crlf(i)?;
    Ok((i, Resp::String(String::from_utf8_lossy(s).into())))
}

fn parse_resp(i: &[u8]) -> IResult<&[u8], Resp> {
    alt((parse_simple, parse_bulk_string, parse_integer, parse_error)).parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Resp;

    #[test]
    fn test_parse_resp() {
        // Int
        let (_, result) = parse_resp(":42\r\n".as_bytes()).unwrap();
        assert_eq!(result, Resp::Int(42));

        // Simple String
        let simple_sample = "Some String";
        let (_, result) = parse_resp(format!("+{}\r\n", simple_sample).as_bytes()).unwrap();
        assert_eq!(result, Resp::String(simple_sample.into()));

        // Bulk String
        let (_, result) =
            parse_resp(format!("${}\r\n{}\r\n", simple_sample.len(), simple_sample).as_bytes())
                .unwrap();
        assert_eq!(result, Resp::String(simple_sample.into()));

        // Error
        let error_sample = "All is broken";
        let (_, result) =
            parse_resp(format!("{}{}{}", "-", error_sample, "\r\n").as_bytes()).unwrap();
        assert_eq!(result, Resp::String(error_sample.into()));
    }

    #[test]
    fn test_parse_number() {
        let (rest, i) = parse_number("123\r\n".as_bytes()).unwrap();
        assert_eq!(i, 123);
        assert_eq!(rest, []);
    }
}
