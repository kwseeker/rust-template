use log::{error, info, warn, Record, Level, Metadata, LevelFilter, trace};

// lib mod 中只需要引入依赖
#[test]
pub fn usage_in_lib() {
    info!("info log message ...");
    trace!("trace log message ...");
    warn!("warn log message ...")
}

// 二进制模块中需要配置日志记录器（Logger）
// 源码注释有完整的实例
static MY_LOGGER: MyLogger = MyLogger;
struct MyLogger;

impl log::Log for MyLogger {
    //根据日志元数据信息判断日志是否应该记录
    fn enabled(&self, metadata: &Metadata) -> bool {
        //值越小等级越高
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}

#[test]
pub fn usage_in_exec() {
    log::set_logger(&MY_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);  //日志屏蔽的最高级别，低于这个级别的日志不会打印

    trace!("trace log message ...");    //不会打印
    info!("info log message ...");
    warn!("warn log message ...");
    error!("error  log message ...");
}