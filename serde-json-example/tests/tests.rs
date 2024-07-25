use serde::{Deserialize, Serialize};
use serde_json;

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
}

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
