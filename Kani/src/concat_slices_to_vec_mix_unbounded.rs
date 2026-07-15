#[cfg(kani)]
fn concat_slices_to_vec_mix<T: Copy>(x: &[T], y: &[T]) -> Vec<T> {
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

#[cfg(kani)]
mod verification {
    use super::*;

    // Takes 30GB of memory
    //#[kani::proof]
    fn main() {
        let x: Vec<bool> = kani::bounded_any::<_, 2>();
        let y: Vec<bool> = kani::bounded_any::<_, 2>();

        let concat = concat_slices_to_vec_mix(&x, &y);

        assert_eq!(concat.len(), x.len() + y.len());
        #[kani::loop_invariant(i < x.len())]
        #[kani::loop_decreases(x.len() - i)]
        for i in 0..x.len() {
            assert_eq!(concat[i], x[i]);
        }
        #[kani::loop_invariant(i < y.len())]
        #[kani::loop_decreases(y.len() - i)]
        for i in 0..y.len() {
            assert_eq!(concat[x.len() + i], y[i]);
        }
    }
}
