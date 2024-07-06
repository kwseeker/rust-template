#[test]
fn test_slice() {
    struct Match {
        start: usize,
        end: usize,
    }

    /// 数组类型也可以拓展方法
    impl std::ops::Index<Match> for [u8] {
        type Output = [u8];

        #[inline]
        fn index(&self, index: Match) -> &[u8] {
            &self[index.start..index.end]
        }
    }

    let message = "Hello, world!".as_bytes();
    let slice1 = &message[3..10];
    println!("{}", std::str::from_utf8(slice1).unwrap());

    // 使用结构体封装 range, 需要为 [u8] 实现 Index trait
    let m = Match { start: 3, end: 10 };
    let slice2 = &message[m];   //这里为何会调用 Index::index(&self, index: Match) ?  container[index] 实际上是容器的语法糖（container需要实现std::ops::Index特征）
    println!("{}", std::str::from_utf8(slice2).unwrap());
}

#[test]
fn test_slice_take_while() {
    let message = "Hello, world!".as_bytes();
    let count = message[3..10]
        .iter()
        .take_while(|&&b| -> bool { // 第一个 & 表示我们借用了迭代器中的元素
                                        // 第二个 & 是闭包参数模式的一部分，它解引用了闭包接收到的引用，因此 &&b 实际上得到的是 b 的值 u8
            match b {
                b' ' => false,  //返回false后迭代会终止
                _ => true
            }
        })
        .count();               //记录迭代次数
    assert_eq!(3, count);
}