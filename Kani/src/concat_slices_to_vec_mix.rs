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
    }
}
