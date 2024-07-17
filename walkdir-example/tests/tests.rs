use std::path::PathBuf;
use walkdir::WalkDir;

#[test]
fn walk_dir() {
    let path = PathBuf::from("./");
    let mut file_paths = Vec::new();
    if path.is_dir() {
        // 使用 WalkDir 遍历目录
        for entry in WalkDir::new(path) {
            // 处理每个条目
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Error accessing entry: {}", e);
                    continue;
                }
            };
            // 确保是文件
            if entry.file_type().is_file() {
                file_paths.push(entry.path().to_path_buf());
            }
        }
    } else {
        eprintln!("Provided path is not a directory: {:?}", path);
    }
    // 打印所有找到的文件路径
    for file_path in file_paths {
        println!("{:?}", file_path);
    }
}
