mod m {
    use flux_rs::{sig, spec};

    #[flux_rs::refined_by(start: int, end: int)]
    #[flux_rs::invariant(start <= end)]
    pub struct Range {
        #[field(i32[start])]
        start: i32,
        #[field(i32[end])]
        end: i32,
    }

    impl Range {
        #[spec(fn(start: i32, end: i32{start <= end}) -> Range[{start: start, end: end}])]
        pub fn new(start: i32, end: i32) -> Self {
            Range { start, end }
        }

        #[spec(fn(&Self[@r]) -> i32{result: result == r.start})]
        pub fn get_start(&self) -> i32 {
            self.start
        }

        #[sig(fn(&mut Self[@r], start: i32{start <= r.end}) ensures r.start == start)]
        pub fn set_start(&mut self, start: i32) {
            self.start = start;
        }

        #[spec(fn(&Self[@r]) -> i32{result: result == r.end})]
        pub fn get_end(&self) -> i32 {
            self.end
        }
    }
}

use m::*;

fn main() {
    let mut range = Range::new(0, 1);
    let start = range.get_start();
    assert_eq!(start, 0);
    let end = range.get_end();
    assert_eq!(end, 1);

    range.set_start(-2);

    let start = range.get_start();
    assert_eq!(start, -2);
    let end = range.get_end();
    assert_eq!(end, 1);
}
