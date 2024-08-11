use std::ptr::addr_of;

#[test]
fn addr_of() {
    let a = 1i32;   //0x00007f26880a943c, RustRover Memory View 中查找到的地址
    let b = 2i16;   //0x00007f26880a9440
    let c = 3i8;     //0x00007f26880a9443
    println!("The address of value is: {:p}", &a);          //0x00007f26880a9444，这两种方式都不是获取a的实际地址
    println!("The address of value is: {:p}", addr_of!(a)); //0x00007f26880a9444，
}
