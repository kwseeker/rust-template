use std::path::Path;
use anyhow::{anyhow, Context};

#[test]
fn test_anyhow_error() {
    let mut v = vec![];

    let _ = add_odd(&mut v, 1);
    let _ = add_odd2(&mut v, 3);
    println!("v: {v:?}");
    let result = add_odd(&mut v, 2);
    match result {
        Ok(_) => println!("Value added successfully."),
        Err(e) => println!("Error: {}", e),
    }
    println!("v: {v:?}");
}

//pub type Result<T, E = Error> = core::result::Result<T, E>;
fn add_odd(vec: &mut Vec<u32>, val: u32) -> anyhow::Result<()> {    //这里返回值如果有错误的话默认是用的标准库的Error类
    if val & 1 == 0 {
        let err = anyhow!("input value is not odd");    //创建的 anyhow::Error
        return Err(err);
    }
    vec.push(val);
    Ok(())
}

fn add_odd2(vec: &mut Vec<u32>, val: u32) -> anyhow::Result<(), anyhow::Error> {
    if val & 1 == 0 {
        let err = anyhow!("input value is not odd");    //创建的 anyhow::Error
        return Err(err);
    }
    vec.push(val);
    Ok(())
}

#[test]
fn test_anyhow_context() {
    let result = read_file();
    match result {
        Ok(content) => {
            println!("File content: {}", content);
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}

fn read_file() -> Result<String, anyhow::Error> {
    let current_dir = std::env::current_dir().unwrap();
    // let file_path = current_dir.join("tests/anyhow/test.txt");  //注意如果join的是绝对路径会替换
    let file_path = current_dir.join("/tests/anyhow/test.txt");  //这样会导致后面找不到文件
    println!("current_dir: {current_dir:?}, file_path: {file_path:?}");
    std::fs::read_to_string(&file_path)
        //使用context，添加更多信息
        .with_context(|| format!("Failed to read file at {}", file_path.display()))     //这里之所以能调用 with_context 方法是因为 anyhow 使用 Context<T, E> Trait 拓展了标准库 Result
                                                                                        //参考 impl<T, E> Context<T, E> for Result<T, E>
}