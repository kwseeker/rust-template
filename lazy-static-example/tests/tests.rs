use std::collections::HashMap;
use lazy_static::lazy_static;

#[test]
fn usage() {
    // 错误写法
    // static HASHMAP: HashMap<u32, &'static str> = {
    //     let mut m = HashMap::new();
    //     m.insert(0, "foo");
    //     m.insert(1, "bar");
    //     m.insert(2, "baz");
    //     m
    // };
    // static COUNT: usize = HASHMAP.len();
    // static NUMBER: u32 = times_two(21);
    lazy_static! {
        static ref HASHMAP: HashMap<u32, &'static str> = {
            let mut m = HashMap::new();
            m.insert(0, "foo");
            m.insert(1, "bar");
            m.insert(2, "baz");
            m
        };
        static ref COUNT: usize = HASHMAP.len();
        static ref NUMBER: u32 = times_two(21);
    }

    fn times_two(n: u32) -> u32 { n * 2 }

    println!("The map has {} entries.", *COUNT);
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    println!("A expensive calculation on a static results in: {}.", *NUMBER);
}