use crate::error::KafkaError;

#[derive(Debug, Clone)]
pub(crate) struct RequsetParserV2 {
    message_size: i32,
    header: RequsetHeaderParserV2,
    body: Vec<u8>,
}

impl RequsetParserV2 {
    pub fn new(request_buf: &mut Vec<u8>) -> Result<RequsetParserV2, KafkaError> {
        let mut offset = 0;
        let message_size_bytes: [u8; 4] = request_buf[offset..offset + 4].try_into().map_err(
            |err: std::array::TryFromSliceError| KafkaError::ParseRequest(err.to_string()),
        )?;
        offset +=4;
        let header: RequsetHeaderParserV2 = RequsetHeaderParserV2::new(&mut request_buf[offset..])
            .map_err(|err: KafkaError| KafkaError::ParseRequest(err.to_string()))?;

        return Ok(RequsetParserV2 {
            message_size: 1,
            header,
            body: vec![1],
        });
    }
    pub(crate) fn get_correlation_id(&self) -> i32 {
        return self.header.correlation_id
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RequsetHeaderParserV2 {
    request_api_key: i16,
    request_api_version: i16,
    pub(crate) correlation_id: i32,
    client_id: String,
    tag_buffer: Vec<u8>,
}

impl RequsetHeaderParserV2 {
    pub fn new(request_buf: &[u8]) -> Result<RequsetHeaderParserV2, KafkaError> {
        let mut offset = 0;
        let request_api_key: [u8; 2] = request_buf[offset..offset + 2].try_into().map_err(
            |err: std::array::TryFromSliceError| KafkaError::ParseRequest(err.to_string()),
        )?;

        offset += 2;
        let request_api_version: &[u8] = &request_buf[offset..offset + 2];
        offset += 2;
        let correlation_id: [u8; 4] = request_buf[offset..offset + 4].try_into().map_err(
            |err: std::array::TryFromSliceError| KafkaError::ParseRequest(err.to_string()),
        )?;
        offset += 4;
        let client_id_len = &request_buf[offset..];

        Ok(RequsetHeaderParserV2 {
            request_api_key: i16::from_be_bytes(request_api_key),
            request_api_version: 0, //todo
            correlation_id: i32::from_be_bytes(correlation_id),
            client_id: "".to_string(), //todo
            tag_buffer: vec![],        //todo
        })
    }
}
