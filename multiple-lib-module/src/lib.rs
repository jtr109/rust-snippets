// lib.rs

mod mod_one; // announce mod_one as a lib module

pub use crate::mod_one::hello;

#[cfg(test)]
mod test {
    use super::hello;

    #[test]
    fn test_say_hello() {
        let expected = "Hello, Ryan!";
        assert_eq!(hello("Ryan"), expected);
    }
}
