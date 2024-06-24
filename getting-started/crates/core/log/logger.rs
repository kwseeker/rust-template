//Rust 官方日志门面
// "https://github.com/rust-lang/log"

//Log 是一个 Trait
use log::{Log, Metadata, Record};
use crate::eprintln_locked;

#[derive(Debug)]
pub(crate) struct Logger(());   //定义一个空元祖结构体, () 是空元组（又叫单元元祖）
                                //类单元结构体的数据结构是一组字段和值，元祖结构体的数据结构则是一个元祖（元祖可以看作是增强的数组，元素数据类型可以不同）
                                //比如 `let tup: (i32, f64, u8) = (500, 6.4, 1);`

// Rust 单例模式实现, 'static 表示静态生命周期，能够存活于整个程序期间，所有字符串字面值都拥有 ‘static 生命周期
const LOGGER: &'static Logger = &Logger(());

impl Logger {
    pub(crate) fn init() -> Result<(), log::SetLoggerError> {
        //
        log::set_logger(LOGGER)
    }
}

impl Log for Logger {
    //实现自定义级别过滤
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    //实现自定义日志记录逻辑
    fn log(&self, record: &Record) {
        match (record.file(), record.line()) {
            (Some(file), Some(line)) => {   //file（文件名） line（行号） 都有值
                eprintln_locked!(
                    "{} | {} | {}:{}: {}",
                    record.level(),
                    record.target(),
                    file,
                    line,
                    record.args()
                );
            }
            (Some(file), None) => {             //file 有值 line 无值
                eprintln_locked!(
                    "{} | {} | {}: {}",
                    record.level(),
                    record.target(),
                    file,
                    record.args()
                );
            }
            _ => {                                      //其他情况
                eprintln_locked!(
                    "{} | {}: {}",
                    record.level(),
                    record.target(),
                    record.args()
                );
            }
        }
    }

    fn flush(&self) {
        // We use eprintln_locked! which is flushed on every call.
    }
}

#[cfg(test)]
mod tests {
    use log::{info};
    use crate::log::logger::Logger;

    #[test]
    fn log() {
        if let Err(err) = Logger::init() {
            println!("failed to initialize logger: {err}");
            return;
        }
        log::set_max_level(log::LevelFilter::Info);
        info!("info log message ...");
    }
}