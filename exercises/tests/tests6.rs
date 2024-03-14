// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.



struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {  

    // 使用 ptr::from_raw 从原始指针创建 Box  

    let ret: Box<Foo> = Box::from_raw(ptr);  

    ret  

}  

#[cfg(test)]  

mod tests {  

    use super::*;  

  

    #[test]  

    fn test_success() {  

        // 初始化 Foo 时将 b 字段设置为 Some("hello")  

        let data = Box::new(Foo { a: 1, b: Some("hello".to_owned()) });  

  

        let ptr_1 = &data.a as *const u128 as usize;  

        // 安全性：我们传递一个 Foo 的拥有盒  

        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };  

  

        let ptr_2 = &ret.a as *const u128 as usize;  

  

        // 确保两个指针相同（即它们指向相同的内存地址）  

        assert_eq!(ptr_1, ptr_2);  

        // 确保 ret 的 b 字段是我们设置的值  

        assert_eq!(ret.b, Some("hello".to_owned()));  

    }  

}
