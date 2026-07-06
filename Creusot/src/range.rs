mod m {
    use creusot_std::prelude::*;

    pub struct Range {
        start: i32,
        end: i32,
    }

    impl Invariant for Range {
        #[logic]
        fn invariant(self) -> bool {
            pearlite! {
                self.start <= self.end
            }
        }
    }

    impl View for Range {
        type ViewTy = (i32, i32);

        #[logic]
        fn view(self) -> Self::ViewTy {
            (self.start, self.end)
        }
    }

    impl Range {
        #[requires(start <= end)]
        #[ensures(result@ == (start, end))]
        pub fn new(start: i32, end: i32) -> Self {
            Range { start, end }
        }

        #[ensures(result == self@.0)]
        pub fn get_start(&self) -> i32 {
            self.start
        }

        #[requires(start <= self@.1)]
        #[ensures((^self)@ == (start, self@.1))]
        pub fn set_start(&mut self, start: i32) {
            self.start = start;
        }

        #[ensures(result == self@.1)]
        pub fn get_end(&self) -> i32 {
            self.end
        }

        #[requires(self@.0 <= end)]
        #[ensures((^self)@ == (self@.0, end))]
        pub fn set_end(&mut self, end: i32) {
            self.end = end;
        }
    }
}

use creusot_std::prelude::proof_assert;
use m::*;

fn main() {
    let mut range = Range::new(0, 1);
    let start = range.get_start();
    proof_assert!(start == 0i32);
    let end = range.get_end();
    proof_assert!(end == 1i32);

    range.set_start(-2);

    let start = range.get_start();
    proof_assert!(start == -2i32);
    let end = range.get_end();
    proof_assert!(end == 1i32);

    range.set_end(42);

    let start = range.get_start();
    proof_assert!(start == -2i32);
    let end = range.get_end();
    proof_assert!(end == 42i32);
}
