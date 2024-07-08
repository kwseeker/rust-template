/// 生命周期参数
/// 当定义一个结构体中包含对其他类型额引用时，需要明确指定生命周期参数（lifetime specifier）
/// 实现生命周期参数跨方法传递，参考
///      pub fn sink_with_path<'p, 's, M>(   //这种方式可以实现生命周期参数跨方法传递
///          &'s mut self,
///          matcher: M,
///          path: &'p Path,
///      ) -> StandardSink<'p, 's, M, W>
///      where
///          M: Matcher, {}

#[test]
fn test_lifetime_specifier() {
    struct A {
        a: i32,
    }

    struct B<'a> {
        // a: &A,  //会报错：Missing lifetime specifier [E0106]
        a: &'a A,
    }

    impl<'a> B<'a> {
        fn new(a: &'a A) -> B {
            B { a }
        }
    }

    let a = A {a: 2};
    let b = B::new(&a);     //这里不仅传递了a的引用，还隐式传递了a的生命周期
}
