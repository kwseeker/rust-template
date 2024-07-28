use std::ffi::OsString;
use lexopt::{Error, ValueExt};

#[test]
fn test_convert_vec_to_array() {
    let mut v = Vec::new();
    v.push(String::from("Hello"));
    v.push(String::from("World"));

    fn func1(words: &[String]) {    //将向量引用转换为切片引用，因为 Vec 实现了 Deref trait，它可以被解引用为一个切片。
        for x in words {
            println!("{x}");
        }
    }

    func1(&v)
}

#[test]
fn convert_osstring_to_usize() {
    let os_str = OsString::from("1");
    let result = os_str.parse::<usize>();
    match result {
        Ok(value) => assert_eq!(value, 1usize),
        Err(_) => {panic!("parse failed")}
    }

    // let os_str = OsString::from("1xx");
    // let result = os_str.parse::<usize>();
    // match result {
    //     Ok(value) => assert_eq!(value, 1usize),
    //     Err(_) => {panic!("parse failed")}
    // }
}