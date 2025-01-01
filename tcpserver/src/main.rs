use std::net::{TcpListener};

fn main() {
    // 绑定到指定的IP地址和端口
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("服务器正在 127.0.0.1:8080 上监听连接...");

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("新连接");
    }

}

