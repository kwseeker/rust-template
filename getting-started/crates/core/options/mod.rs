//mod.rs 是每个带有单独文件夹的模块的固定名称的文件，对标于 bin crate 的 main.rs, lib crate 的 lib.rs

//pub(crate) use 让其他模块可以通过当前options模块访问
pub(crate) use crate::options::{
    hiargs::HiArgs,     // 这一步相当于将 crate::options::hiargs::HiArgs 缩短为了 crate::options::HiArgs
    parse::{parse, ParseResult},
};

//options 模块再声明两个子模块
//mod默认是父mod私有的（即除了父mod其他mod不可以访问），
//pub(crate) mod 将模块声明为了在当前crate范围是公开的
mod parse;
pub(crate) mod hiargs;
pub(crate) mod lowargs;

