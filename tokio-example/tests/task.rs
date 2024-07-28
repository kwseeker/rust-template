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
use tokio::{runtime::Runtime, time};
use futures::future::join_all;

#[test]
fn join() {
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
        rt.block_on(async {             //异步方法中调用其他异步方法，不需要再使用 spawn() 等方式提交任务，被调用的异步方法会自动当作任务提交
            tokio::join!(do_one(), do_two());   // 等待两个任务均完成，才继续向下执行代码
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

#[test]
fn join_with_return_value() {
    async fn do_handle(i: i32) -> i32 {
        time::sleep(time::Duration::from_secs(1)).await;
        println!("任务执行结果: {}, at {}", i, now());
        i
    };

    let rt = Runtime::new().unwrap();
    let mut fs = Vec::new();
    rt.block_on(async {
        println!("启动异步任务 at {}", now());
        for i in 0..10 {
            let f = do_handle(i);
            fs.push(f);
        }

        let joined_fs = join_all(fs); // 将 Vec 转换为可以调度的 Future
        let result = joined_fs.await;
        // let results = tokio::try_join!(joined_fs).unwrap(); // 等待所有任务完成
        println!("所有任务执行完毕, 结果: {:?}", result);
    });
}

#[test]
fn join_with_return_value2() {
    async fn do_handle(i: i32) -> i32 {
        time::sleep(time::Duration::from_secs(1)).await;
        println!("任务执行结果: {}, at {}", i, now());
        i
    };

    let rt = Runtime::new().unwrap();
    let mut handles = Vec::new();
    // rt.spawn(async {
    println!("启动异步任务 at {}", now());
    for i in 0..10 {
        let handle = rt.spawn(do_handle(i));
        handles.push(handle);
    }

    let results = rt.block_on(join_all(handles));
    for result in results {
        println!("任务结果: {:?}", result);
    }
}