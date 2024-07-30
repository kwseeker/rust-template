use std::thread;
use std::thread::Thread;
use std::time::Duration;
/// SNTP (Simple Network Time Protocol)
/// rsntp 提供了从 SNTPv4 时间服务器上同步时间的API
/// 功能：
///     Provides both a synchronous (blocking) and an (optional) asynchronous, tokio based API
///     Optional support for time and date crates chrono and time (chrono is enabled by default)
///     IPv6 support
use chrono::{DateTime, Local};
use rsntp::{AsyncSntpClient, SntpClient};

#[test]
fn test() {
    let client = SntpClient::new();
    let result = client.synchronize("ntp.aliyun.com").unwrap();
    println!("{:?}", result.datetime());
    let local_time: DateTime<Local> = DateTime::from(result.datetime().into_chrono_datetime().unwrap());
    println!("{:?}", local_time);
    let milliseconds = local_time.timestamp_millis();
    println!("{milliseconds}");
}

/// RustGLM 中调用 SntpClient 获取时间戳，但是调用几次后就会报错，怀疑是被限流了
/// 这里测试循环调用100次，看能否复现
/// 经测试短时间内最多可调用5次
#[test]
fn test_loop_call() {
    for _ in 0..100 {
        let client = SntpClient::new();
        let result = client.synchronize("ntp.aliyun.com").unwrap();
        let local_time: DateTime<Local> = DateTime::from(result.datetime().into_chrono_datetime().unwrap());
        let milliseconds = local_time.timestamp_millis();
        println!("{milliseconds}");
        thread::sleep(Duration::from_secs(1));
    }
}

#[tokio::test]
async fn test_async() {
    let client = AsyncSntpClient::new();
    let result = client.synchronize("ntp.aliyun.com").await.unwrap();

    let datetime: DateTime<Local> = DateTime::from(result.datetime().into_chrono_datetime().unwrap());
    println!("{:?}", datetime);
}