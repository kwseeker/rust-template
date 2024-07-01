mod glue;
mod core;

use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Config {}

#[derive(Clone, Debug)]
pub struct SearcherBuilder {
    config: Config,
}

impl Default for SearcherBuilder {
    fn default() -> Self {
        SearcherBuilder::new()
    }
}

impl SearcherBuilder {
    pub fn new() -> SearcherBuilder {
        SearcherBuilder { config: Config::default() }
    }

    pub fn build(&self) -> Searcher {

        Searcher { config }
    }
}

pub struct Searcher {
    config: Config,
}

impl Searcher {
    pub fn new() -> Searcher {
        SearcherBuilder::new().build()
    }

    /// P 指向的数据源，经过 M 匹配器进行一行行的匹配后，将匹配的结果输出到 S,
    /// sink 有接收器的意思，响应式编程中也喜欢用这个词给输出端命名
    pub fn search_path<P, M, S>(&mut self, matcher: M, path: P, write_to: S) -> Result<(), S::Error>
    where
        P: AsRef<Path>,
        M: Matcher,
        S: Sink,
    {

    }

    /// ripgrep 还提供了一种 mmap 方式读取文件内容、还支持多行匹配（从函数名猜测），后面研究 TODO
    /// 这里暂时只展示其常规的文件读取方式和单行匹配实现
    fn search_file_maybe_path<M, S>(
        &mut self,
        matcher: M,
        path: Option<&Path>,
        file: &File,
        write_to: S,
    ) -> Result<(), S::Error>
    where
        M: Matcher,
        S: Sink,
    {
        //
        log::debug!("{:?}: searching using generic reader", path);
        self.search_reader(matcher, file, write_to)
    }

    ///使用 std:io:Read 接口读取文件内容（这里是用的 LineBufferReader）
    pub fn search_reader<M, R, S>(
        &mut self,
        matcher: M,
        read_from: R,
        write_to: S,
    ) -> Result<(), S::Error>
    where
        M: Matcher,
        R: io::Read,
        S: Sink,
    {
        self.check_config(&matcher).map_err(S::Error::error_config)?;

        let mut decode_buffer = self.decode_buffer.borrow_mut();
        let decoder = self
            .decode_builder
            .build_with_buffer(read_from, &mut *decode_buffer)
            .map_err(S::Error::error_io)?;


        let mut line_buffer = self.line_buffer.borrow_mut();
        let rdr = LineBufferReader::new(decoder, &mut *line_buffer);
        log::trace!("generic reader: searching via roll buffer strategy");
        ReadByLine::new(self, matcher, rdr, write_to).run()

    }
}
