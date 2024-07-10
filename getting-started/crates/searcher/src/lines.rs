use bstr::ByteSlice;
use grep_matcher::Match;

#[derive(Debug)]
pub struct LineStep {
    line_term: u8,
    pos: usize,
    end: usize,
}

impl LineStep {

}

/// 返回 bytes 中从右往左数第 count+1 行的起始偏移
/// 比如 b"abc\nefg\nxyz"， 从右往左数第1行是“xyz”起始偏移是8，从右往左数第2行是“efg\n”起始偏移是4
/// b"abc\nxyz\n", 从右往左数第1行是“xyz\n”起始偏移是4
pub(crate) fn preceding(bytes: &[u8], line_term: u8, count: usize) -> usize {
    preceding_by_pos(bytes, bytes.len(), line_term, count)
}

/// 返回 bytes[..pos] 中从右往左数第 count+1 行的起始偏移
fn preceding_by_pos(
    bytes: &[u8],
    mut pos: usize,
    line_term: u8,
    mut count: usize,
) -> usize {
    if pos == 0 {
        return 0;
    } else if bytes[pos - 1] == line_term {
        pos -= 1;
    }
    loop {
        match bytes[..pos].rfind_byte(line_term) {  // 寻找 bytes[..pos] 中最后一个行终止符
            None => {
                return 0;
            }
            Some(i) => {
                if count == 0 {
                    return i + 1;
                } else if i == 0 {
                    return 0;
                }
                count -= 1;
                pos = i;
            }
        }
    }
}

pub(crate) fn count(bytes: &[u8], line_term: u8) -> u64 {
    memchr::memchr_iter(line_term, bytes).count() as u64
}

/// 从匹配字符串在 bytes 中的范围定位出包含这个匹配字符串的行在 bytes 中的范围
/// 即找到匹配字符串左边的行终止符在 bytes 中的位置 pos1 和右边的行终止符的位置 pos2, 返回 [pos1+1, pos2+1)
/// 但是注意可能两边都找不到行终止符的特殊处理
pub(crate) fn locate(bytes: &[u8], line_term: u8, range: Match) -> Match {
    let line_start = bytes[..range.start()].rfind_byte(line_term)
        .map_or(0, |i| i + 1);
    let line_end = bytes[range.end()..].rfind_byte(line_term)
        .map_or(bytes.len(), |i| range.end() + i + 1);  //右边找不到行终止符使用 bytes.len 作为行结束位置
    Match::new(line_start, line_end)
}

#[cfg(test)]
mod tests {
    use crate::lines;

    #[test]
    fn preceding() {
        let bytes = b"abc\nefg\nxyz";
        let term = '\n' as u8;
        let pos1 = lines::preceding(bytes, term, 0);    //xyz虽然没有行终止符但是也作为一行
        let pos2 = lines::preceding(bytes, term, 1);
        assert_eq!(pos1, 8);
        assert_eq!(pos2, 4);

        let bytes = b"abc\nxyz\n";
        let pos1 = lines::preceding(bytes, term, 0);
        assert_eq!(pos1, 4);
    }
}