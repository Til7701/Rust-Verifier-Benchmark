use vstd::prelude::*;

verus! {

spec fn triangle_rec_spec(n: nat) -> nat
    decreases n,
{
    if n == 0 {
        0
    } else {
        n + triangle_rec_spec((n - 1) as nat)
    }
}

fn triangle_rec(n: u64) -> (result: u64)
    requires
        triangle_rec_spec(n as nat) <= u64::MAX,
    ensures
        result == triangle_rec_spec(n as nat),
    decreases n,
{
    if n == 0 {
        0
    } else {
        n + triangle_rec(n - 1)
    }
}

fn main() {
    proof {
        reveal_with_fuel(triangle_rec_spec, 11);
    }
    let r = triangle_rec(10);
    assert(r == 55);
}

} // verus!
