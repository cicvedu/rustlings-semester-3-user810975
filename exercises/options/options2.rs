// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]  
mod tests {  
    use std::option::Option;  
  
    #[test]  
    fn simple_option() {  
        let target = "rustlings";  
        let optional_target: Option<&str> = Some(target);  
  
        if let Some(word) = optional_target {  
            assert_eq!(word, target);  
        }  
  
        // TODO: This line is incorrect and should be removed or corrected.  
        // No correction is provided here because the TODO comment indicates  
        // that this should be rewritten as an if let statement.  
        // word = optional_target {  
        //     assert_eq!(word, target);  
        // }  
    }  
  
    #[test]  
    fn layered_option() {  
        let range = 10;  
        let mut optional_integers: Vec<Option<i8>> = vec![None];  
  
        for i in 1..(range + 1) {  
            optional_integers.push(Some(i as i8));  
        }  
  
        let mut cursor = range;  
  
        while let Some(Some(integer)) = optional_integers.pop() {  
            assert_eq!(integer, cursor);  
            cursor -= 1;  
        }  
  
        // TODO: This line is incorrect and should be removed or corrected.  
        // No correction is provided here because the TODO comment indicates  
        // that this should be rewritten as a while let statement.  
        // integer = optional_integers.pop() {  
        //     assert_eq!(integer, cursor);  
        //     cursor -= 1;  
        // }  
  
        assert_eq!(cursor, 0);  
  
        // Assert that the vector is now empty  
        assert_eq!(optional_integers.len(), 0);  
    }  
}