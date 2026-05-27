// use vstd::prelude::*;
//
// verus! {
//
// fn remainder_rec(n: u64, d: u64) -> (result: (u64, u64))
//     requires
//         d >= 1,
//         n / d <= u64::MAX,
//     ensures
//         n == result.0 * d + result.1,
//         // result.0 == (n / d) as nat,
//         // result.1 == n % d,
//     decreases n,
// {
//     if n < d {
//         (0, n)
//     } else {
//         let (q, r) = remainder_rec(n - d, d);
//         assert(n == (q + 1) * d + r) by (nonlinear_arith);
//         (q + 1, r)
//     }
// }
//
// fn main() {
//     let r = remainder_rec(10, 3);
//     assert(r == (3u64, 1u64));
// }
//
// } // verus!
