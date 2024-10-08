# 宏编程

参考资料：
+ [宏小册](https://zjp-cn.github.io/tlborm/#/)

## 宏的分类

+ **声明宏**

  使用最广泛，定义类似 match 的代码。

  **宏也是将一个值跟对应的模式进行匹配，且该模式会与特定的代码相关联**。但是与 `match` 不同的是，**宏里的值是一段 Rust 源代码**(字面量)，模式用于跟这段源代码的结构相比较，一旦匹配，传入宏的那段源代码将被模式关联的代码所替换，最终实现宏展开。

+ **过程宏**

  + 派生宏 #[derive]
  + 类属性宏
  + 类函数宏

## 宏的优缺点

优点：

+ 可用于元编程
+ 支持可变参数
+ 宏展开发生在编译之前

缺点：

+ 语法复杂、难以理解和维护

## 宏的调试

rustc 提供了 `-Zunpretty=expanded` 参数用于查看宏展开后的代码。
由 dtolnay 制作的名为 cargo-expand 的 cargo 插件基本上对它进行了包装，使用起来更加方便。

## 常见宏源码解析
