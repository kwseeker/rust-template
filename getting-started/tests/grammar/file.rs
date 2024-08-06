/// 文件操作
use std::fs::File;
use std::io::{BufReader, Error, Read, Seek, SeekFrom};
use std::path::Path;

/// 从文件末尾开始按行读取文件，每一行数据读取到
#[test]
fn read_from_end() {
    fn read_lines_from_end<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, Error> {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);

        let size = reader.seek(SeekFrom::End(0))? as i64;
        let (mut pos, end) = (-1, -size);   //pos = 0处是EOF
        let mut lines = Vec::new();
        let mut buffer = Vec::new();
        loop {
            // 逆序读取字符
            reader.seek(SeekFrom::End(pos))?;
            let mut ch = [0; 1];
            reader.read_exact(&mut ch)?;
            // 如果是换行符，将换行符记录到下一行
            if ch[0] == b'\n' {
                buffer.reverse();
                lines.push(String::from_utf8(buffer.clone()).unwrap());
                buffer.clear();
            }
            buffer.push(ch[0]);
            // 如果读取到了文件开头，则退出循环
            if pos == end {
                buffer.reverse();
                lines.push(String::from_utf8(buffer.clone()).unwrap());
                buffer.clear();
                break;
            }
            pos -= 1;
        }

        Ok(lines)
    }

    let mut lines = read_lines_from_end("build.rs").unwrap();
    // Vec 本身已经实现了栈的操作方法
    while let Some(item) = lines.pop() {
        print!("{}", item);
    }
}