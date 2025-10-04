#![allow(unused)] // Data structure need not be completely used

use crate::error::AppError;
use std::fmt::{Display, Formatter};
use std::io::IoSliceMut;
use std::mem::MaybeUninit;
use std::ops::Index;
use std::{mem, slice};

pub struct RingBuffer<T, const N: usize>
where
    T: Copy,
{
    buffer: [MaybeUninit<T>; N],
    read_pos: usize,
    write_pos: usize,
    empty: bool,
}

impl<T, const N: usize> RingBuffer<T, N>
where
    T: Copy,
{
    pub const fn new() -> Self {
        Self {
            buffer: [MaybeUninit::<T>::uninit(); N],
            read_pos: 0,
            write_pos: 0,
            empty: true,
        }
    }

    pub const fn capacity(&self) -> usize {
        N
    }

    pub const fn used_space(&self) -> usize {
        Self::used_space_internal(self.empty, self.read_pos, self.write_pos)
    }

    const fn used_space_internal(empty: bool, read_pos: usize, write_pos: usize) -> usize {
        if empty {
            0
        } else if write_pos > read_pos {
            write_pos - read_pos
        } else if write_pos < read_pos {
            N - (read_pos - write_pos)
        } else {
            N
        }
    }

    pub const fn available_space(&self) -> usize {
        N - self.used_space()
    }

    pub const fn is_empty(&self) -> bool {
        self.empty
    }

    pub const fn is_full(&self) -> bool {
        self.used_space() == N
    }

    pub fn push(&mut self, slice: &[T]) -> Result<(), AppError> {
        if slice.len() > self.available_space() {
            return Err(AppError::new("Not enough space in the buffer"));
        }

        let slice_maybe: &[MaybeUninit<T>] = unsafe { mem::transmute(slice) };

        if (self.write_pos + slice.len()) < N {
            let target: &mut [MaybeUninit<T>] =
                &mut self.buffer[self.write_pos..(self.write_pos + slice.len())];
            target.copy_from_slice(slice_maybe);
        } else {
            let a = &mut self.buffer[self.write_pos..N];
            let a_len = a.len();
            a.copy_from_slice(&slice_maybe[0..a_len]);

            let b = &mut self.buffer[0..(slice.len() - a_len)];
            let b_len = b.len();
            b.copy_from_slice(&slice_maybe[0..b_len]);

            assert_eq!(slice.len(), a_len + b_len);
        }

        self.advance(slice.len());
        Ok(())
    }

    pub fn advance(&mut self, count: usize) -> Result<(), AppError> {
        if count > self.available_space() {
            return Err(AppError::new("Not enough space in the buffer"));
        }

        self.write_pos = (self.write_pos + count) % N;
        self.empty = false;
        Ok(())
    }

    pub fn peek<'a>(&'a self, count: usize) -> Result<RingBufferView<'a, T>, AppError> {
        Self::peek_internal(
            &self.buffer,
            self.empty,
            self.read_pos,
            self.write_pos,
            count,
        )
    }

    fn peek_internal<'a>(
        buffer: &'a [MaybeUninit<T>],
        empty: bool,
        read_pos: usize,
        write_pos: usize,
        count: usize,
    ) -> Result<RingBufferView<'a, T>, AppError> {
        if count > Self::used_space_internal(empty, read_pos, write_pos) {
            return Err(AppError::new("Not enough content in the buffer"));
        }

        let mut view: RingBufferView<'a, T> = RingBufferView {
            first: &[],
            second: &[],
        };

        if count == 0 {
            return Ok(view);
        }

        if read_pos + count < N {
            view.first = unsafe { mem::transmute(&buffer[read_pos..(read_pos + count)]) };
        } else {
            view.first = unsafe { mem::transmute(&buffer[read_pos..N]) };
            view.second = unsafe { mem::transmute(&buffer[0..(count - view.first.len())]) };

            assert_eq!(count, view.first.len() + view.second.len());
        }

        Ok(view)
    }

    pub fn pop<'a>(&'a mut self, count: usize) -> Result<RingBufferView<'a, T>, AppError> {
        let view: RingBufferView<T> = Self::peek_internal(
            &self.buffer,
            self.empty,
            self.read_pos,
            self.write_pos,
            count,
        )?;

        self.read_pos = (self.read_pos + count) % N;
        if self.read_pos == self.write_pos {
            self.empty = true;
        }
        Ok(view)
    }

    pub fn current_empty_slices_mut<'a>(&'a mut self) -> [&'a mut [MaybeUninit<T>]; 2] {
        let available_space: usize = self.available_space();
        let slices: [&'a mut [MaybeUninit<T>]; 2] = {
            if self.write_pos < self.read_pos {
                [&mut self.buffer[self.write_pos..self.read_pos], &mut []]
            } else if self.write_pos > self.read_pos || self.empty {
                let split: (&mut [MaybeUninit<T>], &mut [MaybeUninit<T>]) =
                    self.buffer.split_at_mut(self.write_pos);
                [split.1, &mut split.0[0..self.read_pos]]
            } else {
                [&mut [], &mut []]
            }
        };

        assert_eq!(available_space, slices[0].len() + slices[1].len());
        slices
    }

    /// For use with [std::io::Read::read_vectored]
    /// # Safety
    /// In order to convert to [IoSliceMut], [T] must be cast to [u8].
    /// Any data input via this returned mutable slice will be entered as raw bytes.
    /// Thus, usage of this function's return value is unsafe.
    pub unsafe fn current_empty_slices_as_io_slice_mut<'a>(&'a mut self) -> [IoSliceMut<'a>; 2] {
        let mut slices = self.current_empty_slices_mut();
        [
            IoSliceMut::new(unsafe {
                slice::from_raw_parts_mut(
                    slices[0].as_mut_ptr() as *mut u8,
                    slices[0].len() * size_of::<T>(),
                )
            }),
            IoSliceMut::new(unsafe {
                slice::from_raw_parts_mut(
                    slices[1].as_mut_ptr() as *mut u8,
                    slices[1].len() * size_of::<T>(),
                )
            }),
        ]
    }
}

impl<T: Copy, const N: usize> Display for RingBuffer<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RingBuffer; [capacity: {}] [used: {}]",
            self.capacity(),
            self.used_space()
        )
    }
}

pub struct RingBufferView<'a, T>
where
    T: Copy,
{
    pub first: &'a [T],
    pub second: &'a [T],
}

impl<'a, T> RingBufferView<'a, T>
where
    T: Copy,
{
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.first.iter().chain(self.second.iter())
    }

    pub fn len(&self) -> usize {
        self.first.len() + self.second.len()
    }

    pub fn copy_to(&self, dest: &mut [T]) {
        dest[..self.first.len()].copy_from_slice(self.first);
        dest[self.first.len()..].copy_from_slice(self.second);
    }

    pub fn as_slices(&self) -> (&[T], &[T]) {
        (self.first, self.second)
    }
}

impl<'a, T> From<RingBufferView<'a, T>> for Vec<T>
where
    T: Copy,
{
    fn from(view: RingBufferView<'a, T>) -> Self {
        let mut target: Vec<T> = Vec::with_capacity(view.len());
        target.extend_from_slice(&view.first);
        target.extend_from_slice(&view.second);
        target
    }
}

impl<'a, T> Index<usize> for RingBufferView<'a, T>
where
    T: Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.first.len() {
            self.first.index(index)
        } else if index < self.first.len() + self.second.len() {
            self.second.index(index - self.first.len())
        } else {
            panic!("Index out of bounds");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod ring_buffer {
        use super::*;

        #[test]
        fn push() {
            let mut ring_buffer: RingBuffer<u8, 32> = RingBuffer::<u8, 32>::new();
            assert!(ring_buffer.is_empty());

            let hello_world = b"hello world";
            let result = ring_buffer.push(hello_world);
            assert!(result.is_ok());

            let numbers = &[0, 1, 2];
            let result = ring_buffer.push(&[0, 1, 2]);
            assert!(result.is_ok());

            let result = ring_buffer.push(&vec![1; ring_buffer.capacity()]);
            assert_eq!(true, result.is_err());

            assert!(!ring_buffer.is_empty());
            assert_eq!(hello_world.len() + numbers.len(), ring_buffer.used_space());

            ring_buffer
                .push(&vec![1; ring_buffer.capacity() - ring_buffer.used_space()])
                .unwrap();
            assert!(ring_buffer.is_full());
            assert_eq!(b'e', unsafe { ring_buffer.buffer[1].assume_init() });
            assert_eq!(2, unsafe {
                ring_buffer.buffer[hello_world.len() + 2].assume_init()
            });
            assert_eq!(1, unsafe {
                ring_buffer.buffer[hello_world.len() + numbers.len()].assume_init()
            });
        }

        #[test]
        fn peek() {
            let mut ring_buffer: RingBuffer<u8, 32> = RingBuffer::<u8, 32>::new();
            let hello_world = b"hello world";
            ring_buffer.push(hello_world).unwrap();

            let view = ring_buffer.peek(2).unwrap();
            assert_eq!(2, view.len());
            assert_eq!(b'h', view[0]);
            assert_eq!(b'e', view[1]);

            let view = ring_buffer.peek(2).unwrap();
            assert_eq!(2, view.len());
            assert_eq!(b'h', view[0]);
            assert_eq!(b'e', view[1]);
        }

        #[test]
        fn pop() {
            let mut ring_buffer: RingBuffer<u8, 32> = RingBuffer::<u8, 32>::new();
            let hello_world = b"hello world";
            ring_buffer.push(hello_world).unwrap();

            let view = ring_buffer.pop(2).unwrap();
            assert_eq!(2, view.len());
            assert_eq!(b"h"[0], view[0]);
            assert_eq!(b"e"[0], view[1]);

            let view = ring_buffer.pop(3).unwrap();
            assert_eq!(3, view.len());
            assert_eq!(b'l', view[0]);
            assert_eq!(b'l', view[1]);
            assert_eq!(b'o', view[2]);
        }

        #[test]
        fn wrap() {
            let mut ring_buffer: RingBuffer<u8, 4> = RingBuffer::<u8, 4>::new();
            ring_buffer.push(&[1, 2, 3, 4]).unwrap();

            assert_eq!(4, ring_buffer.capacity());
            assert!(ring_buffer.is_full());
            assert!(ring_buffer.push(&[1]).is_err());

            let _ = ring_buffer.pop(3).unwrap();
            assert_eq!(1, ring_buffer.used_space());
            assert_eq!(3, ring_buffer.available_space());

            ring_buffer.push(&[5, 6]).unwrap();
            assert_eq!(3, ring_buffer.used_space());

            let popped = ring_buffer.pop(2).unwrap();
            assert_eq!(2, popped.len());
            assert_eq!(4, popped[0]);
            assert_eq!(5, popped[1]);
        }

        #[test]
        fn current_empty_slices_mut() {
            let mut ring_buffer: RingBuffer<u8, 4> = RingBuffer::<u8, 4>::new();

            let slices: [&mut [MaybeUninit<u8>]; 2] = ring_buffer.current_empty_slices_mut();
            assert!(slices[0].len() == 4 && slices[1].len() == 0);

            ring_buffer.push(&[1, 2, 3]).unwrap();
            let slices: [&mut [MaybeUninit<u8>]; 2] = ring_buffer.current_empty_slices_mut();
            assert!(slices[0].len() == 1 && slices[1].len() == 0);

            ring_buffer.pop(2).unwrap();
            let slices: [&mut [MaybeUninit<u8>]; 2] = ring_buffer.current_empty_slices_mut();
            assert!(slices[0].len() == 1 && slices[1].len() == 2);

            ring_buffer.push(&[4, 5, 6]).unwrap();
            let slices: [&mut [MaybeUninit<u8>]; 2] = ring_buffer.current_empty_slices_mut();
            assert!(slices[0].len() == 0 && slices[1].len() == 0);
        }
    }

    mod ring_buffer_view {
        use super::*;

        #[test]
        fn index() {
            let mut ring_buffer: RingBuffer<u8, 32> = RingBuffer::<u8, 32>::new();
            ring_buffer.push(&[1, 2]).unwrap();

            let view = ring_buffer.pop(2).unwrap();
            assert_eq!(1, view[0]);
            assert_eq!(2, view[1]);
        }

        #[test]
        #[should_panic]
        fn index_out_of_bounds() {
            let mut ring_buffer: RingBuffer<u8, 32> = RingBuffer::<u8, 32>::new();
            ring_buffer.push(&[1, 2, 3]).unwrap();
            let view = ring_buffer.pop(2).unwrap();

            view[3];
        }
    }
}
