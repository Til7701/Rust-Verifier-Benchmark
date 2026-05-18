use vstd::prelude::*;

verus! {

spec fn fib_rec_spec(n: nat) -> nat
    decreases n,
{
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib_rec_spec((n - 1) as nat) + fib_rec_spec((n - 2) as nat)
    }
}

fn fib_rec(n: u64) -> (result: u64)
    requires
        fib_rec_spec(n as nat) <= u64::MAX,
    ensures
        result == fib_rec_spec(n as nat),
    decreases n,
{
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib_rec(n - 1) + fib_rec(n - 2)
    }
}

fn main() {
    proof {
        reveal_with_fuel(fib_rec_spec, 11);
    }
    let r = fib_rec(10);
    assert(r == 55);
}

} // verus!
