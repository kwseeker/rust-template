use crate::options::hiargs::HiArgs;

#[derive(Debug)]
pub(crate) enum ParseResult<T> {
    // Special(SpecialMode),
    Ok(T),
    // Err(anyhow::Error),
    Err(T),
}

// 解析命令行参数并转换成 HiArgs 类实例
pub(crate) fn parse() -> ParseResult<HiArgs> {
    let args = "some args";
    return ParseResult::Ok(HiArgs::new(args));
}