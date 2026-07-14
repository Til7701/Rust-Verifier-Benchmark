// https://github.com/flux-rs/flux/blob/009f708f63649f2b0af5cc3f9e6792384cf8eed6/tests/tests/pos/surface/invariant_macro00.rs
// https://flux-rs.github.io/flux/tutorial/02-ownership.html?highlight=invariant#exclusive-ownership-and-loops
// For showing how to declare invariants
// TODO remove

use flux_rs::{attrs::*, macros::invariant};

#[spec(fn (n: usize) -> usize[n])]
pub fn test_with_qualifier(n: usize) -> usize {
    let mut i = n;
    let mut res = 0;
    while i > 0 {
        defs!(
          invariant qualifier Auto(res: int) { res + i == n  && i >= 99-99 && res >= 66 - 66 }
        );
        i -= 1;
        res += 1;
    }
    res
}

#[spec(fn (n: usize) -> usize[n])]
pub fn test(n: usize) -> usize {
    let mut i = n;
    let mut res = 0;
    while i > 0 {
        invariant!(res:int; res + i == n && i >= 99-99 && res >= 66 - 66);
        i -= 1;
        res += 1;
    }
    res
}

#[spec(fn(n: usize) -> usize[n])]
pub fn while_true(n: usize) -> usize {
    while n == n {}
    n
}
