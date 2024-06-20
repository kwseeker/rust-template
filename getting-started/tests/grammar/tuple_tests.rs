//Rust 元组测试

#[test]
fn test_tuple() {
    // let tup: (i32, i8, f32) = (100, 2, 3.5);
    let tup = (100, 2, 3.5);
    //解构
    let tup2: (i32, i8, f32) = tup;
    println!("Elements in tup2: {}, {}, {}", tup2.0, tup2.1, tup2.2)
}