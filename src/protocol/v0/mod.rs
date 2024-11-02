pub(crate) struct ResponseBuilder {
    message_size: i32,
    header: Header,
    body: Body,
}

impl ResponseBuilder {
    pub fn new() -> ResponseBuilder {
        ResponseBuilder {
            message_size: 0,
            header: Header {
                // request_api_key,
                // request_api_version => INT16
                correlation_id: 7,
            },
            body: Body {},
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
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
