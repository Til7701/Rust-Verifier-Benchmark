use creusot_std::prelude::*;

#[requires(x@.len() + y@.len() <= usize::MAX@)]
#[ensures(result@.len() == x@.len() + y@.len())]
#[ensures(forall<i> 0 <= i && i < x@.len() ==> T::clone.postcondition((&x@[i],), result@[i]))]
#[ensures(forall<i> 0 <= i && i < y@.len() ==> y@[i] == result@[x@.len() + i])]
fn concat_slices_to_vec_mix<T: Copy>(x: &[T], y: &[T]) -> Vec<T> {
    let mut concat = x.to_vec();

    #[invariant(concat@.len() == x@.len() + (*produced).len())]
    #[invariant(forall<i> 0 <= i && i < x@.len() ==> T::clone.postcondition((&x@[i],), concat@[i]))]
    #[invariant(forall<i> 0 <= i && i < (*produced).len() ==> y@[i] == concat@[x@.len() + i])]
    for e in y.iter() {
        concat.push(*e);
    }

    concat
}
