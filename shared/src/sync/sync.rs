pub trait SyncTrait {
    fn as_bytes(&self) -> Vec<u8>;

    /// Returns [None] (default implementation) iff size is not fixed.
    fn fixed_size(&self) -> Option<usize> {
        None
    }
}

impl SyncTrait for u8 {
    fn as_bytes(&self) -> Vec<u8> {
        Vec::from([*self])
    }

    fn fixed_size(&self) -> Option<usize> {
        Some(size_of::<u8>())
    }
}

impl SyncTrait for u16 {
    fn as_bytes(&self) -> Vec<u8> {
        let big_endian: [u8; 2] = self.to_be_bytes() as [u8; 2];
        Vec::from(&big_endian)
    }

    fn fixed_size(&self) -> Option<usize> {
        const { Some(size_of::<u16>()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod correct_size {
        use super::*;

        #[test]
        fn r#u8() {
            assert_eq!(u8::default().fixed_size().unwrap(), u8::default().as_bytes().len())
        }

        #[test]
        fn r#u16() {
            assert_eq!(u16::default().fixed_size().unwrap(), u16::default().as_bytes().len())
        }
    }
}
