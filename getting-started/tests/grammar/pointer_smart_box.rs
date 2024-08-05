/// Box 智能指针
/// 可以在以下场景中使用它：
///     特意的将数据分配在堆上
///     数据较大时，又不想在转移所有权时进行数据拷贝
///     类型的大小在编译期无法确定，但是我们又需要固定大小的类型时
///     特征对象，用于说明对象实现了一个特征，而不是某个特定的类型
/// 特征对象（Trait Object）
///     即 （Box<dyn Trait>） 一个 trait 要能够被用作 trait object，它必须是“对象安全”的。
///     对象安全意味着该 trait 必须能够在运行时动态调度方法调用，即通过虚函数表（vtable）进行方法调用。
///
#[test]
fn usage() {
    // 在栈上创建一个长度为1000的数组
    let arr = [0; 1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;
    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0; 1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());
}

/// 将动态大小类型变为 Sized 固定大小类型
#[test]
fn dyn_size_to_fixed_size() {
    enum List {
        // Cons(i32, List),    //Rust 无法确认List的实际大小
        Cons(i32, Box<List>),   //使用 Box<> 封装，因为Box是一个指针，指针的大小是固定的，所以不会报错
        Nil,
    }
}

/// 特征对象、动态分派
#[test]
fn dyn_trait() {
    trait Draw {
        // fn new() -> Self;   // 会报错，Rust 明确要求对象安全的 Trait 的函数如果是可分派的必须是一个方法，这里定义了一个函数
                            // 要么就将其定义为一个不可分派的函数，需要具有 where Self: Sized 的约束
        fn new() -> Self where Self: Sized; //定义为不可分派的函数
        fn draw(&self);
    }

    struct Button {
        id: u32,
    }
    impl Draw for Button {
        fn new() -> Self {
            Button {
                id: 0,
            }
        }

        fn draw(&self) {
            println!("这是屏幕上第{}号按钮", self.id)
        }
    }

    struct Select {
        id: u32,
    }
    impl Draw for Select {
        fn new() -> Self {
            Select {
                id: 0,
            }
        }

        fn draw(&self) {
            println!("这个选择框贼难用{}", self.id)
        }
    }

    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];

    for e in elems {
        e.draw()
    }
}