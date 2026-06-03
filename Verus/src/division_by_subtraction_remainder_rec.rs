// use vstd::prelude::*;
//
// verus! {
//
// fn remainder_rec(n: u64, d: u64) -> (result: (u64, u64))
//     requires
//         d >= 1,
//         remainder_rec_spec(n as int, d as nat).0 <= u64::MAX,
//     ensures
//         n == result.0 * d + result.1,
//         result.1 < d,
//         result.0 == remainder_rec_spec(n as int, d as nat).0,
//         result.1 == remainder_rec_spec(n as int, d as nat).1,
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
// spec fn remainder_rec_spec(n: int, d: nat) -> (result: (int, int))
//     decreases n,
// {
//     if n < d {
//         (0, n)
//     } else {
//         let (q, r) = remainder_rec_spec(n - d, d);
//         (q + 1, r)
//     }
// }
//
// fn main() {
//     proof {
//         reveal_with_fuel(remainder_rec_spec, 11);
//     }
//     let r = remainder_rec(10, 3);
//     assert(r == (3u64, 1u64));
// }
//
// } // verus!
