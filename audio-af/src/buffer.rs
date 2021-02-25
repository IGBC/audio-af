use std::borrow::{Borrow, BorrowMut};
use sample::ring_buffer::{Slice, SliceMut};

use sample::Sample;

pub trait Buffer<T>: BorrowMut<[T]> where T: Sample{
    fn new(size: usize) -> Self;
    fn size(&self) -> usize;
}

pub struct TimeBuffer<T> where T: Sample {
    underlying: Box<[T]>,
}

pub struct FreqBuffer<T> where T: Sample {
    underlying: Box<[T]>,
}

pub struct ZBuffer<T> where T: Sample {
    underlying: Box<[T]>,
}


impl<T: Sample> Buffer<T> for TimeBuffer<T> {
    fn new(size: usize) -> Self {
        let reference = vec![T::equilibrium(); size].into_boxed_slice(); 
        Self {
            underlying: reference,
        }
    }

    fn size(&self) -> usize {
        self.underlying.len()
    }
}

impl<T: Sample> Borrow<[T]> for TimeBuffer<T> {
    fn borrow(&self) -> &[T] {
        self.underlying.slice()
   }
}

impl<T: Sample> BorrowMut<[T]> for TimeBuffer<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.underlying.slice_mut()
    }
}


impl<T: Sample> Buffer<T> for FreqBuffer<T> {
    fn new(size: usize) -> Self {
        let reference = vec![T::equilibrium(); size].into_boxed_slice(); 
        Self {
            underlying: reference,
        }
    }

    fn size(&self) -> usize {
        self.underlying.len()
    }
}

impl<T: Sample> Borrow<[T]> for FreqBuffer<T> {
    fn borrow(&self) -> &[T] {
        self.underlying.slice()
   }
}

impl<T: Sample> BorrowMut<[T]> for FreqBuffer<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.underlying.slice_mut()
    }
}


impl<T: Sample> Buffer<T> for ZBuffer<T> {
    fn new(size: usize) -> Self {
        let reference = vec![T::equilibrium(); size].into_boxed_slice(); 
        Self {
            underlying: reference,
        }
    }

    fn size(&self) -> usize {
        self.underlying.len()
    }
}

impl<T: Sample> Borrow<[T]> for ZBuffer<T> {
    fn borrow(&self) -> &[T] {
        self.underlying.slice()
   }
}

impl<T: Sample> BorrowMut<[T]> for ZBuffer<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.underlying.slice_mut()
    }
}