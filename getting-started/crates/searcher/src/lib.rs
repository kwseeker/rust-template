pub use crate::searcher::{
    Searcher
};
pub use crate::sink::{
    Sink, SinkMatch
};

mod line_buffer;
mod searcher;
mod sink;
mod lines;