#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }

    pub(crate) fn regex(err: regex_automata::meta::BuildError) -> Error {
        if let Some(size_limit) = err.size_limit() {
            let kind = ErrorKind::Regex(format!(
                "compiled regex exceeds size limit of {size_limit}",
            ));
            Error { kind }
        } else if let Some(ref err) = err.syntax_error() {
            Error::generic(err)
        } else {
            Error::generic(err)
        }
    }

    pub(crate) fn generic<E: std::error::Error>(err: E) -> Error {
        Error { kind: ErrorKind::Regex(err.to_string()) }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    Regex(String),
    NotAllowed(String),
    InvalidLineTerminator(u8),
    Banned(u8),
}