use super::router::Router;
use http::httpresponse::HttpResponse;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

// server struct
pub struct Server <'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }
    pub fn run(&self) {
        let listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Listening on {}", self.socket_addr);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established");
            
            let mut buffer = [0; 200];
            stream.read(&mut buffer).unwrap();
            let req: HttpResponse = String::from_utf8(buffer.to_vec()).unwrap().parse().unwrap();
            Router::route(req, &mut stream);
        }
    }
}