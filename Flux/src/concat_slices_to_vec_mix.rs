use crate::rvec::RVec;
use flux_rs::attrs::*;

#[spec(fn(x: &RVec<T>[@xlen], y: &RVec<T>[@ylen]) -> RVec<T>[xlen + ylen])]
fn concat_slices_to_vec_mix<T: Copy>(x: &RVec<T>, y: &RVec<T>) -> RVec<T> {
    let mut concat = x.clone();
    let ylen = y.len();

    let mut i = 0;
    while i < ylen {
        concat.push(y[i]);
        i += 1;
    }

    concat
}
