use vstd::prelude::*;

verus! {

mod m {
    pub struct S {
        value: i32,
    }

    impl S {
        pub closed spec fn get_value_spec(&self) -> i32 {
            self.value
        }

        pub fn new() -> (result: Self)
            ensures
                result.get_value_spec() == 0,
        {
            S { value: 0 }
        }

        pub fn get_value(&self) -> (result: i32)
            ensures
                result == self.get_value_spec(),
        {
            self.value
        }

        pub fn set_value(&mut self, v: i32)
            ensures
                final(self).get_value_spec() == v,
        {
            self.value = v;
        }
    }

}

use self::m::*;

fn main() {
    let mut s = S::new();

    let value = s.get_value();
    assert(value == 0);

    s.set_value(5);

    let value = s.get_value();
    assert(value == 5);
}

} // verus!
