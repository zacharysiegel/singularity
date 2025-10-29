use crate::error::AppErrorStatic;
use std::ops::{Deref, DerefMut};
use std::vec;

macro_rules! check_sync_fixed_size {
    ($id:ident) => {
        // I would like to add a compiler error when SYNC_FIXED_SIZE.is_none(), but the compiler doesn't see the non-default definitions of this constant
        if $id.len() != Self::SYNC_FIXED_SIZE.unwrap() {
            return ::std::result::Result::Err(crate::error::AppErrorStatic::new("invalid size"));
        }
    };
}

pub trait SyncTrait: Into<SyncBytes> + TryFrom<SyncBytes> {
    /// [None] (default implementation) iff size is not fixed.
    const SYNC_FIXED_SIZE: Option<usize> = None;
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

impl From<&[u8]> for SyncBytes {
    fn from(value: &[u8]) -> Self {
        // perf: This copy is inefficient but very convenient
        SyncBytes::new(value.to_vec())
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
    const SYNC_FIXED_SIZE: Option<usize> = Some(size_of::<u8>());
}

impl From<u8> for SyncBytes {
    fn from(value: u8) -> Self {
        SyncBytes(Vec::from([value]))
    }
}

impl TryFrom<SyncBytes> for u8 {
    type Error = AppErrorStatic;

    fn try_from(value: SyncBytes) -> Result<Self, Self::Error> {
        check_sync_fixed_size!(value);

        let x: &u8 = value.get(0).ok_or_else(|| AppErrorStatic::default())?;
        Ok(*x)
    }
}

impl SyncTrait for u16 {
    const SYNC_FIXED_SIZE: Option<usize> = Some(size_of::<u16>());
}

impl From<u16> for SyncBytes {
    fn from(value: u16) -> Self {
        let big_endian: [u8; 2] = value.to_be_bytes() as [u8; 2];
        SyncBytes(Vec::from(&big_endian))
    }
}

impl TryFrom<SyncBytes> for u16 {
    type Error = AppErrorStatic;

    fn try_from(value: SyncBytes) -> Result<Self, Self::Error> {
        check_sync_fixed_size!(value);

        let bytes = <[u8; 2]>::try_from(&value[0..2])?;
        Ok(u16::from_be_bytes(bytes))
    }
}

// todo: different parse trait instead of TryFrom?
pub fn parse_vec<T>(bytes: &[u8]) -> Result<(usize, Vec<T>), AppErrorStatic>
where
    T: TryFrom<SyncBytes, Error = AppErrorStatic>,
{
    if bytes.len() < size_of::<u16>() {
        return Err(AppErrorStatic::new("invalid size"));
    }

    let length_bytes: [u8; 2] = bytes[0..2].try_into()?;
    let length: usize = usize::from(u16::from_be_bytes(length_bytes));

    if bytes.len() < size_of::<u16>() + length {
        return Err(AppErrorStatic::new("invalid size"));
    }

    let mut i: usize = 0;
    let size = size_of::<T>();
    let mut out: Vec<T> = Vec::with_capacity(length / size);
    while i < length {
        let bytes: &[u8] = &bytes[(2 + i)..(2 + i + size)];
        let target: T = T::try_from(SyncBytes::from(bytes))?;

        out.push(target);
        i += size;
    }

    Ok((2 + length, out))
}

#[cfg(test)]
mod test {
    use super::*;

    mod correct_size {
        use super::*;

        #[test]
        fn r#u8() {
            assert_eq!(u8::SYNC_FIXED_SIZE.unwrap(), SyncBytes::from(u8::default()).len())
        }

        #[test]
        fn r#u16() {
            assert_eq!(u16::SYNC_FIXED_SIZE.unwrap(), SyncBytes::from(u16::default()).len())
        }
    }
}
