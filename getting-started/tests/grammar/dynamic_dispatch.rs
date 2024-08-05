use std::rc::Rc;

/// Rust 动态分派机制

#[test]
fn dynamic_dispatch() {
    struct CloningLab {
        subjects: Vec<Rc<dyn LiveMammal>>,
    }

    // 2 当尝试给特征对象新增一个特性绑定时，编译器会报错
    // 当前 Rust 不支持通过多个虚函数表实现对多个特性的动态分派，推荐将多个 Trait 合并
    impl CloningLab {
        // fn clone_subjects(&self) -> Vec<Box<Mammal + Clone>> {  // error[E0225]: only auto traits can be used as additional traits in a trait object
        // fn clone_subjects(&self) -> Vec<Box<dyn CloneMammal>> {
        fn clone_subjects(&self) -> Vec<Rc<dyn LiveMammal>> {
            self.subjects.clone()
        }
    }

    trait Mammal {
        fn walk(&self);
        fn run(&self);
    }

    trait Live {
        fn eat(&self);
    }

    // 2
    // trait CloneMammal: Clone + Mammal {}    // 不符合对象安全条件： Clone requires `Self: Sized`
    trait LiveMammal: Live + Mammal {}

    impl<T> LiveMammal for T where T: Live + Mammal {}

    #[derive(Clone)]
    struct Cat {
        meow_factor: u8,
        purr_factor: u8,
    }

    impl Mammal for Cat {
        fn walk(&self) {
            println!("Cat::walk");
        }
        fn run(&self) {
            println!("Cat::run")
        }
    }

    impl Live for Cat {
        fn eat(&self) {
            println!("Cat::eat");
        }
    }

    let cat = Cat {
        meow_factor: 7,
        purr_factor: 8
    };

    // No problem, a CloneMammal is impl for Cat
    let clone_mammal: &dyn LiveMammal = &cat;

    // Error!
    // let clone: &dyn Clone = clone_mammal;   // 会编译失败，因为编译器不可能找到合适的vtable放到trait对象中，比如下面情况
    // let dog = Dog { ... };
    // let clone_mammal: &CloneMammal;
    // if get_random_bool() == true {
    //   clone_mammal = &cat;
    // } else {
    //   clone_mammal = &dog;
    // }
    // let clone: &dyn Clone = clone_mammal;
}
