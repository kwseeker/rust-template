use std::collections::BTreeMap;
use std::fmt::Write;                    //TODO Rust 包导入规则？ 发现不会默认导入，write! 有用到这个 Trait 的 write_fmt() 方法，虽然 String 有实现这个 Trait 但是不手动导入的话， 会报错
use crate::options::{Category, Flag};
use crate::options::defs::FLAGS;
use crate::options::doc::version;

// include_str! 加载 UTF-8 编码的文件为字符串
const TEMPLATE_SHORT: &'static str = include_str!("template.short.help");
// const TEMPLATE_LONG: &'static str = include_str!("template.long.help");

macro_rules! write {
    ($($tt:tt)*) => { std::write!($($tt)*).unwrap(); }
}

/// 生成简短的帮助信息，原理就是使用选项信息替换模板文件中的占位符(!!...!!)
/// 生成详细的帮助信息生成原理类似不展示了
pub(crate) fn generate_short() -> String {
    //BTreeMap基于多路平衡查找树, Key 是 选项类型 Category, Value 是 Vec的元组 （这里元祖实现了类似二维数组的效果，两列多行）
    let mut cats: BTreeMap<Category, (Vec<String>, Vec<String>)> = BTreeMap::new();
    let (mut maxcol1, mut maxcol2) = (0, 0);
    //遍历所有选项信息
    for flag in FLAGS.iter().copied() {
        let columns = cats.entry(flag.doc_category()).or_insert((vec![], vec![]));
        //第一列是选项长短名称拼接的字符串，第二列是选项短描述信息
        let (col1, col2) = generate_short_flag(flag);
        maxcol1 = maxcol1.max(col1.len());
        maxcol2 = maxcol2.max(col2.len());
        columns.0.push(col1);
        columns.1.push(col2);
    }
    //short help doc
    let mut out = TEMPLATE_SHORT.replace("!!VERSION!!", &version::generate_digits());
    for (cat, (col1, col2)) in cats.iter() {
        let var = format!("!!{name}!!", name = cat.as_str());
        let val = format_short_columns(col1, col2, maxcol1, maxcol2);
        out = out.replace(&var, &val);
    }
    out
}

//元组第一列是选项长短名称拼接的值，第二列是简短的描述信息
fn generate_short_flag(flag: &dyn Flag) -> (String, String) {
    let (mut col1, mut col2) = (String::new(), String::new());
    //第一列信息拼接：比如：-h,--help
    if let Some(byte) = flag.name_short() {
        let name = char::from(byte);
        write!(col1, r"-{name}");
        write!(col1, r", ");
    }
    write!(col1, r"--{name}", name = flag.name_long());
    //第二列信息（选项描述信息）
    write!(col2, "{}", flag.doc_short());
    (col1, col2)
}

fn format_short_columns(
    col1: &[String],
    col2: &[String],
    maxcol1: usize,
    _maxcol2: usize,
) -> String {
    assert_eq!(col1.len(), col2.len(), "columns must have equal length");
    const PAD: usize = 2;
    let mut out = String::new();
    for (i, (c1, c2)) in col1.iter().zip(col2.iter()).enumerate() {
        if i > 0 {
            write!(out, "\n");
        }

        let pad = maxcol1 - c1.len() + PAD;
        write!(out, "  ");
        write!(out, "{c1}");
        write!(out, "{}", " ".repeat(pad));
        write!(out, "{c2}");
    }
    out
}