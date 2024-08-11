# Rust 内存布局

参考：

+ [Type Layout](https://doc.rust-lang.org/reference/type-layout.html)
+ [浅聊 Rust 程序内存布局](https://rustcc.cn/article?id=98adb067-30c8-4ce9-a4df-bfa5b6122c2e) (内部参考了官方文档，里面还有一些错误，不可轻信需要验证)

原文的理论缺乏测试实例支撑，这里添加一些测试实例进行验证，同时对一些重要知识点进行深入解析。

To be continued ...



## Rust 内存分配规则

+ 首字节地址address与存储宽度size都必须是【对齐位数alignment】的自然数倍。比如说，
  + 对齐位数alignment等于1字节的变量值可保存于任意内存地址address上。
  + 对齐位数alignment等于2字节且有效数据长度等于3字节的变量值
    + 仅能保存于偶数位的内存地址address上。
    + 存储宽度size也得是4字节 — 从有效长度3字节到存储宽度4字节的扩容过程被称作“对齐”。
  + 存储宽度size等于0字节的变量值可接受任意正整数作为其对齐位数alignment — 惯例是1字节。
+ 对齐位数alignment必须是2的自然数次幂。即，alignment = 2 ^ N且N是≼ 29的自然数。
+ 存储宽度size是有效数据长度加对齐填充位数的总和字节数。
+ address，size与alignment的计量单位都是“字节”。

> 需要内存对齐的原因：
>
> + 可以提升访问效率
>
>   在32位的CPU一次可以处理4个字节（Byte）的数据，那么CPU实际寻址的步长就是4个字节，也就是只对编号是4的倍数的内存地址进行寻址。同理64位的CPU的寻址步长是8字节，**只对编号是8的倍数的内存地址进行寻址**，64位CPU也可以向下兼容32位寻址。
>
>   例如一个double类型的数据在内存中占据8个字节，64位CPU对齐进行读取，如果 double 数据已经对齐，会存储到8的倍数的内存地址，只需要一次寻址，如果没有对齐比如放到了地址为9的地址，则需要两次寻址才能读取到完整数据（**分别对地址8和地址16进行寻址**），效率会大大折扣。
>
> + 硬件要求
>
> **首字节地址**，类型有效数据的初始地址，注意不是上面的寻址地址。经过测试发现不足4字节或8字节的类型的读取可能会和其他不足4字节或8字节的类型变量一起被读取。
>
> 不同数据类型的对齐位数：
>
> 参考单元测试 `test_align_of()`。
>
> 存储宽度size是类型实际变量实际占用字节数，不同数据类型的存储宽度：
>
> 参考单元测试 `test_size_of()`。
>
> 有效数据长度 = 存储宽度（size）- 对齐填充（alignment padding）长度。

对齐填充`alignment padding`又分为

- **小**端填充`Little-Endian padding` — `0`填充位出现在有效数据**右**侧的**低**位
- **大**端填充`Big-Endian padding` — `0`填充位出现在有效数据**左**侧的**高**位

> 个人感觉这里说的不准确，内存只是分高地址和低地址应该不分左右；
>
> 大端存储（Big-endian）将数据的高字节存储在内存的低地址处，低字节存储在内存的高地址处。
> 小端存储（Little-endian）将数据的低字节存储在内存的低地址处，高字节存储在内存的高地址处。
> 小端存储通常用于大多数的 Intel x86 以及 ARM 处理器架构中。
>
> 从单元测试 `test_endian_padding()`的测试结果看使用的小端填充，
>
> - **小**端填充`Little-Endian padding` — `0`填充位出现在有效数据高地址位
> - **大**端填充`Big-Endian padding` — `0`填充位出现在有效数据的低地址位
>
> ```rust
> #[test]
> fn test_endian_padding() {
>     #[derive(Debug)]
>     struct A {
>         a: i32,
>         b: i16,
>         c: i8,  //这里会填充1字节，到底是大端还是小端填充，可以查看 Memory View
>     }
>     let a = A {
>         a: 0x7fffffff,
>         b: 0x7fff,
>         c: 0x7f,
>     };
>     println!("{:?}", a) // Memory View 结果： ff ff ff 7f ff 7f 7f 00
>     					// 					 数据低字节 数据高字节
>     					// 					 内存低地址      	   内存高地址
>     #[derive(Debug)]
>     struct B {
>         a: i8,
>         b: i32,
>         c: i16,
>     }
>     let b = B {
>         a: 0x7f,
>         b: 0x7fffffff,
>         c: 0x7fff,
>     };
>     println!("{:?}", b); // Memory View 结果： ff ff ff 7f ff 7f 7f 00， B 和 A 成员变量声明顺序不一样但是存储结构是一样的。什么原理？也许和后面默认布局方案说的“重排字段存储顺序”有关
> }
> ```

如果【对齐位数`alignment`】与【存储宽度`size`】在**编译时**已知，那么该类型`<T: Sized>`就是【静态分派】`Fixed Sized Type`。于是，

- **类型**的对齐位数可由[std::mem::align_of::()](https://doc.rust-lang.org/std/mem/fn.align_of.html)读取
- **类型**的存储宽度可由[std::mem::size_of::()](https://doc.rust-lang.org/std/mem/fn.size_of.html)读取

若【对齐位数`alignment`】与【存储宽度`size`】在**运行时**才可计算知晓，那么该类型`<T: ?Sized>`就是【动态分派】`Dynamic Sized Type`。于是，

- **值**的对齐位数可由[std::mem::align_of_val::(&T)](https://doc.rust-lang.org/std/mem/fn.align_of_val.html)读取
- **值**的存储宽度可由[std::mem::size_of_val::(&T)](https://doc.rust-lang.org/std/mem/fn.size_of_val.html)读取

> 对于静态分派类型，个人理解上面 align_of::() align_of_val::(&T) 返回值是一样的



## 简单类型内存布局

### 基本数据类型

基本数据类型包括bool，u8，i8，u16，i16，u32，i32，u64，i64，u128，i128，usize，isize，f32，f64和char。它们的内存布局在不同型号的设备上略有差异。

x86_64 CPU架构下测试结果：

即**基本数据类型在 x86 64位CPU架构下的对齐和存储宽度是相等的**。

```
bool: alignment:1, size:1
u8:   alignment:1, size:1
i8:   alignment:1, size:1
u16:  alignment:2, size:2
i16:  alignment:2, size:2
u32:  alignment:4, size:4
i32:  alignment:4, size:4
u64:  alignment:8, size:8
i64:  alignment:8, size:8
u128: alignment:16, size:16
i128: alignment:16, size:16
usize: alignment:8, size:8
isize: alignment:8, size:8
f32:  alignment:4, size:4
f64:  alignment:8, size:8
char: alignment:4, size:4
```

> 这部分上面中文参考文档（[浅聊 Rust 程序内存布局](https://rustcc.cn/article?id=98adb067-30c8-4ce9-a4df-bfa5b6122c2e) ）好像写错了。
>
> 另外官方文档翻译过来（都是模糊的说辞，也没有参考意义）：
>
> **基本类型的对齐是平台特定的**。在大多数情况下，它们的对齐等于它们的大小，但可能更小。特别是，i128和u128通常对齐为4或8字节，即使它们的大小是16，并且在许多32位平台上，i64, u64和f64只对齐为4字节，而不是8字节。

### 指针与引用

> FST 指 Fixed Sized Type, 即上面的静态分派，DST 是 Dynamic Sized Type 动态分派 。
>
> **瘦指针**不是官方的说法，而是 Rust 社区中对官方定义的**裸指针*、原生指针（Raw Pointer）*的一种称呼，指的是那些不携带额外元数据的简单指针，如 *const T 或 *mut T。
>
> 相对的社区中还有**胖指针**的说法，对应Rust官方的**智能指针**，指的是不仅包含指向数据的地址还包含额外的元数据（如所有权计数、生命周期信息等）的指针，如 Box<T>, Rc<T>, Arc<T>, RefCell<T>, Mutex<T> 。

指针和引用拥有相同的内存布局。

指向 FST 类型的指针拥有与usize类型是一样的，不管是裸指针还是智能指针。

指向 DST 类型的指针是固定大小的。保证其大小和对齐方式至少等于指针的大小和对齐方式。不管是裸指针还是智能指针。

官方注释：“虽然您不应该依赖于此，但**所有指向DST的指针当前都是usize大小的两倍，并且具有相同的对齐方式**”。

x86_64 CPU架构下测试结果：

```rust
// 指向 FST 类型的裸指针
*const i16: alignment:8, size:8
*mut i16: alignment:8, size:8
*const i32: alignment:8, size:8
*mut i32: alignment:8, size:8
// 指向 FST 类型的智能指针
Box<i16>: alignment:8, size:8
// 指向 DST 类型的裸指针
*const [i32]: alignment:8, size:16
// 指向 DST 类型的智能指针
Box<[i16]>: alignment:8, size:16
Rc<[i8]>: alignment:8, size:16
Arc<[i16]>: alignment:8, size:16
```

### 数组

数组 `[T;N]` 的大小为 `size_of::<T>() * N`，与 `T` 的对齐方式相同。

### 切片

切片和数组的内存布局相同。

注意: 这里指原始[T]类型，而不是指向切片的指针(&[T]， Box<[T]>等)。

### 字符串

字符串切片是字符的UTF-8表示，与类型[u8]的切片具有相同的布局。

### 元祖

元组是根据Rust自定义数据类型内存布局方案（Representation）进行布局的。唯一的例外是单元元组(())，它被保证为一个大小为0、对齐为1的零大小类型。

### Trait

Trait对象具有与Trait对象的值相同的布局。

### 闭包

闭包没有布局保证。



## 自定义数据类型的内存布局

**自定义数据结构的内存布局包含如下五个属性**：

1. alignment
   - 定义：数据结构自身的对齐位数
   - 规则：
     - `alignment` = `2`的`n`次幂（`n` 是`≼ 29`的自然数）
     - 不同于基本数据类型`alignment = size`，自定义数据结构`alignment`的算法随不同的数据结构而相异。
2. size
   - 定义：数据结构自身的宽度
   - 规则：`size`必须是`alignment`自然数倍。若**有效**数据长度`payload_size`不足`size`，就添补空白【对齐填充位】凑足宽度。
3. field.alignment
   - 定义：每个字段的对齐位数
   - 规则：`field.alignment` = `2`的`n`次幂（`n`是`≼ 29`的自然数）
4. field.size
   - 定义：每个字段的宽度
   - 规则：`field.size`必须是`field.alignment`自然数倍。若**有效**数据长度`field.payload_size`不足`field.size`，就添补空白【对齐填充位】凑足宽度。
5. field.offset
   - 定义：每个字段首字节地址相对于**上一层**数据结构首字节地址的偏移字节数
   - 规则：
     - `field.offset`必须是`field.alignment`自然数倍。若不足，就垫入空白【对齐填充位】和向后推移当前字段的起始位置。
     - 前一个字段的`field.offset + field.size ≼` 后一个字段的`field.offset`

**编译器内置了四款内存布局方案**：

+ **默认`Rust`内存布局** — 没有元属性注释

+ `C`内存布局 `#[repr(C)]`

+ 数字类型·内存布局 `#[repr(u8 / u16 / u32 / u64 / u128 / usize / i8 / i16 / i32 / i64 / i128 / isize)]`

  - 仅适用于枚举类。

  - 支持与`C`内存布局**混搭**使用。比如，`#[repr(C, u8)]`。

+ 透明·内存布局 `#[repr(transparent)]`

  - 仅适用于单字段数据结构。

### 默认Rust内存布局

相较于`C`内存布局，`Rust`内存布局面向**内存空间利用率**做了**优化** — 省内存。具体的技术手段包括`Rust`编译器

- 重排了字段的存储顺序，以尽可能多地消减掉“边角料”（对齐填充）占用的字节位数。于是，在源程序中字段声明的**词法次序**经常**不同于**【运行时】它们在内存里的**实际存储顺序**。
- 允许多个**零宽度**字段共用一个内存地址。甚至，**零宽度**字段也被允许与普通（有数据）字段共享内存地址。

以`C ABI`中间格式为桥的`C`内存布局虽然实现了`Rust`**跨语言**数据结构，但它却更费内存。这主要出于两个方面原因：

1. `C`内存布局**未对**字段存储顺序做优化处理，所以字段在源码中的词法顺序就是它们在内存条里的存储顺序。于是，**若 @程序员 没有拿着算草纸和数着比特位“人肉地”优化每个数据结构定义，那么由对齐填充位冗余造成的内存浪费不可避免**。
2. `C`内存布局不支持零宽度数据类型。零宽度数据类型是`Rust`语言设计的重要创新。相比之下，
   - （参见`C17`规范的第`6.7.2.1`节）**无**字段结构体会导致标准`C`程序出现`U.B.`，除非安装与开启`GNU`的`C`扩展。
   - `Cpp`编译器会强制给**无字段**结构体安排**一个**字节宽度，除非该数据结构被显式地标记为`[[no_unique_address]]`。

以费内存为代价，`C`内存布局赋予`Rust`数据结构的另一个“超能力”就是：“仅通过变换【指针类型】就可将内存上的一段数据重新解读为**另一个**数据类型的值”。比如，`void * / std::ffi::c_void`被允许指向任意数据类型的变量值 [例程](https://github1s.com/stuartZhang/my_rs_ideas_playground/blob/main/src/bin/ffi-closure-callback.rs#L83)。但在`Rust`内存布局下，需要调用专门的标准库函数[std::intrinsics::transmute()](https://doc.rust-lang.org/std/intrinsics/fn.transmute.html)才能达到相同的目的。

除了上述鲜明的差别之外，`C`与`Rust`内存布局都允许【对齐位数`alignment`】参数被微调，而不一定总是全部字段`alignment`中的最大值。这包括但不限于：

- 修饰符`align(x)`增加`alignment`至指定值。例如，`#[repr(C, align(8))]`将`C`内存布局中的【对齐位数】**上调至**`8`字节
- 修饰符`packed(x)`减小`alignment`至指定值。例如，`#[repr(packed)]`将默认`Rust`内存布局中的【对齐位数】**下调至**`1`字节
