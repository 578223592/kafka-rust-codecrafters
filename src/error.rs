use std::fmt::Display;

pub(crate) struct Error {
    message: String,
}

impl Error {
    pub(crate) fn new(message: String) -> Self {
        Error { message }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return f.write_str(&self.message);
    }
}
