use creusot_std::prelude::*;

#[ensures((n@ % 2 == 0) == result)]
fn is_even(n: u64) -> bool {
    if n == 0 {
        return true;
    }
    is_odd(n - 1)
}

#[ensures((n@ % 2 == 1) == result)]
fn is_odd(n: u64) -> bool {
    if n == 0 {
        return false;
    }
    is_even(n - 1)
}

fn main() {
    let e1 = is_even(5);
    proof_assert!(!e1);
    let e2 = is_even(42);
    proof_assert!(e2);

    let o1 = is_odd(5);
    proof_assert!(o1);
    let o2 = is_odd(42);
    proof_assert!(!o2);
}
