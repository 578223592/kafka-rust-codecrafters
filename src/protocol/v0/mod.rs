use std::vec;

use super::ResponseWorker;
use crate::responser::TAG_BUFFER;
pub(crate) struct ApiVersionsWorker {
    message_size: i32, //It specifies the size of the header and body
    request_api_version: i16,
    header: HeaderV0,
    body: ApiVersionBody, // error_code [api_keys]
}

impl ResponseWorker for ApiVersionsWorker {
    fn new(requset_parser: super::v2::RequsetParserV2) -> Self {
        return ApiVersionsWorker {
            message_size: -1,
            request_api_version: requset_parser.get_request_api_version(),
            header: HeaderV0 {
                // request_api_key,
                // request_api_version => INT16
                correlation_id: requset_parser.get_correlation_id(),
            },
            body: ApiVersionBody::new(),
        };
    }

    /**
     * response参考：https://kafka.apache.org/protocol.html#The_Messages_ApiVersions
     */
    fn build_response(&mut self) -> Vec<u8> {
        //判断
        if !(self.request_api_version <= 4 && self.request_api_version >= 0) {
            self.body.error_code = 35;
        }

        // let mut bytes: Vec<u8> = self.message_size.to_be_bytes().to_vec();
        let mut bytes: Vec<u8> = Vec::new();
        bytes.append(&mut self.header.to_bytes());
        bytes.append(&mut self.body.to_bytes());

        self.message_size = bytes.len() as i32;
        //头插
        bytes.splice(0..0, self.message_size.to_be_bytes().to_vec());
        return bytes;
    }
}

pub(crate) struct ApiVersionHeader {
    error_code: i16,
}

struct ApiVersionBody {
    error_code: i16,
    api_keys: Vec<ApiKey>,
    throttle_time_ms: i32,
}
impl ApiVersionBody {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.error_code.to_be_bytes().to_vec();
        // (self.api_keys.len()+1)转成i8

        bytes.append(&mut ((self.api_keys.len() + 1) as i8).to_be_bytes().to_vec()); // num api key records + 1 ； 不知道为啥，但是大家都是这么写的
                                                                                     // 迭代器遍历self.api_keys
        for api_key in self.api_keys.iter() {
            bytes.append(&mut api_key.to_bytes());
        }

        bytes.append(&mut TAG_BUFFER.to_be_bytes().to_vec()); //同样不知道为啥
        bytes.append(&mut self.throttle_time_ms.to_be_bytes().to_vec());
        bytes.append(&mut TAG_BUFFER.to_be_bytes().to_vec()); //同样不知道为啥

        return bytes;
    }

    fn new() -> Self {
        Self {
            error_code: 0,
            api_keys: vec![ApiKey {
                api_key: 18,
                min_version: 0,
                max_version: 4,
            }],
            throttle_time_ms: 0,
        }
    }
}

struct ApiKey {
    api_key: i16,
    min_version: i16,
    max_version: i16,
}
impl ApiKey {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = self.api_key.to_be_bytes().to_vec();
        res.append(&mut self.min_version.to_be_bytes().to_vec());
        res.append(&mut self.max_version.to_be_bytes().to_vec());
        return res;
    }
}

pub(crate) struct CommonResponseBuilder {
    message_size: i32,
    header: HeaderV0,
    body: Body,
}

impl ResponseWorker for CommonResponseBuilder {
    fn new(requset_parser: super::v2::RequsetParserV2) -> CommonResponseBuilder {
        match requset_parser.get_request_api_key() {
            0 => {}
            _ => panic!("Invalid correlation id"),
        }

        CommonResponseBuilder {
            message_size: 0,
            header: HeaderV0 {
                // request_api_key,
                // request_api_version => INT16
                correlation_id: requset_parser.get_correlation_id(),
            },
            body: Body {},
        }
    }

    fn build_response(&mut self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.message_size.to_be_bytes().to_vec();
        bytes.append(&mut self.header.to_bytes());
        // bytes.append(&mut self.body.to_bytes());
        return bytes;
    }
}

// headerV0  https://kafka.apache.org/protocol.html#protocol_messages
pub(crate) struct HeaderV0 {
    // request_api_key: i16,
    // request_api_version: i16,
    correlation_id: i32,
}

impl HeaderV0 {
    fn to_bytes(&self) -> Vec<u8> {
        let bytes: Vec<u8> = self.correlation_id.to_be_bytes().to_vec();
        return bytes;
    }
}

pub(crate) struct Body {}
