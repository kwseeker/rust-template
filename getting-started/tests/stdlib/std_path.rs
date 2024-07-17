use std::env;
use std::path::PathBuf;

#[test]
fn path_is_dir() {
    let cwd = env::current_dir().unwrap();
    println!("{:?}", cwd.as_path().as_os_str());
    let path = PathBuf::from("./crates/grep");
    let ret = path.is_dir();
    println!("{ret}")
}