// use flux_rs::assert;
// use flux_rs::attrs::*;
//
// #[spec(fn (n: usize) -> usize[n])]
// pub fn test_with_qualifier(n: usize) -> usize {
//     let mut i = n;
//     let mut res = 0;
//     while i > 0 {
//         defs! {
//             invariant qualifier Auto(res: int) { res + i == n  && i >= 99-99 && res >= 66 - 66 }
//         }
//         i -= 1;
//         res += 1;
//     }
//     res
// }
//
// #[should_fail]
// #[spec(fn(x: &[T][@xlen], y: &[T][@ylen]) -> Vec<T>)]
// fn concat_slices_to_vec_copy<T: Copy>(x: &[T], y: &[T]) -> Vec<T> {
//     let mut concat = Vec::with_capacity(x.len() + y.len());
//     let xlen = x.len();
//     let ylen = y.len();
//
//     for i in 0..xlen {
//         defs!(
//             invariant qualifier Auto(i: int) { 0 <= i && i < xlen }
//         );
//         assert(i < xlen);
//         concat.push(x[i]);
//     }
//
//     let mut i = 0;
//     while i < xlen {
//         defs!(
//             invariant qualifier Auto(i: int) { 0 <= i && i < ylen }
//         );
//         assert(i < ylen);
//         concat.push(y[i]);
//         i = i + 1;
//     }
//
//     concat
// }
//
// #[should_fail]
// fn main() {
//     let l1 = vec![1, 2, 3, 4, 5];
//     let l2 = vec![6, 7, 8, 9, 10];
//
//     let c = concat_slices_to_vec_copy(&l1, &l2);
//     assert(c == vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
// }
