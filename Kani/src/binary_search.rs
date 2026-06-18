// fn binary_search(v: &[u8], k: u8) -> usize {
//     let mut i1: usize = 0;
//     let mut i2: usize = v.len() - 1;
//
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
// #[cfg(kani)]
// mod verification {
//     use super::*;
//
//     #[kani::proof]
//     fn main() {
//         let k = &kani::any_where(|k| *k < 5);
//         let v: Vec<u8> = kani::bounded_any::<_, 5>();
//         kani::assume(v[0] == *k);
//         let r = binary_search(&v, *k);
//         assert![v[r] == *k];
//     }
// }
