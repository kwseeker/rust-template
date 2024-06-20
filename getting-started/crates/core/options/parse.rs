use crate::options::{
    hiargs::HiArgs,
    lowargs::LowArgs,
};

#[derive(Debug)]
pub(crate) enum ParseResult<T> {
    // Special(SpecialMode),
    Ok(T),
    // Err(anyhow::Error),
    Err(T),
}

// 解析命令行参数到 LowArgs 然后转换成 HiArgs 类实例
pub(crate) fn parse() -> ParseResult<HiArgs> {
    let args = "some args";
    return ParseResult::Ok(HiArgs::new(args));
}

fn parse_low() -> ParseResult<LowArgs> {
    let args = "some args";
    return ParseResult::Ok(LowArgs::new(args))
}