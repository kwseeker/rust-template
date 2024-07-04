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
