/// outer printer
/// print AI reply message to printer

/// standard output
pub(crate) struct Standard {}

impl Standard {
    pub(crate) fn print(message: String) {
        print!("{}", message)
    }
}