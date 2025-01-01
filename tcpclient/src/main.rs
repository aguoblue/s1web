use std::net::TcpStream;

fn main() {
    let _stream = TcpStream::connect("127.0.0.1:8080").unwrap();
}
