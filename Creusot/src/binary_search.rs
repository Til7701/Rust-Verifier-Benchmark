// use creusot_std::prelude::*;
//
// #[requires(1 <= v@.len() && v@.len() <= usize::MAX@)]
// #[requires(v@.sorted())] // https://doc.creusot.rs/creusot_std/logic/seq/struct.Seq.html#method.sorted
// #[requires(v@.contains(k))] // https://doc.creusot.rs/creusot_std/logic/seq/struct.Seq.html#method.contains
// #[ensures(result@ < v@.len())]
// #[ensures(k == v@[result@])]
// fn binary_search(v: &[u64], k: u64) -> usize {
//     let mut i1: usize = 0;
//     let mut i2: usize = v.len() - 1;
//     #[variant(i2 - i1)]
//     #[invariant(i1@ >= 0 && i1 <= i2 && i2@ < v@.len())]
//     #[invariant(v@.subsequence(i1@, i2@ + 1).contains(k))] // https://doc.creusot.rs/creusot_std/logic/seq/struct.Seq.html#method.subsequence
//     #[invariant(v@.sorted())]
//     #[invariant(forall<i: usize> i < i1 ==> v[i@] <= k)]
//     #[invariant(forall<i: usize> i2@ < i@ && i@ < v@.len() ==> k < v[i@])]
//     while i1 != i2 {
//         let ix = i1 + (i2 - i1) / 2;
//         if v[ix] < k {
//             i1 = ix + 1;
//         } else {
//             i2 = ix;
//         }
//     }
//     i1
// }
//
// fn main() {
//     let v = creusot_std::prelude::vec![0, 10, 24, 30, 31, 543];
//     let r = binary_search(&v, 30);
//     proof_assert!(r == 3usize);
// }
