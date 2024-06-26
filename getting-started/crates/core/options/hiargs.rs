use anyhow::anyhow;
use crate::options::lowargs::LowArgs;
use crate::options::ParseResult;

// 面向对象实现的类型，通过结构体或枚举定义数据结构，通过impl块定义在结构体和枚举之上的方法
#[derive(Debug, Default)]    //这一句用于自动派生 Debug 这一 trait 的方法，trait 的定位有点类似其他语言的接口
pub(crate) struct HiArgs {
    raw: String,
}

impl HiArgs {
    //相当于其他语言的静态方法
    // pub(crate) fn new(val: &str) -> HiArgs {
    //     HiArgs { raw: String::from(val) }
    // }

    pub(crate) fn raw(&self) -> &String {
        &self.raw
    }

    pub(crate) fn from_low_args(low: LowArgs) -> anyhow::Result<HiArgs> {
        //TODO
        Err(anyhow!("from_low_args not achieved"))
    }
}