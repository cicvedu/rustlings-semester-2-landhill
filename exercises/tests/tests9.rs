extern "Rust" {
    //fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}
 
mod foo {
   
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }
    pub fn my_demo_function_alias(a: u32) -> u32 {
        a
    }
}
 
#[cfg(test)]
mod tests {
    use super::*;
    use foo::my_demo_function;
    use foo::my_demo_function_alias;
    #[test]
    fn test_success() {

        unsafe {
            let result1 = my_demo_function(123);
            let result2 = my_demo_function_alias(456);

            // 使用 assert_eq! 来验证返回值是否符合预期
            assert_eq!(result1, 123);
            assert_eq!(result2, 456)
        }
    }

}