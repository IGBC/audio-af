use std::borrow::{Borrow, BorrowMut};
use sample::ring_buffer::{Slice, SliceMut};

use sample::Sample;

/// A Trait for Zero-Copy Buffers. Buffers implemented using this type can be 
/// moved freely without invoking a copy of the underlying data. The borrow
/// checker will enforce that only one user can edit the underlying data at once
/// These buffers however don't implement the copy trait.  
pub trait Buffer<T>: BorrowMut<[T]> where T: Sample{
    fn new(size: usize) -> Self;
    fn size(&self) -> usize;
}

/// Buffer implementation that represents a samples in the time domain.
pub struct TimeBuffer<T> where T: Sample {
    underlying: Box<[T]>,
}

/// Buffer implementation that represents a set of frequency bins of a signal having passed through an FFT.
pub struct FreqBuffer<T> where T: Sample {
    underlying: Box<[T]>,
}

/// Buffer implementation that represents a set of Z Bins from a sigal having passed through a Z transform.
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