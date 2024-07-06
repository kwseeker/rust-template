/// 智能指针
/// 智能指针（smart pointers）是一类数据结构，它们的表现类似指针，但是也拥有额外的元数据和功能。
///
/// 普通引用和智能指针的一个额外的区别是引用是一类只借用数据的指针；相反，在大部分情况下，智能指针“拥有”它们指向的数据。
/// 智能指针通常使用结构体实现。智能指针不同于结构体的地方在于其实现了 Deref 和 Drop trait。
///
/// 内部可变性（interior mutability）模式，这是不可变类型暴露出改变其内部值的 API。
///
/// 常用智能指针：
///     Box<T> ，用于在堆上分配值。box 允许你将一个值放在堆上而不是栈上，留在栈上的则是指向堆数据的指针。
///         应用场景：
///         1）当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
///             案例：使用 Box 创建递归类型
///         2）当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
///         3）当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
///     Rc<T> ，一个引用计数类型，其数据可以有多个所有者。
///     Ref<T> 和 RefMut<T> ，通过 RefCell<T> 访问。（ RefCell<T> 是一个在运行时而不是在编译时执行借用规则的类型）。
///         RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针。每次调用 borrow ，
///         RefCell<T> 将活动的不可变借用计数加一。当 Ref<T> 值离开作用域时，不可变借用计数减一。
///         就像编译时借用规则一样，RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用。

use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();     // borrow_mut() 获取 RefCell 中值的可变引用
        // let mut two_borrow = self.sent_messages.borrow_mut();                    // RefCell不允许同时创建多个可变引用
        one_borrow.push(String::from(message));
        // two_borrow.push(String::from(message));
    }
}

#[test]
fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    limit_tracker.set_value(80);
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
}