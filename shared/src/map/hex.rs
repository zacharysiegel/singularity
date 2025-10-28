use crate::map::{HexCoord, ResourceType};
use crate::sync::{SyncBytes, SyncTrait};

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
    fn fixed_size(&self) -> Option<usize> {
        Some(self.hex_coord.fixed_size()? + self.resource_type.fixed_size()?)
    }
}

impl From<Hex> for SyncBytes {
    fn from(value: Hex) -> Self {
        let mut out: Vec<u8> = Vec::with_capacity(value.fixed_size().unwrap());
        out.extend_from_slice(SyncBytes::from(value.hex_coord).as_slice());
        out.extend_from_slice(SyncBytes::from(value.resource_type).as_slice());
        SyncBytes::new(out)
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
        assert_eq!(
            Hex::default().fixed_size().unwrap(),
            SyncBytes::from(Hex::default()).len()
        )
    }
}
