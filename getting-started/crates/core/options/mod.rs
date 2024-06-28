//mod.rs 是每个带有单独文件夹的模块的固定名称的文件，对标于 bin crate 的 main.rs, lib crate 的 lib.rs

use std::fmt::Debug;
use std::panic::{RefUnwindSafe, UnwindSafe};
//pub(crate) use 让其他模块可以通过当前options模块访问
pub(crate) use crate::options::{
    doc::{
        help::{
            generate_short as generate_help_short,  //重命名 options::doc::help::generate_short 为 options::generate_help_short
        },
        version::{
            generate_long as generate_version_long,
            generate_short as generate_version_short,
        },
    },
    hiargs::HiArgs,     // 这一步相当于将 crate::options::hiargs::HiArgs 缩短为了 crate::options::HiArgs
    parse::{parse, ParseResult},
};

//options 模块再声明两个子模块
//mod默认是父mod私有的（即除了父mod其他mod不可以访问），
//pub(crate) mod 将模块声明为了在当前crate范围是公开的
mod parse;
pub(crate) mod hiargs;
pub(crate) mod lowargs;
mod defs;
mod doc;

//命令行选项特征
trait Flag: Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static {
    //获取选项完整名称
    fn name_long(&self) -> &'static str;
    //获取选项单字节简短名称，比如查看版本号的 -V, 取'V'转成u8类型
    fn name_short(&self) -> Option<u8> {
        None
    }
    //获取选项别名，可能有多个
    // fn aliases(&self) -> &'static [&'static str] {
    //     &[]
    // }
    //获取选项的否定选项名称
    fn name_negated(&self) -> Option<&'static str> {
        None
    }
    //选项分类
    fn doc_category(&self) -> Category;
    //简短说明信息
    fn doc_short(&self) -> &'static str;
    //详细说明信息
    // fn doc_long(&self) -> &'static str;
}

/// 选项分类
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Category {
    Output,
    Logging,
    OtherBehaviors,
}

impl Category {
    fn as_str(&self) -> &'static str {
        match *self {
            Category::Output => "output",
            Category::Logging => "logging",
            Category::OtherBehaviors => "other-behaviors",
        }
    }
}