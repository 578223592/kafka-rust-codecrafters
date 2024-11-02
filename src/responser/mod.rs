use std::io::Write;

use crate::{error::KafkaError, protocol::v0::ResponseBuilder};

pub(crate) struct Responeser {
    stream: std::net::TcpStream,
}

impl Responeser {
    pub fn new(stream: std::net::TcpStream) -> Responeser {
        Responeser { stream }
    }

    pub fn run(&mut self) -> Result<(), KafkaError> {
        let response_builder = ResponseBuilder::new();
        let response_ = response_builder.as_bytes();
        if let Err(_) = self.send_response(response_) {
            //考虑包装这个ioError
            return Err(KafkaError::new("Failed to send response".to_string()));
        }
        Ok(())
    }

    fn send_response(&mut self, response: Vec<u8>) -> Result<(), std::io::Error> {
        self.stream.write_all(&response)?;
        self.stream.flush()?;
        Ok(())
    }
}
