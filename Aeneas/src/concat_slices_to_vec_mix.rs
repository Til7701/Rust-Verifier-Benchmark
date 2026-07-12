fn concat_slices_to_vec_mix<T: Copy>(x: &[T], y: &[T]) -> Vec<T> {
    let mut concat = x.to_vec();

    let mut i = 0;
    while i < y.len() {
        concat.push(y[i]);
        i += 1;
    }

    concat
}
