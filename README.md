# MiniRedis

MiniRedis is a simplified Redis clone implemented in Rust. It provides a basic key-value store over a TCP connection, 
supporting commands such as **GET**, **SET**, and **DEL**.

## Server

The server component of MiniRedis listens for incoming TCP connections on <ins>**127.0.0.1:6379**</ins> and handles 
client requests. 
It stores data in memory using a HashMap.

## Building and Running the Server

To build and run the server, follow these steps:

- Ensure you have Rust and Cargo installed on your system. You can download them from the official Rust website: 
  https://www.rust-lang.org/
- Clone the repository or download the source code files.
- Open a terminal or command prompt and navigate to the project directory.
- Build the server by executing the following command:
```shell
cargo build --release
```
- Once the build is complete, run the server using the following command:
```shell
cargo run --release
```

## Available Commands

The MiniRedis server supports the following commands:

- **GET** ```key```: Retrieve the value associated with the specified key.
- **SET** ```key``` ```value```: Set the value associated with the specified key.
- **DEL** ```key```: Delete the key-value pair associated with the specified key.
