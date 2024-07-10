#[test]
fn test_array() {
    let a = [1, 2, 3];
    let arr: [i32; 3] = [1, 2, 3];
    const ARR: &[i32] = &[1, 2, 3];
    let arr2: &[i32] = &a;

    for i in a {
        println!("{i}");
    }
    for i in ARR {
        println!("{i}");
    }
    for i in ARR.iter() {
        println!("{i}");
    }
    for i in arr2 {
        println!("{i}");
    }
}

/// 数字类型
#[test]
fn test_number() {
    assert_eq!(b'V', 86u8); //b开头的字符称为字节字面量，值为字符对应的ASCII码值
}

/// 字符串类型
#[test]
fn test_string() {
    //r开头的字符串称为原生字符串，这种语法可以保留字符串中的特殊符号，类似其他语言 """..."""
    //这个例子展示使用原生字符串存储矩阵，并计算对角线数字之和
    let s = r"1234
5678
9876
4321";
    let (mut x, mut y) = (0, 0);
    for (idx, val) in s.lines().enumerate() {   //一行一行遍历
        let val = val.trim();   //某行字符串
        let left = val.get(idx..idx + 1)//..用在两个数字之间表示区间范围，即[idx, idx+1)
            .unwrap().parse::<u32>().unwrap();
        let right = val.get((3 - idx)..(3 - idx + 1))
            .unwrap().parse::<u32>().unwrap();
        x += left;
        y += right;
    }
    assert_eq!(38, x + y);
}

/// 带防止溢出检查的运算
#[test]
fn checked_add() {
    let a = 254u8;
    let b = 1u8;
    let c = 2u8;
    let sum1 = a.checked_add(b);
    let sum2 = a.checked_add(c);
    assert_eq!(sum1, Some(255u8));
    assert_eq!(sum2, None);
}