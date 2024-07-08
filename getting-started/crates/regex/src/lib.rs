/// pub use 是向外部暴露类型， use 则是使用外部的类型
pub use crate::{
    matcher::{RegexMatcher}
};

mod matcher;
mod config;
mod error;
mod ast;
mod ban;
mod strip;
mod literals;
mod non_matching;