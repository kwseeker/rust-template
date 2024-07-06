use std::io;
use std::io::Write;
use termcolor::{ColorSpec, WriteColor};

/// 会记录成功输出字节数的 Writer (实际使用时是 WriteColor 类型, WriteColor: io::Write)
#[derive(Clone, Debug)]
pub(crate) struct CounterWriter<W> {
    /// 被封装的 std::io::Write (其实是 WriteColor类型)， 比如 StandardStream
    wtr: W,
    /// 自从上次 reset() 到现在输出的字节数
    count: u64,
    /// 自从构造后总共输出的字节树
    total_count: u64,
}

impl<W: Write> CounterWriter<W> {
    pub(crate) fn new(wtr: W) -> CounterWriter<W> {
        CounterWriter { wtr, count: 0, total_count: 0 }
    }
}

impl<W> CounterWriter<W> {
    #[inline]
    pub(crate) fn count(&self) -> u64 {
        self.count
    }

    #[inline]
    pub(crate) fn total_count(&self) -> u64 {
        self.total_count + self.count
    }

    #[inline]
    pub(crate) fn reset_count(&mut self) {
        self.total_count += self.count;
        self.count = 0;
    }
}

impl<W: Write> Write for CounterWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.wtr.write(buf)?;
        self.count += n as u64;
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.wtr.flush()
    }
}

/// 为 CounterWriter 拓展 termcolor 颜色输出行为
impl<W: WriteColor> WriteColor for CounterWriter<W> {
    #[inline]
    fn supports_color(&self) -> bool {
        self.wtr.supports_color()
    }

    #[inline]
    fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        self.wtr.set_color(spec)
    }

    /// 重置为原始的设置
    #[inline]
    fn reset(&mut self) -> io::Result<()> {
        self.wtr.reset()
    }
}