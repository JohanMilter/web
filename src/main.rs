use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread};
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read_exact(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main(){
    // Hardcoded URL
    let url = "http://localhost:4444";
    
    // Extract host and port from the URL
    let host = "127.0.0.1"; // or "localhost"
    let port = 4444;
    let address = format!("{}:{}", host, port);
    
    // Start the TCP listener
    let listener = TcpListener::bind(&address).unwrap();
    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}