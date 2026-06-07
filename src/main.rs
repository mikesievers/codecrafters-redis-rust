use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

use anyhow::{Error, Result};

fn main() -> Result<(), Error> {
    println!("Starting sort-of Redis.");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");

                let mut writer = stream.try_clone()?;
                let reader = BufReader::new(&stream);

                for line in reader.lines() {
                    match line {
                        Ok(_) => {
                            let _ = writer.write_all("+PONG\r\n".as_bytes());
                        }
                        Err(_) => {
                            println!("End of stream.");
                        }
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
