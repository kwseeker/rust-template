use std::cell::RefCell;
use std::io::Read;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};

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
fn usage() {
    let source_data = &b"\xFF\xFEf\x00o\x00o\x00b\x00a\x00r\x00"[..];

    let mut decode_builder = DecodeReaderBytesBuilder::new();
    decode_builder
        // 明确指定 decoder 使用的编码模式（这个其实是指定源数据的编码格式），设置这个后 BOM sniffing 会失效（默认 encoding 优先级高于 BOM嗅探）
        // 如果需要支持多中编码模式转码为 UTF-8 就去掉这个配置
        // .encoding(Some(encoding_rs::UTF_16LE))
        .utf8_passthru(true)// 对于 UTF-8 编码的数据无需额外处理，不设置这个即使原数据是 UTF-8 仍然会做转码操作 (好处是可以将乱码转成替代字符)
        // .strip_bom(true)    // 转码后是否清除 BOM 位, 默认false，仅仅当 utf8_passthru(true) 时有效，其他情况不管设置为什么都会清除 BOM 位
        .strip_bom(false)
        .bom_override(true) // 设置 BOM 探测的优先级高于 encoding() 显示设置
        .bom_sniffing(true);                           // 启用 BOM 探测
    let mut decoder = decode_builder.build(source_data);

    let mut dest = String::new();
    decoder.read_to_string(&mut dest).unwrap();
    assert_eq!(dest, "foobar");     //因为这里是 UTF-16LE 转 UTF-8 无论 strip_bom() 如何设置都不会保留 BOM 位
}

#[test]
fn usage_with_buf() {
    let source_data = &b"\xFF\xFEf\x00o\x00o\x00b\x00a\x00r\x00"[..];

    let mut decode_builder = DecodeReaderBytesBuilder::new();
    decode_builder
        // 明确指定 decoder 使用的编码模式（这个其实是指定源数据的编码格式），设置这个后 BOM sniffing 会失效（默认 encoding 优先级高于 BOM嗅探）
        // 如果需要支持多中编码模式转码为 UTF-8 就去掉这个配置
        // .encoding(Some(encoding_rs::UTF_16LE))
        .utf8_passthru(true)// 对于 UTF-8 编码的数据无需额外处理，不设置这个即使原数据是 UTF-8 仍然会做转码操作 (好处是可以将乱码转成替代字符)
        // .strip_bom(true)    // 转码后是否清除 BOM 位, 默认false，仅仅当 utf8_passthru(true) 时有效，其他情况不管设置为什么都会清除 BOM 位
        .strip_bom(false)
        .bom_override(true) // 设置 BOM 探测的优先级高于 encoding() 显示设置
        .bom_sniffing(true);                           // 启用 BOM 探测
    let buffer_rc: RefCell<Vec<u8>> = RefCell::new(vec![0; 8 * (1 << 10)]);   // 8K bytes
    let mut buffer = buffer_rc.borrow_mut();
    let mut decoder = decode_builder
        .build_with_buffer(source_data, &mut *buffer).unwrap();  //RefMut<Vec<u8>> 先解引用 => Vec<8> 再取可变引用

    let mut bytes: [u8; 32] = [0; 32];
    decoder.read(&mut bytes).unwrap();
    assert_eq!(&bytes[0..6], b"foobar");     //因为这里是 UTF-16LE 转 UTF-8 无论 strip_bom() 如何设置都不会保留 BOM 位
}

#[test]
fn test_strip_bom() {
    let source_data = &b"\xEF\xBB\xBFfoo\xFFbar"[..];
    let mut decoder = DecodeReaderBytesBuilder::new()
        .utf8_passthru(true)
        .strip_bom(true)
        .build(source_data);
    let mut dest = vec![];
    decoder.read_to_end(&mut dest).unwrap();
    assert_eq!(dest, b"foo\xFFbar");

    let mut decoder = DecodeReaderBytesBuilder::new()
        .utf8_passthru(true)
        .strip_bom(false)
        .build(source_data);
    let mut dest = vec![];
    decoder.read_to_end(&mut dest).unwrap();
    assert_eq!(dest, b"\xEF\xBB\xBFfoo\xFFbar");

    let mut decoder = DecodeReaderBytesBuilder::new()
        .utf8_passthru(false)
        .strip_bom(false)
        .build(source_data);
    let mut dest = vec![];
    decoder.read_to_end(&mut dest).unwrap();
    // println!("{:?}", String::from_utf8(dest));
    assert_eq!(dest, b"foo\xEF\xBF\xBDbar");    //这里 \xEF\xBF\xBD 即乱码的替代字符

    // 测试发现其他编码类型转成 UTF-8, 不管 strip_bom() 设置 true 还是 false 都会删除 BOM 位
    let source_data =  &b"\xFF\xFEf\x00o\x00o\x00b\x00a\x00r\x00"[..];
    let mut decoder = DecodeReaderBytesBuilder::new()
        .utf8_passthru(true)
        .strip_bom(false)
        .build(source_data);

    let mut dest = vec![];
    decoder.read_to_end(&mut dest).unwrap();
    assert_eq!(dest, b"foobar");
}

#[test]
fn test_read() {
    let source_data = &b"\xEF\xBB\xBFfoo\xFFbar"[..];
    let mut decoder = DecodeReaderBytesBuilder::new()
        .utf8_passthru(true)
        .strip_bom(false)
        .build(source_data);

    // read() 第一次会读取BOM部分，如果没有BOM就读取剩余部分
    let mut bytes: [u8; 32] = [0; 32];
    decoder.read(&mut bytes).unwrap();
    assert_eq!(&bytes[0..3], b"\xEF\xBB\xBF");

    // read() 第二次会读取剩余部分
    let mut bytes: [u8; 32] = [0; 32];
    decoder.read(&mut bytes).unwrap();
    assert_eq!(&bytes[0..7], b"foo\xFFbar");
}

/// 查看汉字的 UTF-8 编码
#[test]
fn chinese_utf8() {
    let bytes = "中国".as_bytes();
    println!("data utf8: {:?}", bytes.iter().map(|&b|
    format!("{:08b}", b).to_string()).collect::<Vec<String>>().join(" "));   //11100100 10111000 10101101 11100101 10011011 10111101
}