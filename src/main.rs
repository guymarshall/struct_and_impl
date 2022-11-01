fn main() {
    struct Number {
        odd: bool,
        value: i32
    }
    impl Number {
        fn is_positive(self) -> bool {
            self.value > 0
        }
    }
    println!();
    // experiment with this
}
