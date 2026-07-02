fn concat_slices_to_vec_copy<T: Copy>(x: &[T], y: &[T]) -> Vec<T> {
    let mut concat = Vec::with_capacity(x.len() + y.len());
    for i in 0..x.len() {
        concat.push(x[i]);
    }
    for i in 0..y.len() {
        concat.push(y[i]);
    }
    concat
}
