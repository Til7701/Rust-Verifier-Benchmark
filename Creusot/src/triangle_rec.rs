use creusot_std::prelude::*;

#[check(terminates)]
#[variant(n)]
#[requires((n@ * (n@ + 1)) / 2 <= u64::MAX@)]
#[ensures(result@ == (n@ * (n@ + 1)) / 2)]
fn triangle_rec(n: u64) -> u64 {
    if n == 0 { 0 } else { n + triangle_rec(n - 1) }
}

fn main() {
    let r = triangle_rec(10);
    proof_assert!(r == 55u64);
}
