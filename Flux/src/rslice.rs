// https://github.com/flux-rs/flux/blob/009f708f63649f2b0af5cc3f9e6792384cf8eed6/tests/tests/lib/rslice.rs

use crate::rvec::RVec;
use flux_rs::attrs::*;
use std::marker::PhantomData;

#[opaque]
#[refined_by(len: int, valid: (int, int) -> bool)]
pub struct RSlice<'a, T> {
    data: *mut T,
    len: usize,
    _marker: PhantomData<&'a mut [T]>,
}

impl<'a, T> RSlice<'a, T> {
    #[trusted]
    #[sig(fn(vec: &mut RVec<T>[@n]) -> RSlice<T>[n, |i,j| true])]
    pub fn from_vec(vec: &mut RVec<T>) -> RSlice<'_, T> {
        RSlice::from_slice(vec.as_mut_slice())
    }

    #[trusted]
    #[sig(fn(slice: &mut [T][@n]) -> RSlice<T>[n, |i,j| true])]
    pub fn from_slice(slice: &mut [T]) -> RSlice<'_, T> {
        RSlice {
            data: slice.as_mut_ptr(),
            len: slice.len(),
            _marker: PhantomData,
        }
    }

    #[trusted]
    #[sig(
        fn(self: &strg RSlice<T>[@len, @valid], left: usize, right: usize) -> RSlice<T>[right - left + 1, |i,j| true]
        requires left <= right && right < len && valid(left, right)
        ensures self: RSlice<T>[len, |i, j| valid(i, j) && (right < i || j < left)]
    )]
    pub fn subslice(&mut self, left: usize, right: usize) -> RSlice<'a, T> {
        unsafe {
            RSlice {
                data: self.data.add(left),
                len: right - left + 1,
                _marker: PhantomData,
            }
        }
    }

    #[trusted]
    #[sig(fn(&mut RSlice<T>[@n, |i,j| true]) -> &mut [T][n])]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.data, self.len) }
    }

    #[trusted]
    #[sig(fn(&RSlice<T>[@n, |i,j| true]) -> &[T][n])]
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data, self.len) }
    }
}
