use crate::map::{HexCoord, ResourceType};
use crate::sync::SyncTrait;

#[derive(Debug, Copy, Clone)]
pub struct Hex {
    pub hex_coord: HexCoord,
    pub resource_type: ResourceType,
}

impl Default for Hex {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl SyncTrait for Hex {
    fn as_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::with_capacity(self.fixed_size().unwrap());
        out.extend_from_slice(self.hex_coord.as_bytes().as_slice());
        out.extend_from_slice(self.resource_type.as_bytes().as_slice());
        out
    }

    fn fixed_size(&self) -> Option<usize> {
        Some(self.hex_coord.fixed_size()? + self.resource_type.fixed_size()?)
    }
}

impl Hex {
    pub const DEFAULT: Hex = Hex {
        hex_coord: HexCoord::DEFAULT,
        resource_type: ResourceType::DEFAULT,
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_size() {
        assert_eq!(Hex::default().fixed_size().unwrap(), Hex::default().as_bytes().len())
    }
}
