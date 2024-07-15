use std::io;
use bstr::ByteSlice;

/// ripgrep 搜索流程的3个重要的类型之一 LineBufferReader
/// LineBufferReader 用于从指定的 path file 中读取数据到缓冲 (LineBuffer)
/// ripgrep 还支持二进制内容读取，还支持两种缓冲池容量扩容策略，不过这里先略

pub(crate) const DEFAULT_BUFFER_CAPACITY: usize = 64 * (1 << 10); // 64 KB

#[derive(Clone, Copy, Debug)]
struct Config {
    ///
    capacity: usize,
    /// 行终止符
    line_terminator: u8,
    // The behavior for handling long lines.
    // buffer_alloc: BufferAllocation,
    // When set, the presence of the given byte indicates binary content.
    // binary: BinaryDetection,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            capacity: DEFAULT_BUFFER_CAPACITY,
            line_terminator: b'\n',
        }
    }
}

/// 用于创建行缓冲
#[derive(Clone, Debug, Default)]
pub(crate) struct LineBufferBuilder {
    config: Config,
}

impl LineBufferBuilder {
    pub(crate) fn new() -> LineBufferBuilder {
        LineBufferBuilder {
            config: Config::default()
        }
    }

    pub(crate) fn capacity(&mut self, capacity: usize, ) -> &mut LineBufferBuilder {
        self.config.capacity = capacity;
        self
    }

    pub(crate) fn line_terminator(&mut self, line_terminator: u8) -> &mut LineBufferBuilder {
        self.config.line_terminator = line_terminator;
        self
    }

    pub(crate) fn build(&self) -> LineBuffer {
        LineBuffer {
            config: self.config,
            buf: vec![0; self.config.capacity],
            pos: 0,
            last_line_terminator: 0,
            end: 0,
            absolute_byte_offset: 0,
            // binary_byte_offset: None,
        }
    }
}

/// 读规则：
/// 1）读取时按照缓冲剩余空间读取（开始时buf实际分配容量默认是64KB），如果没有读到行终止符就循环先扩容再继续读取（可能读取到多行）；
/// 2）每次消费到最后一个行终止符，如果行终止符后面还有数据，不会消费会等待下次读取到完整行再消费， 参考 LineBuffer buffer() 方法
#[derive(Debug)]
pub(crate) struct LineBufferReader<'b, R> { //TODO 注意这里的作用域
    //std::io::Read, 相当于一个输入流
    rdr: R,
    //行缓冲
    line_buffer: &'b mut LineBuffer,
}

impl<'b, R: io::Read> LineBufferReader<'b, R> {
    pub(crate) fn new(rdr: R, line_buffer: &'b mut LineBuffer) -> LineBufferReader<'b, R> {
        line_buffer.clear();
        LineBufferReader { rdr, line_buffer }
    }

    /// 读取 std::io::Read 数据到缓冲
    pub(crate) fn fill(&mut self) -> Result<bool, io::Error> {
        self.line_buffer.fill(&mut self.rdr)
    }

    /// 查看缓冲可读取内容（即从pos到最后一个终止符，不会修改指针值）
    pub(crate) fn buffer(&self) -> &[u8] {
        self.line_buffer.buffer()
    }

    /// 消费缓冲可读取内容，只会修改指针值。amt: 消费字节数量
    /// 由于只会修改指针值，所以估计是配合 buffer() 一起使用的
    pub(crate) fn consume(&mut self, amt: usize) {
        self.line_buffer.consume(amt);
    }
}

#[derive(Clone, Debug)]
pub(crate) struct LineBuffer {
    config: Config,
    /// 缓冲池
    buf: Vec<u8>,
    /// 缓冲数据可消费位置（指向最后消费到的位置）
    pos: usize,
    /// 最后的行终止符在buf中的位置(其实表示的是第几个字符，last_line_terminator-1是终止符在buf中的索引)
    /// 参考：self.last_line_terminator = old_end + i + 1;
    last_line_terminator: usize,
    /// 数据结束的位置（同样表示的是第几个字符，end-1是最后一个字符在buf中的索引），
    /// 因为从 ripgrep 流程看它是默认每次读取64KB数据，最后读取到的字符可能是行终止符也可能不是，
    /// 所以需要额外记录读取的数据的最后的位置
    end: usize,
    /// 自构造或执行clear()依赖绝对偏移量
    absolute_byte_offset: u64,
    // binary_byte_offset: Option<u64>,
}

impl LineBuffer {
    fn clear(&mut self) {
        self.pos = 0;
        self.last_line_terminator = 0;
        self.end = 0;
        self.absolute_byte_offset = 0;
        // self.binary_byte_offset = None;
    }

    /// 将数据从 std::io::Read 读取到 LineBuffer
    fn fill<R: io::Read>(&mut self, mut rdr: R) -> Result<bool, io::Error> {
        //将上次未消费的数据放到缓冲最前面，新读取的数据追加到后面
        self.roll();
        //这里退出循环的条件是要么没有数据可读，要么读取的新数据至少包含一个行终止符号
        loop {
            //确保行缓冲剩余空间足够容纳新读取的数据，不够就额外分配一些空间，Vec不是变长的么？会自动拓展为何还需要手动分配？
            //主要是因为下一行 io::Read 读操作只会按Vec实际分配的空间读
            self.ensure_capacity()?;
            //将文件内容读取到缓冲空闲的切片，返回实际读取的字节数量
            //注意 DecodeReaderBytes read() 方法第一次读取会尝试读取文件的 BOM 部分（3字节）
            let read_len = rdr.read(self.free_buffer().as_bytes_mut())?;    //数据读完或buffer读满为止，返回实际读取的长度（按元素个数算）
            if read_len == 0 {  //说明文件读取完毕
                self.last_line_terminator = self.end;   //将行终止符位置设置为最后一个字符的位置，这样即使没有行终止符也可以被消费了
                return Ok(!self.buffer().is_empty());
            }
            //更新行缓冲中的指针
            let old_end = self.end;
            self.end += read_len;
            let newbytes = &mut self.buf[old_end..self.end];
            if let Some(i) = newbytes.rfind_byte(self.config.line_terminator) {    //寻找最后一个行终止符在 newbytes 中的索引
                self.last_line_terminator = old_end + i + 1;
                return Ok(true);
            }
        }
    }

    /// 将行缓冲中还未消费的数据滚动到前面，因为每次消费只会消费到最后一个行终止符，后面还有数据会等下次消费
    /// 下次读取前需要先将这部分未消费的数据放到行缓冲最前面
    fn roll(&mut self) {
        if self.pos == self.end {   //说明之前读取的数据全部消费完毕了，刚开始或者正好读到行终止符号会满足这个条件
            self.pos = 0;
            self.last_line_terminator = 0;
            self.end = 0;
            return;
        }
        //走到这说明上次还遗留了一点数据没消费（因为不是完整的行），将这部分数据移动到行缓冲头部
        let roll_len = self.end - self.pos;
        self.buf.copy_within(self.pos..self.end, 0);    //将未消费的数据写到行缓冲0开始的位置
        self.pos = 0;
        self.last_line_terminator = roll_len;   //如果后面新读取的数据也不包含行终止符怎么办？查看 fill() 方法可以确保新读取的数据至少包含一个终止符
        self.end = roll_len;
    }

    /// ripgrep 支持两种扩容机制（Eager、Error），这里只展示 Eager(每次扩容增加之前的两倍容量，相当于扩容为3倍)
    fn ensure_capacity(&mut self) -> Result<(), io::Error> {
        if !self.free_buffer().is_empty() { //有空余空间即可，不够的话下个循环可以继续扩容
            return Ok(());
        }
        let len = std::cmp::max(1, self.buf.len());
        let new_len = self.buf.len() + len * 2;
        self.buf.resize(new_len, 0);
        Ok(())
    }

    fn free_buffer(&mut self) -> &mut [u8] {
        &mut self.buf[self.end..]
    }

    /// 读取缓冲内容（即读取从 pos 到 last_line_terminator 的数据）
    fn buffer(&self) -> &[u8] {
        &self.buf[self.pos..self.last_line_terminator]
    }

    fn consume(&mut self, amt: usize) {
        assert!(amt <= self.buffer().len());
        self.pos += amt;
        self.absolute_byte_offset += amt as u64;
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;
    use crate::line_buffer::{LineBuffer, LineBufferBuilder, LineBufferReader};

    #[test]
    fn bstr_rfind_byte() {
        let line = "Hello, World!";
        let bytes = line.as_bytes();
        let idx = bytes.rfind_byte(b',').unwrap();
        println!("{idx}");
        assert_eq!(5, idx);
    }

    /// 测试将内容读取到行缓冲，以及读取缓冲内容消费缓冲
    #[test]
    fn buffer_basic() {
        let lines = "homer\nlisa\nmaggie";
        let mut line_buffer = LineBufferBuilder::new().build();         //创建一个行缓冲
        let mut rdr = LineBufferReader::new(lines.as_bytes(), &mut line_buffer);    //TODO 这里自动将数组转成了 io:Read 特征类型，内部原理？
        assert!(rdr.buffer().is_empty());
        //1 执行io::Read读操作将数据写入行缓冲
        rdr.fill().unwrap();
        //2 读取缓冲内容（从pos到最后一个行终止符）
        let mut bstr = rdr.buffer().as_bstr();     //as_bstr() 是 bstr::ByteSlice 为 [u8] 拓展的方法
        assert_eq!("homer\nlisa\n", bstr);
        //3 消费缓冲数据
        println!("bstr len: {}", bstr.len());
        rdr.consume(bstr.len());

        //继续消费后面的"maggie"
        rdr.fill().unwrap();
        bstr = rdr.buffer().as_bstr();
        rdr.consume(bstr.len());
    }

    /// 测试行缓冲扩容
    #[test]
    fn buffer_resize() {
        let lines = "homer\nlisa_this_message_will_cause_buffer_resize\nmaggie";
        let mut line_buffer = LineBufferBuilder::new().capacity(8).build();         //创建一个行缓冲
        let mut rdr = LineBufferReader::new(lines.as_bytes(), &mut line_buffer);
        //1 执行io::Read读操作将数据写入行缓冲
        //  这次会读取8个字节，即”homer\nli“
        rdr.fill().unwrap();
        //2 读取缓冲内容（从pos到最后一个行终止符）
        let bstr = rdr.buffer().as_bstr();
        assert_eq!("homer\n", bstr);
        //3 消费缓冲数据
        println!("bstr len: {}", bstr.len());
        rdr.consume(bstr.len());

        //行缓冲中还有”li“未消费，继续读取，由于剩余空间6不足以读取到下一个行终止符（"sa_this_message_will_cause_buffer_resize\n"），会扩容且是多次扩容，直到扩容到 72（8*3*3）
        rdr.fill().unwrap();
        let bstr = rdr.buffer().as_bstr();
        assert_eq!("lisa_this_message_will_cause_buffer_resize\n", bstr);
        rdr.consume(bstr.len());
    }
}

