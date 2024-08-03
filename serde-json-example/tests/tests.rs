use serde::{Deserialize, Serialize};
use serde_json;

/// 测试目标：
/// 1 嵌套结构体、枚举的反序列化
/// 2 基本数据类型、枚举反序列化默认值
/// 3 字符串格式转换
/// 4 Option 字段为空时反序列化
#[test]
fn usage() {
    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(default)]
    struct User {
        name: String,
        // #[serde(default)]
        age: u8,
        is_active: bool,
        addr: Addr,
        status: Option<Status>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    struct Addr {
        addr: String,
        id: u32,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    enum Status {
        #[default]
        Active,
        Inactive,
    }

    // let json = r#"{"name": "John Doe", "age": 30, "is_active": true, "addr": {"addr": "123 Main St", "id": 12345}, "status": "active"}"#;
    let json = r#"{"name": "John Doe", "is_active": true, "addr": {"addr": "123 Main St", "id": 12345}, "status": "inactive"}"#;
    let user: User = serde_json::from_str(json).expect("Failed to parse JSON");

    assert_eq!(user.name, "John Doe");
    // assert_eq!(user.age, 30);
    assert!(user.is_active);
    assert_eq!(user.addr.addr, "123 Main St");
    assert_eq!(user.addr.id, 12345);

    // 4 Option 字段为空时反序列化
    // let json = r#"{"name": "John Doe", "is_active": true, "addr": {"addr": "123 Main St", "id": 12345}}"#;
    let json = r#"{"name": "John Doe", "is_active": true, "addr": {"addr": "123 Main St", "id": 12345}, "status": null}"#;
    let user: User = serde_json::from_str(json).expect("Failed to parse JSON");
    assert!(user.status.is_none());
    // 注意序列化后 status 字段为 null, 不是省略 status 字段
    println!("{}", serde_json::to_string(&user).unwrap())   //{"name":"John Doe","age":0,"is_active":true,"addr":{"addr":"123 Main St","id":12345},"status":null}
}

/// 测试目标：
/// 1 结构体数组的反序列化
#[test]
fn usage_for_array() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Person {
        name: String,
        age: u8,
    }

    let json_array_str = r#"[{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]"#;
    let people: Vec<Person> = serde_json::from_str(json_array_str)
        .expect("JSON was not well-formatted");

    // 输出反序列化后的数据
    for person in people {
        println!("{:?}", person);
    }
}

/// 测试目标：
/// 1 测试JSON中存在但是结构体中不存在的字段能否影响反序列化，（并不影响）
#[test]
fn test_json_field_not_exist_in_struct() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Person {
        name: String,
        age: u8,
    }

    let json_array_str = r#"{"name": "Alice", "age": 30, "country": "China"}"#;
    let person: Person = serde_json::from_str(json_array_str)
        .expect("JSON was not well-formatted");
    println!("{:?}", person);
}

/// 测试目标：
/// 1 枚举->字符串格式转换
#[test]
fn usage_for_enum() {
    // "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case", "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE"
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    // #[serde(rename_all = "UPPERCASE")]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    enum ReviewEvent {
        /// 只是评论
        Comment,
        /// 合并投票，一个PR一般需要达到最低投票数才能合并
        Approve,
        /// 必须修改，否则PR无法合并
        RequestChanges,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Outer {
        event: ReviewEvent,
    }

    let outer = Outer {
        event: ReviewEvent::Comment,
    };
    let json = serde_json::to_string(&outer).unwrap();
    assert_eq!(json, r#"{"event":"COMMENT"}"#);

    let outer = Outer {
        event: ReviewEvent::Approve,
    };
    let json = serde_json::to_string(&outer).unwrap();
    assert_eq!(json, r#"{"event":"APPROVE"}"#);

    let outer = Outer {
        event: ReviewEvent::RequestChanges,
    };
    let json = serde_json::to_string(&outer).unwrap();
    assert_eq!(json, r#"{"event":"REQUEST_CHANGES"}"#);
}

/// 测试目标：
/// 1 Option 字段为 None 时, 序列化不包含此字段
#[test]
fn usage_for_option_none() {
    // Option 字段为 None 时序列化不展示此字段
    #[derive(Debug, Serialize, Deserialize)]
    struct Outer {
        // 如果 Option 类型值为 None, 序列化时不包含此字段
        #[serde(skip_serializing_if = "Option::is_none")]
        event: Option<String>,
    }

    let outer = Outer {
        event: None,
    };
    let json = serde_json::to_string(&outer).unwrap();
    assert_eq!(json, "{}");
}