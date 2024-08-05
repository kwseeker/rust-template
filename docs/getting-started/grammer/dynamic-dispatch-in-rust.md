# Rust 动态分派模型：虚函数表 与 对象安全

使用特征对象（Trait Object， 如 `Box<dyn Trait>`）时，可能会遇到相关的概念和问题。

动态分派是指在程序运行期间，根据对象的实际类型来决定调用哪个方法的过程。这意味着，即使两个不同的对象看起来具有相同的接口（例如，它们都实现了同一个接口或继承自同一个基类），实际调用的方法取决于对象的实际类型。

> 与动态分派对应的是静态分派（static dispatch），指编译器在编译时就确定应该调用哪个方法实现。

像C++一样，Rust的动态分派也是通过一个虚函数表（`VTable`）实现的([Rust文档](https://doc.rust-lang.org/book/trait-objects.html#representation)中有说明)。

虚拟函数表是一种数据结构，它通常是一个数组，其中包含了指向类的虚函数的指针。当一个对象调用虚函数时，程序会查找该对象的虚拟函数表，并通过表中的相应条目来调用正确的函数。这种方法允许在运行时确定要调用哪个函数，而不是在编译时确定。

> 虚函数（Virtual Function）是面向对象编程中的一种特性，这种函数或方法可以被子类继承和覆盖，通常使用动态分派实现，从而实现多态性。
>
> 有了虚函数，程序甚至能够调用编译期还不存在的函数。

参考文档 [深入理解 Rust 的动态分派模型](https://www.oschina.net/translate/exploring-dynamic-dispatch-in-rust?print) 中举的例子,

一个特征对象实例的内存布局实际包含**两个指针**一个指向**实际类型的数据成员**，一个指向**针对Trait的虚函数表**；

![](https://static.oschina.net/uploads/space/2018/0509/150307_3toz_3820517.png)

C++ 实现多继承是通过包含多个虚函数表，每个虚函数表代表继承的一种类型。

Rust 实现多重特性（Trait）绑定则是通过合并虚函数表实现，合并虚函数表可以消除一些重复字段的冗余拷贝。

![](https://static.oschina.net/uploads/space/2018/0509/150632_whyg_3820517.png)

能够被用作特征对象的对象，它必须是“对象安全”的，对象安全定义参考：[Object Safety](https://doc.rust-lang.org/reference/items/traits.html#object-safety)

**对象安全的 Trait 有下面特点**：

+ 所有 supertrait 也必须是对象安全的。
+ 不能以 Sized 不能作为 supertrait。换句话说，不能有约束 Self: Sized。
+ 它不能有任何相关的常量。
+ 它不能有任何带有泛型的关联类型。
+ 所有的关联函数必须要么可以从 trait 对象分派，要么明确表示不可分派：
  + 可分派的函数必须满足以下条件：
    + 不带有任何类型参数（尽管生命周期参数是允许的）。
    + 是一个方法，并且除了在接收者类型中使用 Self 外，不使用 Self。
    + 接收者的类型必须是以下类型之一：
      + &Self（即 &self）
      + &mut Self（即 &mut self）
      + Box<Self>
      + Rc<Self>
      + Arc<Self>
      + Pin<P> 其中 P 是上述类型之一
    + 不具有不透明的返回类型；也就是说，
      + 不是异步函数（异步函数具有隐式的 Future 类型）。
      + 不具有返回位置上的 impl Trait 类型（例如 fn example(&self) -> impl Trait）。
    + 不具有 where Self: Sized 的约束（Self 类型的接收者（即 self）隐含了这一点）。
  + 明确不可分派的函数要求：
    + 具有 where Self: Sized 的约束（Self 类型的接收者（即 self）隐含了这一点）。

参考：

+ [深入理解 Rust 的动态分派模型](https://www.oschina.net/translate/exploring-dynamic-dispatch-in-rust?print)

