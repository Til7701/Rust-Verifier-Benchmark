// use flux_rs::assert;
// use flux_rs::attrs::*;

// I cannot define a postcondition that uses both elements of the returned tuple
// #[spec(fn(n: u64, d: u64{d >= 1}) -> (u64{r0: r0 <= n - d}, u64{r1: r1 < d}))]
// fn remainder_rec(n: u64, d: u64) -> (u64, u64) {
//     if n < d {
//         (0, n)
//     } else {
//         let (q, r) = remainder_rec(n - d, d);
//         (q + 1, r)
//     }
// }
//
// fn main() {
//     assert(remainder_rec(10, 3) == (3, 1));
// }
