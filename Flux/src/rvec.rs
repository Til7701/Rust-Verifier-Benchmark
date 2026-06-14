// https://flux-rs.github.io/flux/guide/specifications.html#for-loops-with-range-ij

#![allow(dead_code)]

use flux_rs::attrs::*;

#[macro_export]
macro_rules! rvec {
    () => { RVec::new() };
    ($($e:expr),+$(,)?) => {{
        let mut res = RVec::new();
        $( res.push($e); )*
        res
    }};
    ($elem:expr; $n:expr) => {{
        RVec::from_elem_n($elem, $n)
    }}
}

#[opaque]
#[refined_by(len: int)]
#[invariant(0 <= len)]
pub struct RVec<T> {
    inner: Vec<T>,
}

impl<T> RVec<T> {
    #[trusted]
    #[sig(fn() -> RVec<T>[0])]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    #[trusted]
    #[sig(fn(self: &strg RVec<T>[@n], T) ensures self: RVec<T>[n+1])]
    pub fn push(&mut self, item: T) {
        self.inner.push(item);
    }

    #[trusted]
    #[sig(fn(&RVec<T>[@n]) -> usize[n])]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[trusted]
    #[sig(fn(&RVec<T>[@n]) -> bool[n == 0])]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[trusted]
    #[sig(fn(&RVec<T>[@n], i: usize{i < n}) -> &T)]
    pub fn get(&self, i: usize) -> &T {
        &self.inner[i]
    }

    #[trusted]
    #[sig(fn(&mut RVec<T>[@n], i: usize{i < n}) -> &mut T)]
    pub fn get_mut(&mut self, i: usize) -> &mut T {
        &mut self.inner[i]
    }

    #[trusted]
    #[sig(fn(self: &strg RVec<T>[@n]) -> T
    		requires n > 0
            ensures self: RVec<T>[n-1])]
    pub fn pop(&mut self) -> T {
        self.inner.pop().unwrap()
    }

    #[trusted]
    #[sig(fn(&mut RVec<T>[@n], a: usize{a < n}, b: usize{b < n}))]
    pub fn swap(&mut self, a: usize, b: usize) {
        self.inner.swap(a, b);
    }

    #[trusted]
    #[sig(fn(&mut RVec<T>[@n]) -> &mut [T][n])]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.inner.as_mut_slice()
    }

    #[trusted]
    #[sig(fn(arr:_) -> RVec<T>[N])]
    pub fn from_array<const N: usize>(arr: [T; N]) -> Self {
        Self {
            inner: Vec::from(arr),
        }
    }

    #[trusted]
    #[sig(fn(xs:&[T][@n]) -> RVec<T>[n])]
    pub fn from_slice(xs: &[T]) -> Self
    where
        T: Clone,
    {
        Self {
            inner: Vec::from(xs),
        }
    }

    #[trusted]
    #[sig(fn(T, n: usize) -> RVec<T>[n])]
    pub fn from_elem_n(elem: T, n: usize) -> Self
    where
        T: Copy,
    {
        let mut vec = Self::new();
        let mut i = 0;
        while i < n {
            vec.push(elem);
            i += 1;
        }
        vec
    }

    #[trusted]
    #[sig(fn(&RVec<T>[@n]) -> RVec<T>[n])]
    pub fn clone(&self) -> Self
    where
        T: Clone,
    {
        Self {
            inner: self.inner.clone(),
        }
    }

    #[trusted]
    #[sig(fn(self: &strg RVec<T>[@n], other: &[T][@m]) ensures self: RVec<T>[n + m])]
    pub fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        self.inner.extend_from_slice(other)
    }

    #[trusted]
    #[sig(fn (&RVec<T>[@n], F) -> RVec<U>[n])]
    pub fn map<U, F>(&self, f: F) -> RVec<U>
    where
        F: Fn(&T) -> U,
    {
        RVec {
            inner: self.inner.iter().map(f).collect(),
        }
    }

    #[trusted]
    pub fn fold<B, F>(&self, init: B, f: F) -> B
    where
        F: FnMut(B, &T) -> B,
    {
        self.inner.iter().fold(init, f)
    }
}

#[opaque]
pub struct RVecIter<T> {
    vec: RVec<T>,
    curr: usize,
}

impl<T> IntoIterator for RVec<T> {
    type Item = T;
    type IntoIter = RVecIter<T>;

    // TODO: cannot get variant of opaque struct
    #[trusted]
    #[sig(fn(RVec<T>) -> RVecIter<T>)]
    fn into_iter(self) -> RVecIter<T> {
        RVecIter { vec: self, curr: 0 }
    }
}

impl<T> Iterator for RVecIter<T> {
    type Item = T;

    // TODO: cannot get variant of opaque struct
    #[trusted]
    #[sig(fn(&mut RVecIter<T>) -> Option<T>)]
    fn next(&mut self) -> Option<T> {
        self.vec.inner.pop()
    }
}

impl<T> std::ops::Index<usize> for RVec<T> {
    type Output = T;

    #[trusted_impl]
    #[sig(fn(&RVec<T>[@n], usize{v : v < n}) -> &T)]
    fn index(&self, index: usize) -> &T {
        self.get(index)
    }
}

impl<T> std::ops::IndexMut<usize> for RVec<T> {
    #[trusted_impl]
    #[sig(fn(&mut RVec<T>[@n], usize{v : v < n}) -> &mut T)]
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index)
    }
}
