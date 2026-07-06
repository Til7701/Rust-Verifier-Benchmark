mod m {

    pub struct Range {
        start: i32,
        end: i32,
    }

    #[cfg(kani)]
    impl kani::Invariant for Range {
        fn is_safe(&self) -> bool {
            self.start <= self.end
        }
    }

    impl Range {
        pub fn new(start: i32, end: i32) -> Self {
            Range { start, end }
        }

        pub fn get_start(&self) -> i32 {
            self.start
        }

        pub fn set_start(&mut self, start: i32) {
            self.start = start;
        }

        pub fn get_end(&self) -> i32 {
            self.end
        }

        pub fn set_end(&mut self, end: i32) {
            self.end = end;
        }
    }
}

#[cfg(kani)]
mod verification {
    use super::m::*;

    #[kani::proof]
    fn main() {
        let any_start: i32 = kani::any();
        let any_end = kani::any_where(|e| *e <= any_start);
        let mut range = Range::new(any_start, any_end);
        let start = range.get_start();
        assert!(start == any_start);
        let end = range.get_end();
        assert!(end == any_end);

        let any_other_start = kani::any_where(|s| *s <= any_end);
        range.set_start(any_other_start);

        let start = range.get_start();
        assert!(start == any_other_start);
        let end = range.get_end();
        assert!(end == any_end);

        let any_other_end = kani::any_where(|e| any_other_start <= *e);
        range.set_end(any_other_end);

        let start = range.get_start();
        assert!(start == any_other_start);
        let end = range.get_end();
        assert!(end == any_other_end);
    }
}
