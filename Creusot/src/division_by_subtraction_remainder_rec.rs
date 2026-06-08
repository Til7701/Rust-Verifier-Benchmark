use creusot_std::prelude::*;

#[check(terminates)]
#[variant(n)]
#[requires(d@ >= 1)]
#[ensures(result.0@ == n@ / d@)]
#[ensures(result.1@ == n@ % d@)]
fn remainder_rec(n: u64, d: u64) -> (u64, u64) {
    if n < d {
        (0, n)
    } else {
        let (q, r) = remainder_rec(n - d, d);
        proof_assert!(n == (q + 1u64) * d + r);
        (q + 1, r)
    }
}

fn main() {
    let r = remainder_rec(10, 3);
    proof_assert!(r == (3u64, 1u64));
}
