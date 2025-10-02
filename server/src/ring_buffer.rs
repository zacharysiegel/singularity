use crate::error::AppError;
use std::ops::Index;

struct RingBuffer<const N: usize> {
    buffer: [u8; N],
    read_pos: usize,
    write_pos: usize,
    empty: bool,
}

impl<const N: usize> RingBuffer<N> {
    const fn new() -> Self {
        Self {
            buffer: [0; N],
            read_pos: 0,
            write_pos: 0,
            empty: true,
        }
    }

    const fn capacity(&self) -> usize {
        N
    }

    const fn used_space(&self) -> usize {
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

    const fn available_space(&self) -> usize {
        N - self.used_space()
    }

    const fn is_empty(&self) -> bool {
        self.empty
    }

    const fn is_full(&self) -> bool {
        self.used_space() == N
    }

    fn push(&mut self, slice: &[u8]) -> Result<(), AppError> {
        if slice.len() > self.available_space() {
            return Err(AppError::new("Not enough space in the buffer"));
        }

        if (self.write_pos + slice.len()) < N {
            let target: &mut [u8] =
                &mut self.buffer[self.write_pos..(self.write_pos + slice.len())];
            target.copy_from_slice(slice);
            assert_eq!(
                &self.buffer[self.write_pos..(self.write_pos + slice.len())],
                slice
            );
        } else {
            let a = &mut self.buffer[self.write_pos..N];
            let a_len = a.len();
            a.copy_from_slice(&slice[0..a_len]);

            let b = &mut self.buffer[0..(slice.len() - a_len)];
            let b_len = b.len();
            b.copy_from_slice(&slice[a_len..]);

            assert_eq!(slice.len(), a_len + b_len);
            assert_eq!(slice[0..a_len], self.buffer[self.write_pos..N]);
            assert_eq!(slice[a_len..], self.buffer[0..b_len]);
        }

        self.write_pos = (self.write_pos + slice.len()) % N;
        self.empty = false;
        Ok(())
    }

    fn pop<'a>(&'a mut self, count: usize) -> Result<RingBufferView<'a>, AppError> {
        if count > self.used_space() {
            return Err(AppError::new("Not enough content in the buffer"));
        }

        let mut view: RingBufferView = RingBufferView {
            first: &[],
            second: &[],
        };

        if count == 0 {
            return Ok(view);
        }

        if self.read_pos + count < N {
            view.first = &self.buffer[self.read_pos..(self.read_pos + count)];
        } else {
            view.first = &self.buffer[self.read_pos..N];
            view.second = &self.buffer[0..(count- view.first.len())];

            assert_eq!(count, view.first.len() + view.second.len());
        }

        self.read_pos = (self.read_pos + count) % N;
        if self.read_pos == self.write_pos {
            self.empty = true;
        }
        Ok(view)
    }
}

struct RingBufferView<'a> {
    pub first: &'a [u8],
    pub second: &'a [u8],
}

impl<'a> RingBufferView<'a> {
    fn iter(&self) -> impl Iterator<Item = &u8> {
        self.first.iter().chain(self.second.iter())
    }

    fn len(&self) -> usize {
        self.first.len() + self.second.len()
    }

    fn copy_to(&self, dest: &mut [u8]) {
        dest[..self.first.len()].copy_from_slice(self.first);
        dest[self.first.len()..].copy_from_slice(self.second);
    }

    fn as_slices(&self) -> (&[u8], &[u8]) {
        (self.first, self.second)
    }
}

impl<'a> Index<usize> for RingBufferView<'a> {
    type Output = u8;

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
            let mut ring_buffer: RingBuffer<32> = RingBuffer::<32>::new();
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
            assert_eq!(&ring_buffer.buffer[0..hello_world.len()], hello_world);
            assert_eq!(
                &ring_buffer.buffer[hello_world.len()..(hello_world.len() + numbers.len())],
                numbers
            );
        }

        #[test]
        fn pop() {
            let mut ring_buffer: RingBuffer<32> = RingBuffer::<32>::new();
            let hello_world = b"hello world";
            ring_buffer.push(hello_world).unwrap();

            let popped = ring_buffer.pop(2).unwrap();
            assert_eq!(2, popped.len());
            assert_eq!(b"h"[0], popped[0]);
            assert_eq!(b"e"[0], popped[1]);
        }

        #[test]
        fn wrap() {
            let mut ring_buffer: RingBuffer<4> = RingBuffer::<4>::new();
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
            let mut ring_buffer: RingBuffer<32> = RingBuffer::<32>::new();
            ring_buffer.push(&[1, 2]).unwrap();

            let view = ring_buffer.pop(2).unwrap();
            assert_eq!(1, view[0]);
            assert_eq!(2, view[1]);
        }

        #[test]
        #[should_panic]
        fn index_out_of_bounds() {
            let mut ring_buffer: RingBuffer<32> = RingBuffer::<32>::new();
            ring_buffer.push(&[1, 2, 3]).unwrap();
            let view = ring_buffer.pop(2).unwrap();

            view[3];
        }
    }
}
