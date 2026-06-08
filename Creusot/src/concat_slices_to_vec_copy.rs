use creusot_std::prelude::*;

#[requires(x@.len() + y@.len() <= usize::MAX@)]
#[ensures(x@.concat(y@) == result@)]
fn concat_slices_to_vec_copy<T: Copy>(x: &[T], y: &[T]) -> Vec<T> {
    let mut concat: Vec<T> = Vec::with_capacity(x.len() + y.len());

    #[invariant(concat@ == produced.inner().to_owned_seq())]
    for e in x.iter() {
        concat.push(*e);
    }

    #[invariant(concat@ == x@.concat(produced.inner().to_owned_seq()))]
    for e in y.iter() {
        concat.push(*e);
    }
    concat
}

fn main() {
    let l1 = creusot_std::prelude::vec![1, 2, 3, 4, 5];
    let l2 = creusot_std::prelude::vec![6, 7, 8, 9, 10];

    let c = concat_slices_to_vec_copy(&l1, &l2);
    proof_assert!(c@ == seq![1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32, 8i32, 9i32, 10i32]);
}
