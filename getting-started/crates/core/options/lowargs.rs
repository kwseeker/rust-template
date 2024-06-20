// 可以理解为是原生态的参数，
#[derive(Debug, Default)]
pub(crate) struct LowArgs {
    raw: String,
}

impl LowArgs {
    pub(crate) fn new(val: &str) -> LowArgs {
        LowArgs { raw: String::from(val) }
    }
}