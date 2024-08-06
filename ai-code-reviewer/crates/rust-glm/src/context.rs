use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::message::{Message, MessageType};

/// message context，also history message

const CONTEXT_FILE: &str = "context.txt";

/// message data structure for storing into CONTEXT_FILE, extend tokens for context history message calculation
#[derive(Serialize, Deserialize)]
pub(crate) struct ContextMessage {
    message: MessageType,
    // message: Box<dyn Message>,
    // total_tokens: u32,
    /// for calculating tokens when assemble request body with context messages
    tokens: u32,
}

impl ContextMessage {
    pub(crate) fn new(message_type: MessageType, tokens: u32) -> ContextMessage {
        ContextMessage {
            message: message_type,
            tokens,
        }
    }

    pub(crate) fn message(&self) -> &MessageType {
        &self.message
    }

    pub(crate) fn message_value(&self) -> Value {
        match self.message {
            MessageType::User(ref message) => message.to_value(),
            MessageType::Assistant(ref message) => message.to_value(),
            MessageType::System(ref message) => message.to_value(),
            MessageType::Tool(ref message) => message.to_value(),
        }
    }
}

pub(crate) struct Writer {
    path: &'static str,
}

impl Writer {
    pub(crate) fn new() -> Writer {
        Writer {
            path: CONTEXT_FILE,
        }
    }

    pub fn append(&self, messages: Vec<ContextMessage>) -> anyhow::Result<()> {
        let mut file = File::options()
            .append(true)
            .create(true)   // will create if not exists
            .open(self.path)?;
        for message in messages {
            let content = serde_json::to_string(&message)?;
            writeln!(file, "{}", content)?;
        }
        Ok(())
    }

    pub(crate) fn append_str(&self, message: String) -> anyhow::Result<()> {
        let mut file = File::options()
            .append(true)
            .create(true)   // will create if not exists
            .open(self.path)?;
        writeln!(file, "{}", message)?;
        Ok(())
    }
}

pub(crate) struct Loader {
    path: &'static str,
}

impl Loader {
    pub(crate) fn new() -> Loader {
        Loader {
            path: CONTEXT_FILE,
        }
    }

    /// 从上下文中加载历史消息
    pub(crate) fn load(&self, max_context_tokens: u32) -> anyhow::Result<Vec<ContextMessage>> {
        let path = Path::new(self.path);
        if !path.exists() || !path.is_file() {
            return Ok(Vec::new());
        }
        let mut lines = read_file_from_end(self.path, max_context_tokens)?;
        let mut context_messages = Vec::new();
        while let Some(line) = lines.pop() {
            let message = serde_json::from_str::<ContextMessage>(&line)?;
            context_messages.push(message);
        }
        Ok(context_messages)
    }
}

/// please read Vec by pop()
pub(crate) fn read_file_from_end(path: &str, max_context_tokens: u32) -> anyhow::Result<Vec<String>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let size = reader.seek(SeekFrom::End(0))? as i64;
    if size <= 0 {
        return Ok(Vec::new());
    }
    let (mut pos, end) = (-1, -size);   //pos = 0 is EOF
    let mut tokens_count = 0u32;
    let mut lines = Vec::new();
    let mut buffer = Vec::new();
    loop {
        // 逆序读取字符
        reader.seek(SeekFrom::End(pos))?;
        let mut ch = [0; 1];
        reader.read_exact(&mut ch)?;
        // 如果是换行符，将换行符记录到下一行
        if ch[0] == b'\n' || pos == end {
            if (pos == end) {
                buffer.push(ch[0]);
            }
            buffer.reverse();
            let line = String::from_utf8(buffer.clone()).unwrap();
            if !line.is_empty() {
                let context_message = serde_json::from_str::<ContextMessage>(&line)?;
                // 上下文消息不能超过 tokens 上限，算了先不限制了，现在智谱的接口根本无法精确计算传参中token数量
                // if tokens_count + context_message.tokens > max_context_tokens {
                //     break;
                // }
                tokens_count += context_message.tokens;
                lines.push(line);
                buffer.clear();
            }
        }

        if pos == end {
            break;
        }
        buffer.push(ch[0]);
        pos -= 1;
    }

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use crate::context::ContextMessage;
    use crate::message::{Message, MessageType, UserMessage};

    #[test]
    fn context_message_serialize() {
        let message = ContextMessage {
            message: MessageType::User(UserMessage::new("hello".to_string())),
            tokens: 1,
        };
        let json = serde_json::to_string(&message).unwrap();
        println!("{json}");
        let message = serde_json::from_str::<ContextMessage>(&json).unwrap();
        assert_eq!(message.tokens, 1);
    }
}