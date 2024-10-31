#![allow(unused_imports)]
use std::net::TcpListener;

// 学到了：https://app.codecrafters.io/courses/kafka/stages/nv3?repo=e2a5f6bf-3345-4c95-840b-2161d08e62a2
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
