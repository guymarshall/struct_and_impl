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
    let seven: Number = Number { odd: true, value: 7 };
    println!("Is seven positive: {}", seven.is_positive());
}
