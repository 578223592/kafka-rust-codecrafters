use std::fmt::Display;



#[derive(thiserror::Error, Debug)]
pub enum KafkaError {
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    #[error("Base error")]
    Base,
    #[error("parse request error `{0}`")]
    ParseRequest(String),
    #[error("net error , example:read write error")]
    Connetion,
}

// pub(crate) struct KafkaError {
//     message: String,
// }

// impl KafkaError {
//     pub(crate) fn new(message: String) -> Self {
//         KafkaError { message }
//     }
// }

// impl Display for KafkaError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         return f.write_str(&self.message);
//     }
// }
