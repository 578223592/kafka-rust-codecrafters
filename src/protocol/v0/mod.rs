use super::ResponseWorker;





pub(crate) struct ApiVersionsWorker {
    message_size: i32, //目前先给0 占位
    request_api_version: i16,
    header: Header, //
    body: ApiVersionBody,  // error_code [api_keys] 
}

impl ResponseWorker for ApiVersionsWorker {
    fn new(requset_parser: super::v2::RequsetParserV2) -> Self {
        return ApiVersionsWorker {
            message_size: 0,
            request_api_version: requset_parser.get_request_api_version(),
            header: Header {
                // request_api_key,
                // request_api_version => INT16
                
                correlation_id: requset_parser.get_correlation_id(),
            },
            body: ApiVersionBody{
                error_code: 0,
            },
        }
    }

    fn build_response(&mut self) -> Vec<u8> {
        //判断
        if !(self.request_api_version<=4 && self.request_api_version>=0) {
            self.body.error_code = 35;
        }

        let mut bytes: Vec<u8> = self.message_size.to_be_bytes().to_vec();
        bytes.append(&mut self.header.to_bytes());
        bytes.append(&mut self.body.to_bytes());
        return bytes;
    }
}

pub(crate) struct ApiVersionHeader {
    error_code: i16,
}

struct ApiVersionBody {
    error_code: i16,
    // api_keys: Vec<ApiVersionApiKey>, //
}
impl ApiVersionBody {
    fn to_bytes(&self) ->  Vec<u8> {
        return self.error_code.to_be_bytes().to_vec();
    }
}



pub(crate) struct CommonResponseBuilder {
    message_size: i32,
    header: Header,
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
            header: Header {
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

pub(crate) struct Header {
    // request_api_key: i16,
    // request_api_version: i16,
    correlation_id: i32,
}

impl Header {
    fn to_bytes(&self) -> Vec<u8> {
        let bytes: Vec<u8> = self.correlation_id.to_be_bytes().to_vec();
        return bytes;
    }
}

pub(crate) struct Body {}
