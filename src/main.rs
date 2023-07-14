use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: TcpStream, db: &mut HashMap<String, String>) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    let mut buffer = String::new();

    while let Ok(bytes_read) = reader.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let command_parts: Vec<&str> = buffer.trim().splitn(2, ' ').collect();
        let command = command_parts[0];
        let key = command_parts.get(1).map(|&k| k);

        match command {
            "GET" => {
                if let Some(value) = db.get(key.unwrap_or("")) {
                    writeln!(writer, "{}", value).unwrap();
                } else {
                    writeln!(writer, "(nil)").unwrap();
                }
            }
            "SET" => {
                if let Some(mut parts) = key.map(|k| k.splitn(2, ' ')) {
                    if let Some(key) = parts.next() {
                        if let Some(value) = parts.next() {
                            db.insert(key.to_owned(), value.to_owned());
                            writeln!(writer, "OK").unwrap();
                        }
                    }
                }
            }
            "DEL" => {
                if let Some(key) = key {
                    if db.remove(key).is_some() {
                        writeln!(writer, "OK").unwrap();
                    } else {
                        writeln!(writer, "(nil)").unwrap();
                    }
                }
            }
            _ => {
                writeln!(writer, "Invalid command").unwrap();
            }
        }

        writer.flush().unwrap();
        buffer.clear();
    }
}

const ADDR_DEFAULT: &str = "127.0.0.1:6379";

fn main() {
    let listener = TcpListener::bind(ADDR_DEFAULT).expect("Failed to bind address");
    let db = HashMap::new();

    println!("MiniRedis server listening on 127.0.0.1:6379");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut db = db.clone();
                thread::spawn(move || {
                    handle_client(stream, &mut db);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept client connection: {}", e);
            }
        }
    }
}
