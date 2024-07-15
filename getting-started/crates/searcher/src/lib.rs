pub use crate::searcher::{
    Searcher, SearcherBuilder
};
pub use crate::sink::{
    Sink, SinkMatch, SinkError
};

mod line_buffer;
mod searcher;
mod sink;
mod lines;