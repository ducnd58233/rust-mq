use std::env;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(addr: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(addr).expect("Failed to connect");

    println!("Client connected to {}", addr);

    Ok(())
}

fn handle_server(addr: &str) -> std::io::Result<()> {
    let mut listener = TcpListener::bind(addr).expect("Failed to bind");

    println!("Server listening on {}", addr);

    for conn in listener.incoming() {
        match conn {
            Ok(_stream) => println!("Accepted a connection"),
            Err(e) => eprint!("accept error: {e}"),
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let addr = "127.0.0.1:1234";

    if args.contains(&"--server".to_string()) {
        handle_server(addr)?;
    } else if args.contains(&"--client".to_string()) {
        handle_client(addr)?;
    }

    Ok(())
}
