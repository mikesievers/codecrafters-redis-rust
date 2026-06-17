use tokio::net::{TcpListener, TcpStream};

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio_util::codec::{FramedRead, FramedWrite};

mod parser;
mod resp_codec;

use resp_codec::RespCodec;

#[derive(Debug, PartialEq, Clone)]
pub enum Resp {
    Simple(String),
    BulkString(String),
    Error(String),
    Int(i64),
    Array(Vec<Resp>),
    //NullArray,
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

async fn handle_stream(mut stream: TcpStream) -> Result<()> {
    // let mut writer = stream.try_clone()?;
    // let reader = BufReader::new(&stream);

    let (raw_reader, raw_writer) = stream.split();

    let mut reader = FramedRead::new(raw_reader, RespCodec {});
    let mut writer = FramedWrite::new(raw_writer, RespCodec {});

    while let Some(frame) = reader.next().await {
        match frame {
            Ok(resp) => {
                println!("Found: {:?}", resp);
                writer.send(handle_command(resp)).await?;
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

fn handle_command(resp: Resp) -> Resp {
    match resp {
        Resp::Simple(_) => Resp::Error("Can't handle Simple (yet)".into()),
        Resp::BulkString(_) => Resp::Error("Can't handle BulkString (yet)".into()),
        Resp::Error(_) => Resp::Error("Can't handle Error (yet)".into()),
        Resp::Int(_) => Resp::Error("Can't handle Int (yet)".into()),
        // Commands should arrive as arrays
        Resp::Array(a) if a.len() == 0 => Resp::Error("Can't handle empty arrays (yet)".into()),
        Resp::Array(resps) => {
            if let Some((command, args)) = resps.split_first() {
                match command {
                    Resp::BulkString(s) if s.to_uppercase() == "PING" => {
                        Resp::Simple("PONG".into())
                    }
                    Resp::BulkString(s) if s.to_uppercase() == "ECHO" => {
                        args.first().cloned().unwrap_or(Resp::Simple("".into()))
                    }
                    _ => todo!(),
                }
            } else {
                Resp::Error("Don't know how to handle that array".into())
            }
        }
    }
}
