use vstd::prelude::*;

verus! {

fn triangle_rec(n: u64) -> (result: u64)
    requires
        (n * (n + 1)) / 2 <= u64::MAX,
    ensures
        result == (n * (n + 1)) / 2,
    decreases n,
    no_unwind
{
    if n == 0 {
        0
    } else {
        assert((((n - 1) * n) / 2) + n == (n * (n + 1)) / 2) by (nonlinear_arith);
        n + triangle_rec(n - 1)
    }
}

fn main() {
    let r = triangle_rec(10);
    assert(r == 55);
}

} // verus!
