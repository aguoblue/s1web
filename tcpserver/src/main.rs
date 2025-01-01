use std::net::{TcpListener};
use std::io::{Read, Write};

fn main() {
    // 绑定到指定的IP地址和端口
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("服务器正在 127.0.0.1:8080 上监听连接...");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("新连接");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&buffer).unwrap();
    }

}

