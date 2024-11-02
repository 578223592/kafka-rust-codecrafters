use std::io::Write;

use crate::protocol::v0::ResponseBuilder;

pub(crate) struct Responeser {
    stream: std::net::TcpStream,
}

impl Responeser {
    pub fn new(stream: std::net::TcpStream) -> Responeser {
        Responeser { stream }
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let response_builder = ResponseBuilder::new();
        let response_ = response_builder.as_bytes();
        self.send_response(response_)
    }

    pub fn send_response(&mut self, response: Vec<u8>) -> Result<(), std::io::Error> {
        self.stream.write_all(&response)?;
        self.stream.flush()?;
        Ok(())
    }
}
