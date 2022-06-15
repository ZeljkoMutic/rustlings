// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` for hints!



struct Wrapper<T> {
    value: T,
}



impl<T: ToString> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
    // pub fn split(self) -> Vec<T>{
    //     self.value.to_string().split().collect().iter().parse().unwrap()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }


    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }

    // fn store_vec_in_wrapper(){
    //     assert_eq!(Wrapper::new("Foo").value,"Foo");
    // }
}
