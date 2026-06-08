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
