use std::ops::{Deref, DerefMut};
use std::vec;

pub trait SyncTrait: Into<SyncBytes> {
    /// Returns [None] (default implementation) iff size is not fixed.
    fn fixed_size(&self) -> Option<usize> {
        None
    }
}

pub struct SyncBytes(Vec<u8>);

impl Deref for SyncBytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SyncBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for SyncBytes {
    type Item = u8;
    type IntoIter = vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl SyncBytes {
    pub fn new(inner: Vec<u8>) -> Self {
        Self(inner)
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl SyncTrait for u8 {
    fn fixed_size(&self) -> Option<usize> {
        Some(size_of::<u8>())
    }
}

impl From<u8> for SyncBytes {
    fn from(value: u8) -> Self {
        SyncBytes(Vec::from([value]))
    }
}

impl SyncTrait for u16 {
    fn fixed_size(&self) -> Option<usize> {
        const { Some(size_of::<u16>()) }
    }
}

impl From<u16> for SyncBytes {
    fn from(value: u16) -> Self {
        let big_endian: [u8; 2] = value.to_be_bytes() as [u8; 2];
        SyncBytes(Vec::from(&big_endian))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod correct_size {
        use super::*;

        #[test]
        fn r#u8() {
            assert_eq!(
                u8::default().fixed_size().unwrap(),
                SyncBytes::from(u8::default()).len()
            )
        }

        #[test]
        fn r#u16() {
            assert_eq!(
                u16::default().fixed_size().unwrap(),
                SyncBytes::from(u16::default()).len()
            )
        }
    }
}
