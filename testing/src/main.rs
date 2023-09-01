fn main() {}

#[cfg(test)]
mod code_tests {
    struct Rectangle {
        width: u8,
        height: u8,
    }

    impl Rectangle {
        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }
    fn give_two() -> i32 {
        2
    }
    #[test]
    #[should_panic]

    fn test_basic() {
        assert!(1 == 1);
        panic!("oh no...");
    }

    #[test]

    fn test_equals() {
        assert_eq!(give_two(), 1 + 1);
        assert_ne!(give_two(), 1 + 2);
    }

    #[test]
    #[should_panic]

    fn test_structs() {
        let r = Rectangle {
            width: 30,
            height: 10,
        };

        assert!(r.is_square());
    }
}
