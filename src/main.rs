use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use anyhow::Result;

mod parser;
mod resp_codec;

#[derive(Debug, PartialEq)]
enum Resp {
    String(String),
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

async fn handle_stream(mut stream: TcpStream) -> Result<()> {
    // let mut writer = stream.try_clone()?;
    // let reader = BufReader::new(&stream);

    loop {
        let mut buf = [0; 2048];

        // TODO: Improve by using a parser combinator like nom.
        // Article for inspiration: https://dpbriggs.ca/blog/Implementing-A-Copyless-Redis-Protocol-in-Rust-With-Parsing-Combinators/
        let bytes_read = stream.read(&mut buf).await?;

        if bytes_read == 0 {
            println!("Connection closed by client");
            break;
        }

        if &buf[..bytes_read] == b"*1\r\n$4\r\nPING\r\n" {
            stream.write_all("+PONG\r\n".as_bytes()).await?;
        }
    }

    Ok(())
}
