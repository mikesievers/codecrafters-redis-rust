use std::iter;

use nom::bytes::streaming::{tag, take_until};
use nom::{self, IResult};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let (rest, i) = parse_number("123\r\n".as_bytes()).unwrap();
        assert_eq!(i, 123);
        assert_eq!(rest, []);
    }
}
