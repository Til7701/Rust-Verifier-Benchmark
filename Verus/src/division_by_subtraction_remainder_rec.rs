// use vstd::prelude::*;
//
// verus! {
//
// // Verus cannot prove termination since I cannot add a precondition
// spec fn remainder_rec_spec(n: nat, d: nat) -> (nat, nat)
//     recommends
//         d >= 1,
//     decreases n,
// {
//     if n < d {
//         (0, n)
//     } else {
//         let (q, r) = remainder_rec_spec((n - d) as nat, d);
//         (q + 1, r)
//     }
// }
//
// proof fn lemma_remainder_rec(n: nat, d: nat)
//     requires
//         d >= 1,
//     ensures
//         n == remainder_rec_spec(n, d).0 * d + remainder_rec_spec(n, d).1,
//         remainder_rec_spec(n, d).1 < d,
// {
// }
//
// fn remainder_rec(n: u64, d: u64) -> (result: (u64, u64))
//     requires
//         d >= 1,
//         remainder_rec_spec(n as nat, d as nat).0 <= u64::MAX,
//         remainder_rec_spec(n as nat, d as nat).1 <= u64::MAX,
//     ensures
//         result.0 == remainder_rec_spec(n as nat, d as nat).0,
//         result.1 == remainder_rec_spec(n as nat, d as nat).1,
//     decreases n,
// {
//     if n < d {
//         (0, n)
//     } else {
//         let (q, r) = remainder_rec(n - d, d);
//         (q + 1, r)
//     }
// }
//
// fn main() {
//     proof {
//         lemma_remainder_rec(10, 3);
//     }
//     let r = remainder_rec(10, 3);
//     assert(r == (3u64, 1u64));
// }
//
// } // verus!
