use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    stream.write("Hello, world!".as_bytes()).unwrap();
    let mut buffer = [0; 13];
    stream.read(&mut buffer).unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buffer[..]));
    println!("Received: {:?}",std::str::from_utf8(&buffer).unwrap());
}
