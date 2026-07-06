mod m {
    pub struct S {
        value: i32,
    }

    impl S {
        pub fn new() -> Self {
            S { value: 0 }
        }

        pub fn get_value(&self) -> i32 {
            self.value
        }

        pub fn set_value(&mut self, v: i32) {
            self.value = v;
        }
    }
}

#[cfg(kani)]
mod verification {
    use super::m::*;

    #[kani::proof]
    fn main() {
        let mut s = S::new();

        let value = s.get_value();
        assert_eq!(value, 0);

        let test_value = kani::any::<i32>();
        s.set_value(test_value);

        let value = s.get_value();
        assert_eq!(value, test_value);
    }
}
