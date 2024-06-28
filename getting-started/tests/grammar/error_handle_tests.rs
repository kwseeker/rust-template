use rand::random;

/// 错误处理
/// 错误传播运算符: 函数后面的“？”, 只能用在返回值为 Result 或 Option 的函数中

#[test]
fn test_propagation_result() {
    match use_might_fail() {
        Ok(_) => {}
        Err(err) => println!("error: {err}")
    }
}

fn might_fail() -> Result<i32, &'static str> {
    let val = random::<u8>();
    println!("val: {val}");
    if val & 1 == 1 {
        Ok(10)
    } else {
        Err("something went wrong")
    }
}

fn use_might_fail() -> Result<(), &'static str> {
    let result = might_fail()?;
    // 如果 might_fail 返回 Err，use_might_fail 也会返回 Err 并退出 (注意出现错误后会直接退出)
    // 如果返回 Ok，result 将包含值 10
    println!("Result was: {}", result);
    Ok(())
}

#[test]
fn test_propagation_option() {
    match use_might_return_none() {
        Some(val) => {println!("val: {val}")}
        None => println!("return None")
    }
}

fn might_return_none() -> Option<i32> {
    let val = random::<u8>();
    println!("val: {val}");
    if val & 1 == 1 {
        Some(10)
    } else {
        None
    }
}

fn use_might_return_none() -> Option<i32> {
    let result = might_return_none()?;
    println!("return Some: {result}");
    Some(result)
}