/// Tokio task
/// task::spawn() 提交一个异步任务
/// task::spawn_blocking() 生成一个blocking thread执行同步任务
/// task::block_in_place() 在当前worker thread中执行指定的可能会长时间运行或长时间阻塞线程的任务，但是它会先将该worker thread中已经存在的异步任务转移到其它worker thread，使得这些异步任务不会被饥饿。
/// task::yield_now 让当前任务立即放弃CPU，将worker thread交还给调度器
/// task::spawn_local()
/// task.abort() 取消任务
/// tokio::join! 必须等待所有任务完成
/// tokio::try_join! 要么等待所有异步任务正常完成，要么等待第一个返回Result Err的任务出现
/// task::LocalSet 让某些任务固定在某一个线程中运行，比如没有实现 Send 的异步任务，不能跨线程
/// tokio::select! 轮询指定的多个异步任务，每个异步任务都是select!的一个分支，当某个分支已完成，则执行该分支对应的代码，同时取消其它分支。
/// JoinHandle::is_finished() 判断任务是否已终止，它是非阻塞的
/// tokio::task::JoinSet 用于收集一系列异步任务，并判断它们是否终止。
use std::time::SystemTime;

#[test]
fn join() {
    use tokio::{self, runtime::Runtime, time};

    async fn do_one() {
        println!("doing one: {}", now());
        time::sleep(time::Duration::from_secs(2)).await;
        println!("do one done: {}", now());
    }

    async fn do_two() {
        println!("doing two: {}", now());
        time::sleep(time::Duration::from_secs(1)).await;
        println!("do two done: {}", now());
    }

    fn main() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            tokio::join!(do_one(), do_two());// 等待两个任务均完成，才继续向下执行代码
            println!("all done: {}", now());
        });
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}