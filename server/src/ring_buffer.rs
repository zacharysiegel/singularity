#![allow(unused)] // Data structure need not be completely used

use crate::error::AppError;
use std::mem;
use std::mem::MaybeUninit;
use std::ops::Index;

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
        if self.empty {
            0
        } else if self.write_pos > self.read_pos {
            self.write_pos - self.read_pos
        } else if self.write_pos < self.read_pos {
            N - (self.read_pos - self.write_pos)
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

        self.write_pos = (self.write_pos + slice.len()) % N;
        self.empty = false;
        Ok(())
    }

    pub fn pop<'a>(&'a mut self, count: usize) -> Result<RingBufferView<'a, T>, AppError> {
        if count > self.used_space() {
            return Err(AppError::new("Not enough content in the buffer"));
        }

        let mut view: RingBufferView<'a, T> = RingBufferView {
            first: &[],
            second: &[],
        };

        if count == 0 {
            return Ok(view);
        }

        if self.read_pos + count < N {
            view.first =
                unsafe { mem::transmute(&self.buffer[self.read_pos..(self.read_pos + count)]) };
        } else {
            view.first = unsafe { mem::transmute(&self.buffer[self.read_pos..N]) };
            view.second = unsafe { mem::transmute(&self.buffer[0..(count - view.first.len())]) };

            assert_eq!(count, view.first.len() + view.second.len());
        }

        self.read_pos = (self.read_pos + count) % N;
        if self.read_pos == self.write_pos {
            self.empty = true;
        }
        Ok(view)
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
        fn pop() {
            let mut ring_buffer: RingBuffer<u8, 32> = RingBuffer::<u8, 32>::new();
            let hello_world = b"hello world";
            ring_buffer.push(hello_world).unwrap();

            let popped = ring_buffer.pop(2).unwrap();
            assert_eq!(2, popped.len());
            assert_eq!(b"h"[0], popped[0]);
            assert_eq!(b"e"[0], popped[1]);
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
