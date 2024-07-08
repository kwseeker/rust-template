use std::io::Read;
use encoding_rs_io::DecodeReaderBytes;

#[test]
fn convert_utf16_to_utf8() {
    // foobar 的 utf-16-le 编码
    // \xFF\xFE 是字节序标记，f\x00 是 'f' 的编码，后面依次类推
    let source_data = &b"\xFF\xFEf\x00o\x00o\x00b\x00a\x00r\x00"[..];
    // N.B. `source_data` can be any arbitrary io::Read implementation.
    let mut decoder = DecodeReaderBytes::new(source_data);

    // 读取到 utf-8 编码的 String 对象
    let mut dest = String::new();
    decoder.read_to_string(&mut dest).unwrap();
    assert_eq!(dest, "foobar");

    // foobar 的 utf-8 编码
    let data_utf8 = dest.as_bytes();
    println!("data utf8: {:?}", data_utf8.iter().map(|&b|
        format!("{:02X}", b).to_string()).collect::<Vec<String>>().join(""));   //666F6F626172
}

#[test]
fn chinese_utf8() {
    let bytes = "中国".as_bytes();
    println!("data utf8: {:?}", bytes.iter().map(|&b|
    format!("{:08b}", b).to_string()).collect::<Vec<String>>().join(" "));   //11100100 10111000 10101101 11100101 10011011 10111101
}