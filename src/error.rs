use std::fmt::Display;

pub(crate) struct KafkaError {
    message: String,
}

impl KafkaError {
    pub(crate) fn new(message: String) -> Self {
        KafkaError { message }
    }
}

impl Display for KafkaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return f.write_str(&self.message);
    }
}
