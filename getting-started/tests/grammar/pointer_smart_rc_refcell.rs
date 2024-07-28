use std::cell::RefCell;
use std::rc::Rc;

/// 用例来自 《Rust语言圣经》 https://course.rs/advance/smart-pointer/cell-refcell.html
#[test]
fn rc_with_refcell() {
    // Rc 打破 “一个数据只有一个所有者” 的规则，让数据可以拥有多个所有者
    // RefCell 打破 “数据要么有多个不可变借用，要么有一个可变借用”， 实现编译期可变、不可变引用共存
    let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

    let s1 = s.clone();
    let s2 = s.clone();
    s2.borrow_mut().push_str(", oh yeah!");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}

#[test]
fn rc_with_refcell_1() {
    fn append_in_other_lifetime(s: Rc<RefCell<String>>) {
        s.borrow_mut().push_str(", oh yeah!");      //这里的修改也无法让其他生命周期获取
        println!("{:?}", s);
    }
    fn just_print(s: Rc<RefCell<String>>) {
        println!("{:?}", s);
    }

    // Rc 打破 “一个数据只有一个所有者” 的规则，让数据可以拥有多个所有者
    // RefCell 打破 “数据要么有多个不可变借用，要么有一个可变借用”， 实现编译期可变、不可变引用共存
    let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

    append_in_other_lifetime(s.clone());
    append_in_other_lifetime(s.clone());
    just_print(s.clone());

    println!("{:?}", s);
}

/// 上面的场景如果要使用其他方式能否实现？
#[test]
fn rc_with_refcell_1_clone() {  // 克隆的方式，无法共享修改结果
    fn append_in_other_lifetime(mut s: String) {
        s.push_str(", oh yeah!");      //这里的修改也无法让其他生命周期获取
        println!("{:?}", s);
    }
    fn just_print(s: String) {
        println!("{:?}", s);
    }

    let s = String::from("a string");

    append_in_other_lifetime(s.clone());
    append_in_other_lifetime(s.clone());
    just_print(s.clone());

    println!("{:?}", s);
}

#[test]
fn rc_with_refcell_1_ref() {  // 引用的方式，可以实现但是必须声明值可变, 另外实际的项目可能涉及复杂的生命周期参数问题
    fn append_in_other_lifetime(s: &mut String) {
        s.push_str(", oh yeah!");      //这里的修改也无法让其他生命周期获取
        println!("{:?}", s);
    }
    fn just_print(s: &String) {
        println!("{:?}", s);
    }

    let mut s = String::from("a string");

    append_in_other_lifetime(&mut s);
    append_in_other_lifetime(&mut s);
    just_print(&s);

    println!("{:?}", s);
}

#[test]
fn rc_with_refcell_1_refcell() {  // RefCell的方式（RefCell实现编译期可变、不可变引用共存），本身无法在不同生命周期传递
    fn append_in_other_lifetime(s: RefCell<String>) {
        s.borrow_mut().push_str(", oh yeah!");      //这里的修改也无法让其他生命周期获取
        println!("{:?}", s);
    }
    fn just_print(s: RefCell<String>) {
        println!("{:?}", s);
    }

    let s = RefCell::new(String::from("a string"));
    append_in_other_lifetime(s);
    // 后面代码都会报错
    // append_in_other_lifetime(s);    //RefCell值本身不能所有权共享，它本身也不是引用（只是通过borrow获取借用）不能在不同的生命周期传递
    // just_print(s);

    // println!("{:?}", s);
}