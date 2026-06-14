// use crate::rvec::RVec;
// use flux_rs::{assert, spec};
//
// #[spec(fn(x: &[T][@xlen], y: &[T][@ylen]) -> RVec<T>[xlen + ylen])]
// fn concat_slices_to_vec_std<T: Clone>(x: &[T], y: &[T]) -> RVec<T> {
//     let mut concat = RVec::new();
//     concat.extend_from_slice(x);
//     concat.extend_from_slice(y);
//     concat
// }
//
// fn main() {
//     let l1 = vec![1, 2, 3, 4, 5];
//     let l2 = vec![6, 7, 8, 9, 10];
//
//     let c = concat_slices_to_vec_std(&l1, &l2);
//     assert(c.len() == 10);
// }
