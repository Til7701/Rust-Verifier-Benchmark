use crate::rvec::RVec;
use flux_rs::*;

#[spec(fn(x: &[T][@xlen], y: &[T][@ylen]) -> RVec<T>[xlen + ylen])]
fn concat_slices_to_vec_std<T: Clone>(x: &[T], y: &[T]) -> RVec<T> {
    let mut concat = RVec::new();
    concat.extend_from_slice(x);
    concat.extend_from_slice(y);
    concat
}

fn main() {
    let mut l1 = RVec::from_slice(&vec![1, 2, 3, 4, 5]);
    let mut l2 = RVec::from_slice(&vec![6, 7, 8, 9, 10]);

    let c = concat_slices_to_vec_std(l1.as_mut_slice(), l2.as_mut_slice());

    assert(c.len() == l1.len() + l2.len());
}

// To show what can not be asserted.
#[should_fail]
fn fails() {
    let mut l1 = RVec::from_slice(&vec![1, 2, 3, 4, 5]);
    let mut l2 = RVec::from_slice(&vec![6, 7, 8, 9, 10]);

    let c = concat_slices_to_vec_std(l1.as_mut_slice(), l2.as_mut_slice());

    assert(c[0] == l1[0]);
}
