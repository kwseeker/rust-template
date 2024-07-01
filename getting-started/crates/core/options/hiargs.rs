use anyhow::anyhow;
use crate::options::lowargs::{LowArgs, Mode};

/// HiArgs 高级参数，HiArgs 对象是命令行参数经过和Parser中map info匹配后
/// 实际需要使用的参数最终对象（LowArgs 只有一些选项配置的基础信息, HiArgs 则需要根据这些信息初始化真正干活的工具对象）
/// 面向对象实现的类型，通过结构体或枚举定义数据结构，通过impl块定义在结构体和枚举之上的方法
#[derive(Debug, Default)]    //这一句用于自动派生 Debug 这一 trait 的方法，trait 的定位有点类似其他语言的接口
pub(crate) struct HiArgs {
    mode: Mode,         //工作模式，这里仅支持 Search Files
}

impl HiArgs {

    //从 LowArgs 创建 HiArgs, 对一些选项做进一步处理
    pub(crate) fn from_low_args(low: LowArgs) -> anyhow::Result<HiArgs> {
        //工作模式

        //

        Err(anyhow!("from_low_args not achieved"))
    }

    pub(crate) fn searcher(&self) -> anyhow::Result<grep::searcher::Searcher> {
        
    }
}