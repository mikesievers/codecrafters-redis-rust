use crate::parser::parse_resp;
use bytes::{Buf, BufMut};
use itertools::Itertools;
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
                Ok(Some(resp))
            }
            // Nom has determined that data is not sufficient to parse a complete frame
            Err(nom::Err::Incomplete(_)) => Ok(None),
            // Catchall - data does not match any configured parser
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Could not parse frame",
            )),
        }
    }
}

impl Encoder<Resp> for RespCodec {
    type Error = io::Error;

    fn encode(&mut self, resp: Resp, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let payload = self._encode_resp(&resp);
        dst.put_slice(payload.as_bytes());
        Ok(())
    }
}

impl RespCodec {
    // NOTE: There is a lot of Strings being used here. An optimization
    // possibility is to prevent the extra heap allocations and append to the
    // BytesMut dst buffer instead
    fn _encode_resp(&self, resp: &Resp) -> String {
        match resp {
            Resp::Simple(s) => format!("+{}\r\n", s),
            Resp::BulkString(s) => format!("${}\r\n{}\r\n", s.len(), s),
            Resp::Error(s) => format!("-{}\r\n", s),
            Resp::Int(i) => format!(":{}\r\n", i),
            Resp::Array(resps) => format!(
                "*{}\r\n{}",
                resps.len(),
                resps
                    .iter()
                    .map(|r| self._encode_resp(r))
                    .collect_vec()
                    .join("")
            ),
        }
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

    #[test]
    fn test_encode_array() {
        let resp = Resp::Array(vec![Resp::Simple("The answer is".into()), Resp::Int(64)]);
        let codec = RespCodec {};
        let encoded = codec._encode_resp(&resp);
        assert_eq!(encoded, "*2\r\n+The answer is\r\n:64\r\n");
    }
}
