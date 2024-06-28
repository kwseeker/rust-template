/// Struct std::sync::OnceLock
/// 只能写入一次的同步原语，就是一个同步写入且只能写入一次的容器
/// https://doc.rust-lang.org/std/sync/struct.OnceLock.html

use std::sync::OnceLock;

struct DeepThought {
    answer: String,
}

impl DeepThought {
    fn new() -> Self {
        println!("exec DeepThought::new()");
        Self {
            // M3 Ultra takes about 16 million years in --release config
            answer: Self::great_question(),
        }
    }

    fn great_question() -> String {
        String::from("something")
    }
}

fn computation() -> &'static DeepThought {
    // n.b. static items do not call [`Drop`] on program termination, so if
    // [`DeepThought`] impls Drop, that will not be used for this instance.
    static COMPUTATION: OnceLock<DeepThought> = OnceLock::new();    //静态变量只会在程序启动时初始化一次，并且它们的生命周期是 'static。
                                                                    //这意味着静态变量的创建只会发生一次，即使在多线程环境中也是如此。即 computation() 方法可能执行多次，但是这行只会执行一次
    //只有第一次执行时，会新建 DeepThought 实例并写入 COMPUTATION
    COMPUTATION.get_or_init(|| DeepThought::new())
}

#[test]
fn test_once_lock() {
    let _ = computation().answer;
    let _ = computation().answer;
}