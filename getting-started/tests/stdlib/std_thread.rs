#[test]
fn parallelism() {
    let parallelism = std::thread::available_parallelism()
        .map_or(1, |n| n.get()).min(12);
    println!("{parallelism}")
}