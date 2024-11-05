use std::{clone, fmt::Display};

#[derive(thiserror::Error, Debug,Clone)]
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
    // UNSUPPORTED_VERSION	35
    #[error("unsupported version")]
    UnsupportedVersion,
}

impl KafkaError {
    pub fn getErrorCode(&self) -> i32 {
        match self {
            KafkaError::Base => {
                todo!();
                return 0;
            }
            KafkaError::ParseRequest(_) => {
                todo!();
                return 0;
            }
            KafkaError::Connetion => {
                todo!();
                return 0;
            }
            KafkaError::UnsupportedVersion => 35,
        }
    }
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
