use crate::protocol::{response_worker_factory, v2::RequsetParserV2, ResponseWorker};
use std::{
    io::{Read, Write},
    vec,
};

use crate::{
    error::KafkaError,
    protocol::{v0::CommonResponseBuilder, v2::RequsetHeaderParserV2},
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
        let mut requset_parser_v2: RequsetParserV2 = RequsetParserV2::new(&mut Vec::from(buf))?;
        dbg!(requset_parser_v2.clone());

        // requset_parser_v2.validate(); //valide能力应该放到ResponseWorker里面


        let mut response_builder = response_worker_factory(requset_parser_v2);
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
