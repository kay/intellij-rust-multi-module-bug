
pub fn foo() -> i32 {
    2 + 2
}

#[cfg(test)]
mod tests {
    use crate::foo;

    #[test]
    fn it_works() {
        assert_eq!(foo(), 4);
    }
}
