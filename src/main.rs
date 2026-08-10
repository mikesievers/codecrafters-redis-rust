use tokio::net::{TcpListener, TcpStream};

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio_util::codec::{FramedRead, FramedWrite};

mod command;
mod db;
mod parser;
mod resp_codec;

use command::*;
use resp_codec::RespCodec;

use crate::db::Db;

#[derive(Debug, PartialEq, Clone)]
pub enum Resp {
    Simple(String),
    BulkString(String),
    Error(String),
    Int(i64),
    Array(Vec<Resp>),
    NullArray,
    NullBulkString,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting sort-of Redis.");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let db = db::MemoryDb::new();
    let command_registry = CommandRegistry::new();

    loop {
        let stream = listener.accept().await;

        match stream {
            Ok((stream, _)) => {
                println!("accepted new connection");

                tokio::spawn(handle_stream(db.clone(), command_registry.clone(), stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_stream<T: Db + Clone + Send + Sync + 'static>(
    db: T,
    command_registry: CommandRegistry,
    mut stream: TcpStream,
) -> Result<()> {
    let (raw_reader, raw_writer) = stream.split();

    let mut reader = FramedRead::new(raw_reader, RespCodec {});
    let mut writer = FramedWrite::new(raw_writer, RespCodec {});

    while let Some(frame) = reader.next().await {
        match frame {
            Ok(resp) => {
                println!("Found: {:?}", resp);
                writer
                    .send(handle_command(&db, &command_registry, resp).await)
                    .await?;
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

async fn handle_command(db: &dyn Db, command_registry: &CommandRegistry, resp: Resp) -> Resp {
    match resp {
        Resp::Array(resps) => {
            if let Some((cmd_part, args)) = resps.split_first() {
                // The first element of the Resp Vec must be a BulkString..
                if let Resp::BulkString(cmd_name) = cmd_part {
                    // ... and, uppercased, ...
                    let cmd_upper = cmd_name.to_uppercase();
                    // ... must be a known command
                    match command_registry.commands.get(cmd_upper.as_str()) {
                        Some(command) => command.execute(db, &args).await,
                        None => Resp::Error("Command not implemented".into()),
                    }
                } else {
                    Resp::Error("First element must be a BulkString".into())
                }
            } else {
                Resp::Error("Don't know how to handle that array".into())
            }
        }
        _ => Resp::Error("Commands must be Arrays of BulkStrings".into()),
    }
}
