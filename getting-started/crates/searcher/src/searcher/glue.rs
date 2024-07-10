use grep_matcher::Matcher;
use crate::line_buffer::LineBufferReader;
use crate::searcher::Config;
use crate::searcher::core::Core;
use crate::{Searcher, Sink};
use crate::sink::SinkError;

#[derive(Debug)]
pub struct ReadByLine<'s, M, R, S> {
    config: &'s Config,
    ///
    core: Core<'s, M, S>,
    rdr: LineBufferReader<'s, R>,
}

impl<'s, M, R, S> ReadByLine<'s, M, R, S>
where
    M: Matcher,
    R: std::io::Read,
    S: Sink,
{
    pub(crate) fn new(
        searcher: &'s Searcher,
        matcher: M,
        read_from: LineBufferReader<'s, R>,
        write_to: S,
    ) -> ReadByLine<'s, M, R, S> {
        ReadByLine {
            config: &searcher.config,
            core: Core::new(searcher, matcher, write_to, false),
            rdr: read_from,
        }
    }

    pub(crate) fn run(mut self) -> Result<(), S::Error> {
        if self.core.begin()? {
            // 先读取一块数据到缓冲，读取成功后对缓冲中的数据进行正则匹配及输出
            while self.fill()? && self.core.match_by_line(self.rdr.buffer())? {}
            // while self.fill()? && self.core.match_by_line(self.rdr.buffer())? {}
        }

        //TODO
        // self.core.finish(
        //     self.rdr.absolute_byte_offset(),
        //     self.rdr.binary_byte_offset(),
        // )
        Ok(())
    }

    /// 内部会调用 LineBufferReader fill() 按缓冲容量读取文件内容到缓冲
    fn fill(&mut self) -> Result<bool, S::Error> {
        let old_buf_len = self.rdr.buffer().len();  //上次读取未被消费的数据的长度（一般都是不完整的行）
        // 相当于对上次读取的统计数据进行归档、重置
        let consumed = self.core.roll(self.rdr.buffer());
        self.rdr.consume(consumed); // 消费缓冲中可读取数据，只是移动指针，实际读取数据是上一步的 buffer()
        // 继续读取一块数据到缓冲（缓冲中的数据：上次未消费的数据 + 新读取的数据）
        let did_read = match self.rdr.fill() {
            Err(err) => return Err(S::Error::error_io(err)),
            Ok(did_read) => did_read,
        };

        if !did_read {
            return Ok(false)
        }
        if consumed == 0 && old_buf_len == self.rdr.buffer().len() {    //即上次消费数据长度0且这次新读取的数据长度为0,即再没有可读取的数据
            self.rdr.consume(old_buf_len);
            return Ok(false);
        }
        Ok(true)
    }
}