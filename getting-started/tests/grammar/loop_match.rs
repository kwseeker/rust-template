use std::collections::HashMap;

///
#[test]
fn loop_match() {
    // 在一个开源项目中碰到一种奇怪的写法，简化后如下：
    fn something(cmd: String) -> Result<String, String> {
        let mut methods: HashMap<&str, String> = HashMap::new();
        methods.insert("sse", "call_sse".to_string());
        methods.insert("async", "call_async".to_string());
        methods.insert("sync", "call_sync".to_string());
        // loop 没有什么意义，不管怎么样都只会执行一次
        // loop {
        //     match cmd.as_str() {
        //         "exit" => break,
        //         method => {
        //             return if let Some(call_invoke) = methods.get(method) {
        //                 Ok(call_invoke.to_string())
        //             } else {
        //                 Err("Invalid method".to_string())
        //             }
        //         }
        //     }
        // }
        // Err("Force exit".to_string())
        // 上面代码和下面代码等价
        match cmd.as_str() {
            "exit" => Err("Force exit".to_string()),
            method => {
                return if let Some(call_invoke) = methods.get(method) {
                    Ok(call_invoke.to_string())
                } else {
                    Err("Invalid method".to_string())
                }
            }
        }
    }
    let result = something("sync".to_string());
    assert_eq!(result, Ok("call_sync".to_string()));
    let result = something("exit".to_string());
    assert_eq!(result, Err("Force exit".to_string()));
    let result = something("xxx".to_string());
    assert_eq!(result, Err("Invalid method".to_string()));
}