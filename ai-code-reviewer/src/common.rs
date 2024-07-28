use serde::Serialize;

// 想用 Option<T> 代表一个可有可无的方法传参，但是传 None 时，还必须制定 T 的实际类型，这里就定义个空类型哄骗下编译器
#[derive(Serialize)]
pub(crate) struct Null {
}

