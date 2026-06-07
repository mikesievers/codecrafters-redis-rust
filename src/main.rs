use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    thread,
};

use anyhow::{Error, Result};

fn main() -> Result<(), Error> {
    println!("Starting sort-of Redis.");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");

                thread::spawn(move || -> Result<(), Error> { handle_stream(stream) });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_stream(stream: std::net::TcpStream) -> Result<(), Error> {
    let mut writer = stream.try_clone()?;
    let reader = BufReader::new(&stream);
    Ok(for line in reader.lines() {
        match line {
            Ok(line) => {
                // println!("Got: {}", line);
                if line == "PING" {
                    let _ = writer.write_all("+PONG\r\n".as_bytes());
                    let _ = writer.flush();
                }
            }
            Err(_) => {
                println!("End of stream.");
            }
        }
    })
}
