use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub fn handle(mut stream: TcpStream)
{
    let mut buffer = [0; 1024];
    _ = stream
        .read(&mut buffer)
        .expect("Failed to read from client!");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {request}");
    let response = "Hello Client".as_bytes();
    _ = stream.write(response).expect("Failed to write response!");
}
