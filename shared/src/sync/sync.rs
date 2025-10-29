use crate::error::AppErrorStatic;

macro_rules! check_sync_fixed_size {
    ($id:ident) => {
        // I would like to add a compiler error when SYNC_FIXED_SIZE.is_none(), but the compiler doesn't see the non-default definitions of this constant
        if $id.len() < Self::SYNC_FIXED_SIZE.unwrap() {
            return ::std::result::Result::Err(crate::error::AppErrorStatic::new("invalid size"));
        }
    };
}

pub trait SyncTrait: Sized {
    /// [None] (default implementation) iff size is not fixed.
    const SYNC_FIXED_SIZE: Option<usize> = None;

    fn to_bytes(&self) -> Vec<u8>;
    fn try_deserialize(bytes: &[u8]) -> Result<(usize, Self), AppErrorStatic>;
}

impl SyncTrait for u8 {
    const SYNC_FIXED_SIZE: Option<usize> = Some(size_of::<u8>());

    fn to_bytes(&self) -> Vec<u8> {
        [self.clone()].to_vec()
    }

    fn try_deserialize(bytes: &[u8]) -> Result<(usize, Self), AppErrorStatic> {
        check_sync_fixed_size!(bytes);

        Ok((Self::SYNC_FIXED_SIZE.unwrap(), bytes[0]))
    }
}

impl SyncTrait for u16 {
    const SYNC_FIXED_SIZE: Option<usize> = Some(size_of::<u16>());

    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn try_deserialize(bytes: &[u8]) -> Result<(usize, Self), AppErrorStatic> {
        check_sync_fixed_size!(bytes);

        let array = <[u8; 2]>::try_from(&bytes[0..2])?;
        Ok((2, u16::from_be_bytes(array)))
    }
}

impl<T: SyncTrait> SyncTrait for Vec<T> {
    fn to_bytes(&self) -> Vec<u8> {
        let length: usize = self.len(); // note: Length communicates the number of objects, not total byte length
        let mut out: Vec<u8> = Vec::with_capacity(size_of::<u16>() + length);
        let length_bytes: [u8; 2] = (length as u16).to_be_bytes();

        out.extend_from_slice(length_bytes.as_slice());
        out.extend(self.iter().map(|item| item.to_bytes()).flatten());
        out
    }

    fn try_deserialize(bytes: &[u8]) -> Result<(usize, Self), AppErrorStatic> {
        let length_bytes: [u8; 2] = bytes.get(0..2).ok_or_else(|| AppErrorStatic::new("invalid size"))?.try_into()?;
        let length: usize = usize::from(u16::from_be_bytes(length_bytes));

        let mut offset: usize = 0;
        let mut out: Vec<T> = Vec::with_capacity(length);
        for _ in 0..length {
            let slice: &[u8] = &bytes.get(offset..).ok_or_else(|| AppErrorStatic::new("invalid size"))?;
            let (size, target): (usize, T) = T::try_deserialize(slice)?;
            out.push(target);
            offset += size;
        }

        Ok((offset, out))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod correct_size {
        use super::*;

        #[test]
        fn r#u8() {
            assert_eq!(u8::SYNC_FIXED_SIZE.unwrap(), u8::default().to_bytes().len())
        }

        #[test]
        fn r#u16() {
            assert_eq!(u16::SYNC_FIXED_SIZE.unwrap(), u16::default().to_bytes().len())
        }

        #[test]
        fn vec() {
            assert_eq!(2, Vec::<u8>::new().to_bytes().len());
            assert_eq!(3, Vec::<u8>::from([0]).to_bytes().len());
            assert_eq!(4, Vec::<u8>::from([0, 1]).to_bytes().len());
        }
    }
}
