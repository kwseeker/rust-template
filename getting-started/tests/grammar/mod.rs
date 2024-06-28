mod oop_tests;
mod mod_tests;
mod option_tests;
mod tuple_tests;
mod life_cycle;
mod trait_tests;
mod concurrency_tests;
mod ownership_tests;
mod macro_tests;
mod error_handle_tests;
mod concurrency_lock_tests;
mod data_type_primitive_tests;
mod trait_constraint_tests;
mod pointer_tests;
mod patterns_tests;
mod data_type_convert_tests;
mod macros_stdlib_tests;

// mod.rs中也可以定义测试
#[cfg(test)]
mod tests {
    #[test]
    fn test_in_mod() {
        println!("test test_in_mod()");
    }
}