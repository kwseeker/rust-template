//命令行参数解析成的最终目标数据类型
struct Args {
    thing: String,
    number: u32,
    shout: bool,
}

/// 这里模拟一个包含两个选项一个参数的命令行的解析
/// Usage: hello [-n|--number=NUM] [--shout] THING
#[test]
fn test_usage() {
    match parse_args() {
        Ok(args) => {
            let mut message = format!("Hello {}", args.thing);
            if args.shout {
                message = message.to_uppercase();
            }
            for _ in 0..args.number {
                println!("{}", message);
            }
        }
        Err(err) => {
            println!("error: {err}");
        }
    }
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut thing = None;
    let mut number = 1;
    let mut shout = false;

    // let mut parser = lexopt::Parser::from_env();     //可以直接解析命令行参数，但是这里是单元测试无法使用这个方法创建解析器
    let raw_args =  vec!("-n=3", "--shout", "Arvin");
    // let raw_args =  vec!("-n 3", "--shout", "Arvin");    //会报错，看来并不支持这种写法
    let mut parser = lexopt::Parser::from_args(raw_args);   //from_args() 接收实现了 IntoIterator Trait 的类型
    while let Some(arg) = parser.next()? {  //? 是错误传播运算符同样可以传播空值None
        match arg {
            Short('n') | Long("number") => {        //TODO 模式匹配规则
                number = parser.value()?.parse()?;
            }
            Long("shout") => {
                shout = true;
            }
            Value(val) if thing.is_none() => {
                thing = Some(val.string()?);
            }
            Long("help") => {
                println!("Usage: hello [-n|--number=NUM] [--shout] THING");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        thing: thing.ok_or("missing argument THING")?,
        number,
        shout,
    })
}
