use crate::protocol::v2::RequsetParserV2;
use std::{
    io::{Read, Write},
    vec,
};

use crate::{
    error::KafkaError,
    protocol::{v0::ResponseBuilder, v2::RequsetHeaderParserV2},
};

pub(crate) struct Responeser {
    stream: std::net::TcpStream,
}

impl Responeser {
    pub fn new(stream: std::net::TcpStream) -> Responeser {
        Responeser { stream }
    }

    pub fn run(&mut self) -> Result<(), KafkaError> {
        // 从self.stream中读取数据
        let mut buf = [0; 1024];
        if let Err(_) = self.stream.read(&mut buf) {
            //由于没有设置read timeout，所以这里会阻塞，直到有数据可读
            return Err(KafkaError::Connetion);
        }
        let requset_parser_v2 = RequsetParserV2::new(&mut Vec::from(buf))?;
        dbg!(requset_parser_v2.clone());
        let response_builder = ResponseBuilder::new(requset_parser_v2.get_correlation_id());
        let response_ = response_builder.build_response();
        if let Err(_) = self.send_response(response_) {
            //考虑包装这个ioError
            return Err(KafkaError::Connetion);
        }
        Ok(())
    }

    fn send_response(&mut self, response: Vec<u8>) -> Result<(), std::io::Error> {
        self.stream.write_all(&response)?;
        self.stream.flush()?;
        Ok(())
    }
}
