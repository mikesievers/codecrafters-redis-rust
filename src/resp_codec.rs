use crate::parser::parse_resp;
use bytes::{Buf, BufMut};
use nom::AsBytes;
use std::io;
use tokio_util::codec::{Decoder, Encoder};

use crate::Resp;

pub struct RespCodec {}

impl Decoder for RespCodec {
    type Item = Resp;

    type Error = io::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let buf_len = src.len();
        match parse_resp(src.as_bytes()) {
            // Happy path: a full frame has been found, advance buffer by parsed size
            // and return the parsed struct
            Ok((rest, resp)) => {
                src.advance(buf_len - rest.len());
                return Ok(Some(resp));
            }
            // Nom has determined that data is not sufficient to parse a complete frame
            Err(nom::Err::Incomplete(_)) => Ok(None),
            // Catchall - data does not match any configured parser
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Could not parse frame",
                ));
            }
        }
    }
}

impl Encoder<Resp> for RespCodec {
    type Error = io::Error;

    fn encode(&mut self, resp: Resp, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let payload = match resp {
            Resp::Simple(s) => "something",
            _ => "else",
        };
        dst.put_slice(payload.as_bytes());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn test_decode_happy() {
        let mut src = bytes::BytesMut::from("$3\r\nhey\r\n".as_bytes());
        let mut codec = RespCodec {};
        let result = codec.decode(&mut src);

        assert_eq!(result.unwrap(), Some(Resp::BulkString("hey".into())));
    }

    #[test]
    fn test_decode_incomplete() {
        let mut src = bytes::BytesMut::from("$3\r\nhey\r".as_bytes());
        let mut codec = RespCodec {};
        let result = codec.decode(&mut src);

        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_decode_error() {
        let mut src = bytes::BytesMut::from("$3\r\nheyWRONG\r\n".as_bytes());
        let mut codec = RespCodec {};
        let result = codec.decode(&mut src);

        assert_matches!(result, Err(_));
    }
}
