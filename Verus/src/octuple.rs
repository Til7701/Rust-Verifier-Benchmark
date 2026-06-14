use vstd::prelude::*;

verus! {

fn octuple(x1: i8) -> (x8: i8)
    requires
        -16 <= x1,
        x1 < 16,
    ensures
        x8 == 8 * x1,
    no_unwind
{
    let x2 = x1 + x1;
    let x4 = x2 + x2;
    x4 + x4
}

fn main() {
    let r = octuple(3);
    assert(r == 24);
}

} // verus!
