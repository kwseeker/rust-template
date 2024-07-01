#[test]
fn test_vec_resize() {
    let mut vec = vec!["hello"];
    vec.resize(3, "world");    //修改容量为3个元素长度, 并填充值world
    assert_eq!(vec, ["hello", "world", "world"]);

    let mut vec = vec![1, 2, 3, 4];
    vec.resize(2, 0);
    assert_eq!(vec, [1, 2]);

    let mut vec2 = vec![3, 4, 5, 6, 7, 8, 9];
    vec2.copy_within(2.., 0);
    assert_eq!(vec2, [5, 6, 7, 8, 9, 8, 9]);
}