//Rust 官方日志门面
// "https://github.com/rust-lang/log"

//Log 是一个 Trait
use log::Log;

#[derive(Debug)]
pub(crate) struct Logger(());   //定义一个空元祖结构体, () 是空元组（又叫单元元祖）
                                //类单元结构体的数据结构是一组字段和值，元祖结构体的数据结构则是一个元祖（元祖可以看作是增强的数组，元素数据类型可以不同）
                                //比如 `let tup: (i32, f64, u8) = (500, 6.4, 1);`

// Rust 单例模式实现, 'static 表示静态生命周期，能够存活于整个程序期间，所有字符串字面值都拥有 ‘static 生命周期
const LOGGER: &'static Logger = &Logger(());

impl Logger {
    // pub(crate) fn init() -> Result<(), log::SetLoggerError> {
    //     // log::set_logger(LOGGER)
    // }
}
