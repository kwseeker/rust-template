/*
宏
宏属于元编程，在编译阶段生成Rust代码。
现在很多语言都提供了宏操作，大致可以分为两类： 文本替换和语法扩展。
    C语言中的宏函数就属于文本替换。Rust 宏属于语法拓展。
参考：
    《Rust编程之道》（入门）
    《Rust宏小册》（深入，包含完整的语法讲解和丰富的实例）：https://zjp-cn.github.io/tlborm/#/
宏的分类：
    声明宏：使用macro_rules声明
        声明宏的定义格式：
            macro_rules! $name {
                $rule0 ;
                $rule1 ;
                // …
                $ruleN ;
            }
            每条规则的格式：
                 ($matcher) => {$expansion}
                matcher 还可以包含捕获 (captures)。即基于某种通用语法类别来匹配输入，并将结果捕获到元变量 (metavariable) 中，然后将替换元变量到输出。
                捕获的书写方式是：先写美元符号 $，然后跟一个标识符，然后是冒号 :，最后是捕获方式，比如 $e:expr。
                捕获方式又被称作“片段分类符” (fragment-specifier)，必须是以下一种：
                    block：一个块（比如一块语句或者由大括号包围的一个表达式）
                    expr：一个表达式 (expression)
                    ident：一个标识符 (identifier)，包括关键字 (keywords)
                    item：一个条目（比如函数、结构体、模块、impl 块）
                    lifetime：一个生命周期注解（比如 'foo、'static）
                    literal：一个字面值（比如 "Hello World!"、3.14、'🦀'）
                    meta：一个元信息（比如 #[...] 和 #![...] 属性内部的东西）
                    pat：一个模式 (pattern)
                    path：一条路径（比如 foo、::std::mem::replace、transmute::<_, int>）
                    stmt：一条语句 (statement)
                    tt：单棵标记树
                    ty：一个类型
                    vis：一个可能为空的可视标识符（比如 pub、pub(in crate)）
    过程宏：
        函数式：实现 $name！$input 功能的宏
        属性式：实现 #[$input] 功能的属性
        derive 式：实现 #[derive($name)] 功能的属性
宏按使用语法形式分类：
    调用宏：
    属性宏：
宏展开过程：
宏和函数的区别：
    宏可以接收不通数量的参数；宏可以在编译器翻译代码前展开（比如宏可以在一个给定类型上实现Trait）；
    宏定义比函数定义更复杂，也更难以阅读、理解和维护；
    在一个文件里调用宏 之前 必须定义它，或将其引入作用域，而函数则可以在任何地方定义和调用。

*/

//最简单的声明宏定义，这里可以看到声明宏最基础的结构
#[macro_export]
macro_rules! one {
    () => {1}
}

#[macro_export]
macro_rules! num {
    ( $($num:literal),* ) => {

    };
}

///这里分支模式的解释：( $( $x:expr ),* )
/// 美元符号（$ ）在宏系统中声明一个变量来包含匹配该模式的 Rust 代码。美元符号明确表明这是一个宏变量而不是普通 Rust 变量。
/// 之后是一对括号，其捕获了符合括号内模式的值用以在替代代码中使用。$() 内则是 $x:expr ，其匹配Rust 的任意表达式，并将该表达式命名为 $x 。
/// $() 之后的逗号说明一个可有可无的逗号分隔符可以出现在 $() 所匹配的代码之后。紧随逗号之后的 * 说明该模式匹配零个或更多个 * 之前的任何模式。
#[macro_export]
macro_rules! my_vec {
    //这里 x 是自定义命名, expr 是一个片段分类符： fragment specifier，
    // valid fragment specifiers are `ident`, `block`, `stmt`, `expr`, `pat`, `ty`, `lifetime`, `literal`, `path`, `meta`, `tt`, `item` and `vis`
    // ( $( $x:expr ),* ) => {
    ( $( $var:expr ),* ) => { //
        {
            let mut temp_vec = Vec::new();
            $(
                // temp_vec.push($x);
                temp_vec.push($var);
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! string {
    ($e:expr) => {String::from($e)};
}

#[test]
fn test_marco() {
    let one = one!();
    println!("one: {one}");
    let v = my_vec![1, 3, 5];
    println!("v: {v:?}");
    let s = string!("hello");
    println!("s: {s}");
}

