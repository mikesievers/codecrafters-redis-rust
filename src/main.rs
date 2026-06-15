use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio_util::codec::{FramedRead, FramedWrite};

mod parser;
mod resp_codec;

use resp_codec::RespCodec;

#[derive(Debug, PartialEq)]
pub enum Resp {
    Simple(String),
    BulkString(String),
    Error(String),
    Int(i64),
    // TODO: Implement parser for remaining types
    // Array(Vec<Resp>),
    // NullArray,
    // NullBulkString,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting sort-of Redis.");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let stream = listener.accept().await;

        match stream {
            Ok((stream, _)) => {
                println!("accepted new connection");

                tokio::spawn(handle_stream(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_stream(stream: TcpStream) -> Result<()> {
    // let mut writer = stream.try_clone()?;
    // let reader = BufReader::new(&stream);

    let (raw_reader, raw_writer) = stream.into_split();

    let mut reader = FramedRead::new(raw_reader, RespCodec {});
    let mut writer = FramedWrite::new(raw_writer, RespCodec {});

    while let Some(frame) = reader.next().await {
        match frame {
            Ok(resp) => {
                println!("Found: {:?}", resp);
                writer.send(Resp::Simple("PONG".into())).await?;
            }
            Err(e) => {
                eprintln!("Could not decode {:?}", e);
                return Err(e.into());
            }
        }
    }

    println!("Connection closed by client");

    Ok(())
}
