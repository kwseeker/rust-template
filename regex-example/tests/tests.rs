use regex::Regex;

#[test]
fn split() {
    let regex = Regex::new(r"@@\s-(\d+)(?:,(\d+))?\s\+(\d+)(?:,(\d+))?\s@@").unwrap(); // 创建一个正则表达式，这里使用\s+来匹配一个或多个空白字符
    let text = "@@ -0,0 +1,93 @@\n+Hello, World\n+Hello, Lee\n@@ -0,0 +1,93 @@\n+Hello, Rust\n@@ -0,0 +1,93 @@\n+Hello, Regex\n";

    let split_text: Vec<&str> = regex.split(text).collect(); // 使用正则表达式切分字符串
    assert_eq!(split_text.len(), 4);
    assert_eq!(split_text[0], "");
    assert_eq!(split_text[1], "\n+Hello, World\n+Hello, Lee\n");
    assert_eq!(split_text[2], "\n+Hello, Rust\n");
    assert_eq!(split_text[3], "\n+Hello, Regex\n");
}

#[test]
fn captures() {
    let regex = Regex::new(r"@@\s-(\d+)(?:,(\d+))?\s\+(\d+)(?:,(\d+))?\s@@").unwrap(); // 创建一个正则表达式，这里使用\s+来匹配一个或多个空白字符
    // let text = "@@ -0,0 +1,93 @@\n+Hello, World\n+Hello, Lee\n";
    let text = "@@ -1 +1 @@\n+Hello, World\n+Hello, Lee\n";
    let mat = regex.is_match(text);
    assert!(mat);

    let caps = regex.captures(text).unwrap();
    let old_start = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let old_lines = caps.get(2).map_or(0, |m| m.as_str().parse::<usize>().unwrap());
    let new_start = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
    let new_lines = caps.get(4).map_or(0, |m| m.as_str().parse::<usize>().unwrap());

    println!("Numbers: {old_start}, {old_lines}, {new_start}, {new_lines}");
}

#[test]
fn find_iter() {
    let regex = Regex::new(r"@@\s-(\d+)(?:,(\d+))?\s\+(\d+)(?:,(\d+))?\s@@").unwrap(); // 创建一个正则表达式，这里使用\s+来匹配一个或多个空白字符
    let text = "@@ -0,0 +1,93 @@\n+Hello, World\n+Hello, Lee\n@@ -0,0 +1,93 @@\n+Hello, Rust\n@@ -0,0 +1,93 @@\n+Hello, Regex\n";

    let matches: Vec<_> = regex.find_iter(text).map(|m| {
        println!("{:?}", m.range());
        m.as_str()
    }).collect();
    assert_eq!(matches.len(), 3);
    assert_eq!(matches[0], "@@ -0,0 +1,93 @@");
    assert_eq!(matches[1], "@@ -0,0 +1,93 @@");
    assert_eq!(matches[2], "@@ -0,0 +1,93 @@");
}

// 通过 find 实现带匹配的字段的拆分
#[test]
fn find() {
    let regex = Regex::new(r"@@\s-(\d+)(?:,(\d+))?\s\+(\d+)(?:,(\d+))?\s@@").unwrap(); // 创建一个正则表达式，这里使用\s+来匹配一个或多个空白字符
    let text = "@@ -0,0 +1,93 @@\n+Hello, World\n+Hello, Lee\n@@ -0,0 +1,93 @@\n+Hello, Rust\n@@ -0,0 +1,93 @@\n+Hello, Regex\n";

    let mut text1 = text.clone();
    loop {
        let mat = regex.find(text1);
        if mat.is_none() {
            break;
        }
        let range = mat.unwrap().range();
        let next_mat = regex.find(&text1[range.end..]);
        if (next_mat.is_some()) {
            let next_range = next_mat.unwrap().range();
            println!("{}", &text1[range.start..range.end + next_range.start]);
            text1 = &text1[range.end + next_range.start..];
        } else {
            println!("{}", &text1[range.start..]);
            break;
        }
    }
}