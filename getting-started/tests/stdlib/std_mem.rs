use std::mem::{align_of, align_of_val, size_of};
use std::ptr::addr_of;
use std::rc::Rc;
use std::sync::Arc;

/// Rust 内存布局

/// 读取类型的对齐位数
#[test]
fn test_align_of() {
    struct A {
        a: i32,
        b: i16,
        c: i8,
    }
    struct B {
        a: i64,
        b: i16,
        c: i8,
    }

    // align of A: 4
    // align of B: 8
    // align of usize: 8
    // align of isize: 8
    // align of i128: 16
    // align of i64: 8
    // align of i32: 4
    // align of i16: 2
    // align of i8: 1
    // align of u64: 8
    // align of u16: 2
    // align of f64: 8
    println!("align of A: {}", align_of::<A>());
    println!("align of B: {}", align_of::<B>());
    println!("align of usize: {}", align_of::<usize>());
    println!("align of usize value: {}", align_of_val(&1usize));
    println!("align of isize: {}", align_of::<isize>());
    println!("align of i128: {}", align_of::<i128>());
    println!("align of i128: {}", align_of_val(&2i128));
    println!("align of i64: {}", align_of::<i64>());
    println!("align of i32: {}", align_of::<i32>());
    println!("align of i16: {}", align_of::<i16>());
    println!("align of i8: {}", align_of::<i8>());
    println!("align of u64: {}", align_of::<u64>());
    println!("align of u16: {}", align_of::<u16>());
    println!("align of f64: {}", align_of::<f64>());
}

/// 读取类型的存储宽度
#[test]
fn test_size_of() {
    struct A {
        a: i32,
        b: i16,
        c: i8,
    }
    struct B {
        a: i64,
        b: i16,
        c: i8,
    }

    // size of A: 8
    // size of B: 16
    // size of usize: 8
    // size of isize: 8
    // size of i128: 16
    // size of i64: 8
    // size of i32: 4
    // size of i16: 2
    // size of i8: 1
    // size of u64: 8
    // size of u16: 2
    // size of f64: 8
    println!("size of A: {}", size_of::<A>());
    println!("size of B: {}", size_of::<B>());
    println!("size of usize: {}", size_of::<usize>());
    println!("size of isize: {}", size_of::<isize>());
    println!("size of i128: {}", size_of::<i128>());
    println!("size of i64: {}", size_of::<i64>());
    println!("size of i32: {}", size_of::<i32>());
    println!("size of i16: {}", size_of::<i16>());
    println!("size of i8: {}", size_of::<i8>());
    println!("size of u64: {}", size_of::<u64>());
    println!("size of u16: {}", size_of::<u16>());
    println!("size of f64: {}", size_of::<f64>());
}

/// 基本类型的内存布局
#[test]
fn test_primitive_layout() {
    // bool: alignment:1, size:1
    // u8:   alignment:1, size:1
    // i8:   alignment:1, size:1
    // u16:  alignment:2, size:2
    // i16:  alignment:2, size:2
    // u32:  alignment:4, size:4
    // i32:  alignment:4, size:4
    // u64:  alignment:8, size:8
    // i64:  alignment:8, size:8
    // u128: alignment:16, size:16
    // i128: alignment:16, size:16
    // usize: alignment:8, size:8
    // isize: alignment:8, size:8
    // f32:  alignment:4, size:4
    // f64:  alignment:8, size:8
    // char: alignment:4, size:4
    println!("bool: alignment:{}, size:{}", align_of::<bool>(), size_of::<bool>());
    println!("u8:   alignment:{}, size:{}", align_of::<u8>(), size_of::<u8>());
    println!("i8:   alignment:{}, size:{}", align_of::<i8>(), size_of::<i8>());
    println!("u16:  alignment:{}, size:{}", align_of::<u16>(), size_of::<u16>());
    println!("i16:  alignment:{}, size:{}", align_of::<i16>(), size_of::<i16>());
    println!("u32:  alignment:{}, size:{}", align_of::<u32>(), size_of::<u32>());
    println!("i32:  alignment:{}, size:{}", align_of::<i32>(), size_of::<i32>());
    println!("u64:  alignment:{}, size:{}", align_of::<u64>(), size_of::<u64>());
    println!("i64:  alignment:{}, size:{}", align_of::<i64>(), size_of::<i64>());
    println!("u128: alignment:{}, size:{}", align_of::<u128>(), size_of::<u128>());
    println!("i128: alignment:{}, size:{}", align_of::<i128>(), size_of::<i128>());
    println!("usize: alignment:{}, size:{}", align_of::<usize>(), size_of::<usize>());
    println!("isize: alignment:{}, size:{}", align_of::<isize>(), size_of::<isize>());
    println!("f32:  alignment:{}, size:{}", align_of::<f32>(), size_of::<f32>());
    println!("f64:  alignment:{}, size:{}", align_of::<f64>(), size_of::<f64>());
    println!("char: alignment:{}, size:{}", align_of::<char>(), size_of::<char>());
}

/// 瘦指针（裸指针、原生指针）内存布局
#[test]
fn test_fst_pointer_layout() {
    println!("*const i16: alignment:{}, size:{}", align_of::<*const i16>(), size_of::<*const i16>());
    println!("*mut i16: alignment:{}, size:{}", align_of::<*mut i16>(), size_of::<*mut i16>());
    println!("*const i32: alignment:{}, size:{}", align_of::<*const i32>(), size_of::<*const i32>());
    println!("*mut i32: alignment:{}, size:{}", align_of::<*mut i32>(), size_of::<*mut i32>());
    println!("*const [i32]: alignment:{}, size:{}", align_of::<*const [i32]>(), size_of::<*const [i32]>());
    // addr_of! 宏可以为一个值创建一个裸指针
    // let a = 1i16;
    // let mut b = 1i16;
    // let ptr = addr_of!(a);
    // let ptr_mut = addr_of_mut!(b);
    let arr = [1i32, 2, 3];
    let ptr = addr_of!(arr);
    println!("arr: {:?}", ptr);
}

#[test]
fn test_dst_pointer_layout() {
    println!("Box<[i16]>: alignment:{}, size:{}", align_of::<Box<[i16]>>(), size_of::<Box<[i16]>>());
    println!("Rc<[i8]>: alignment:{}, size:{}", align_of::<Rc<[i8]>>(), size_of::<Rc<[i8]>>());
    println!("Arc<[i16]>: alignment:{}, size:{}", align_of::<Arc<[i16]>>(), size_of::<Arc<[i16]>>());
    println!("Box<i16>: alignment:{}, size:{}", align_of::<Arc<i16>>(), size_of::<Arc<i16>>());
}

#[test]
fn test_cpu_addressing() {
    struct A {
        a: i8,
    }
    let a = A {     //某次测试地址值 0x00007f35f5dc54c6
        a: 0x7f,
    };
    let b = 0x7fffffffi32;
    let c = A {     //某次测试地址值 0x00007f35f5dc54c7
        a: 0x6f,
    };
    // 为何 a b c 存储结构是 ff ff ff 7f 00 00 7f 6f ? 看上去是先将b初始化后再初始化a c, 貌似经过默认布局方案说的“重排字段存储顺序”处理
    // 另外为何不是 ff ff ff 7f 7f 6f 00 00 ? TODO 上面看上去不太符合内存填充规则
    println!("align of A: {}", align_of::<A>());  //对齐位数 1
    println!("size of A: {}", size_of::<A>());  //存储宽度 1
}

/// 关于对齐填充的大小端填充
#[test]
fn test_endian_padding() {
    #[derive(Debug)]
    struct A {
        a: i32,
        b: i16,
        c: i8,  //这里会填充1字节，到底是大端还是小端填充，可以查看 Memory View
    }
    let a = A {
        a: 0x7fffffff,
        b: 0x7fff,
        c: 0x7f,
    };
    #[derive(Debug)]
    struct B {
        a: i8,
        b: i32,
        c: i16,
    }
    let b = B {
        a: 0x7f,
        b: 0x7fffffff,
        c: 0x7fff,
    };
    println!("{:?}", a); // Memory View 结果： ff ff ff 7f ff 7f 7f 00， 可以看到是小端填充
    println!("{:?}", b); // Memory View 结果： ff ff ff 7f ff 7f 7f 00， B 和 A 成员变量声明顺序不一样但是存储结构是一样的， 貌似经过默认布局方案说的“重排字段存储顺序”处理
}