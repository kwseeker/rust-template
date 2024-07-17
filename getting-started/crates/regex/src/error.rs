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

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use bstr::ByteSlice;

        match self.kind {
            ErrorKind::Regex(ref s) => write!(f, "{}", s),
            ErrorKind::NotAllowed(ref lit) => {
                write!(f, "the literal {:?} is not allowed in a regex", lit)
            }
            ErrorKind::InvalidLineTerminator(byte) => {
                write!(
                    f,
                    "line terminators must be ASCII, but {byte:?} is not",
                    byte = [byte].as_bstr(),
                )
            }
            ErrorKind::Banned(byte) => {
                write!(
                    f,
                    "pattern contains {byte:?} but it is impossible to match",
                    byte = [byte].as_bstr(),
                )
            }
        }
    }
}