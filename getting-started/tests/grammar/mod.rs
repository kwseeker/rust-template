mod oop_tests;
mod mod_tests;
mod option_tests;
mod tuple_tests;
mod lifetime;
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
mod oop_structure_tests;
mod trait_as_ref_tests;
mod pointer_smart_tests;
mod macros_attribute_tests;
mod data_type_slice_tests;
mod data_type_str_tests;
mod lifetime_specifier;
mod trait_marker;
mod macro_derive;
mod file;
mod generic;
mod async_await;
mod pointer_smart_rc_refcell;
mod loop_match;
mod lifetime_oom;
mod pointer_smart_box;
mod dynamic_dispatch;
mod error_handle;

// mod.rs中也可以定义测试
#[cfg(test)]
mod tests {
    #[test]
    fn test_in_mod() {
        println!("test test_in_mod()");
    }
}