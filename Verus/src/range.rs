use vstd::prelude::*;

verus! {

mod m {
    use vstd::prelude::*;

    pub struct Range {
        start: i32,
        end: i32,
    }

    impl Range {
        #[verifier::type_invariant]
        spec fn type_inv(self) -> bool {
            self.start <= self.end
        }

        pub closed spec fn get_start_spec(&self) -> i32 {
            self.start
        }

        pub closed spec fn get_end_spec(&self) -> i32 {
            self.end
        }

        pub fn new(start: i32, end: i32) -> (result: Self)
            requires
                start <= end,
            ensures
                result.get_start_spec() == start,
                result.get_end_spec() == end,
        {
            Range { start, end }
        }

        pub fn get_start(&self) -> (result: i32)
            ensures
                result == self.get_start_spec(),
        {
            self.start
        }

        pub fn set_start(&mut self, start: i32)
            requires
                start <= self.get_end_spec(),
            ensures
                final(self).get_start_spec() == start,
                final(self).get_end_spec() == old(self).get_end_spec(),
        {
            self.start = start;
        }

        pub fn get_end(&self) -> (result: i32)
            ensures
                result == self.get_end_spec(),
        {
            self.end
        }

        pub fn set_end(&mut self, end: i32)
            requires
                self.get_start_spec() <= end,
            ensures
                final(self).get_start_spec() == old(self).get_start_spec(),
                final(self).get_end_spec() == end,
        {
            self.end = end;
        }
    }

}

use self::m::*;

fn main() {
    let mut range = Range::new(0, 1);
    let start = range.get_start();
    assert(start == 0);
    let end = range.get_end();
    assert(end == 1);

    range.set_start(-2);

    let start = range.get_start();
    assert(start == -2);
    let end = range.get_end();
    assert(end == 1);

    range.set_end(42);

    let start = range.get_start();
    assert(start == -2);
    let end = range.get_end();
    assert(end == 42);
}

} // verus!
