// 面向对象实现的类型，通过结构体或枚举定义数据结构，通过impl块定义在结构体和枚举之上的方法
#[derive(Debug)]    //这一句用于自动派生 Debug 这一 trait 的方法，trait 的定位有点类似其他语言的接口
pub(crate) struct HiArgs {
    raw: String,
}

impl HiArgs {
    //相当于其他语言的静态方法
    pub fn new(val: &str) -> HiArgs {
        HiArgs { raw: String::from(val) }
    }

    pub fn raw(&self) -> &String {
        &self.raw
    }
}