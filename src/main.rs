#![allow(unused_imports)]
mod error;
mod responser;
mod protocol;

use std::net::TcpListener;

use responser::Responeser;

// 学到了：https://app.codecrafters.io/courses/kafka/stages/nv3?repo=e2a5f6bf-3345-4c95-840b-2161d08e62a2
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(new_stream) => {
                // message_size
                // Header
                /*
                Request Header v0 => request_api_key request_api_version correlation_id
                request_api_key => INT16
                request_api_version => INT16
                correlation_id => INT32
                                   */
                // Body
                let mut responeser = Responeser::new(new_stream);
                let _ = responeser.run();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
