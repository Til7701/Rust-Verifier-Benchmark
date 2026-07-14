// Takes 30GB of memory
#[cfg(kani)]
fn concat_slices_to_vec_mix_loop<T: Copy>(x: &[T], y: &[T]) -> Vec<T> {
    let mut concat = x.to_vec();

    #[kani::loop_invariant(concat.len() == x.len() + kani::index)]
    #[kani::loop_invariant(kani::forall!(|i in (0,x.len())| concat[i] == x[i]))]
    #[kani::loop_invariant(kani::forall!(|i in (0,kani::index)| concat[x.len() + i] == y[i]))]
    #[kani::loop_modifies(&concat)]
    #[kani::loop_decreases(y.len() - kani::index)]
    for e in y {
        concat.push(e);
    }

    concat
}

fn concat_slices_to_vec_mix<T: Copy>(x: &[T], y: &[T]) -> Vec<T> {
    let mut concat = x.to_vec();
    concat.extend_from_slice(y);

    concat
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn main() {
        let x: Vec<bool> = kani::bounded_any::<_, 2>();
        let y: Vec<bool> = kani::bounded_any::<_, 2>();

        let concat = concat_slices_to_vec_mix(&x, &y);

        assert_eq!(concat.len(), x.len() + y.len());
        if x.len() > 0 {
            assert_eq!(concat[0], x[0]);
        }
    }
}
