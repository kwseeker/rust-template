use std::fs::File;
use std::io::Write;

/// 消息上下文， 也即历史消息

const CONTEXT_FILE: &str = "context.json";

pub(crate) struct Writer {
    path: &'static str,
}

impl Writer {
    pub(crate) fn new() -> Writer {
        Writer {
            path: CONTEXT_FILE,
        }
    }

    pub(crate) fn append(&self, message: String) -> anyhow::Result<()> {
        let mut file = File::options()
            .append(true)
            .create(true)   // will create if not exists
            .open(self.path)?;
        writeln!(file, "{}", message)?;
        Ok(())
    }
}