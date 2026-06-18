use crate::rvec::RVec;
use flux_rs::assert;
use flux_rs::attrs::*;

#[spec(fn(x: &RVec<T>[@xlen], y: &RVec<T>[@ylen]) -> RVec<T>[xlen + ylen])]
fn concat_slices_to_vec_clone<T: Clone>(x: &RVec<T>, y: &RVec<T>) -> RVec<T> {
    let mut concat = RVec::new();

    let xlen = x.len();
    let ylen = y.len();

    let mut i = 0;
    while i < xlen {
        concat.push(x[i].clone());
        i += 1;
    }

    let mut j = 0;
    while j < ylen {
        concat.push(y[j].clone());
        j += 1;
    }
    concat
}

fn main() {
    let l1 = RVec::from_slice(&vec![1, 2, 3, 4, 5]);
    let l2 = RVec::from_slice(&vec![6, 7, 8, 9, 10]);

    let c = concat_slices_to_vec_clone(&l1, &l2);

    assert(c.len() == l1.len() + l2.len());
}

// To show what can not be asserted.
#[should_fail]
fn fails() {
    let l1 = RVec::from_slice(&vec![1, 2, 3, 4, 5]);
    let l2 = RVec::from_slice(&vec![6, 7, 8, 9, 10]);

    let c = concat_slices_to_vec_clone(&l1, &l2);

    assert(c[0] == l1[0]);
}
