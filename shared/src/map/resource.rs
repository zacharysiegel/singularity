use crate::color::{MAP_BACKGROUND_COLOR, METAL_BACKGROUND_COLOR, OIL_BACKGROUND_COLOR};
use crate::error::AppErrorStatic;
use crate::sync::SyncTrait;
use crate::try_from_repr;
use raylib::color::Color;
use strum::FromRepr;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, FromRepr)]
pub enum ResourceType {
    None = 0,
    Metal,
    Oil,
}

try_from_repr!(ResourceType<u8>);

impl Default for ResourceType {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl SyncTrait for ResourceType {
    const SYNC_FIXED_SIZE: Option<usize> = Some(1);

    fn to_bytes(&self) -> Vec<u8> {
        [self.clone() as u8].to_vec()
    }

    fn try_deserialize(bytes: &[u8]) -> Result<(usize, Self), AppErrorStatic> {
        check_sync_fixed_size!(bytes);

        let resource_type: ResourceType = ResourceType::try_from(bytes[0])?;
        Ok((Self::SYNC_FIXED_SIZE.unwrap(), resource_type))
    }
}

impl ResourceType {
    pub const DEFAULT: ResourceType = ResourceType::None;

    pub const fn color(&self) -> Color {
        match self {
            ResourceType::None => MAP_BACKGROUND_COLOR,
            ResourceType::Metal => METAL_BACKGROUND_COLOR,
            ResourceType::Oil => OIL_BACKGROUND_COLOR,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_size() {
        assert_eq!(
            ResourceType::SYNC_FIXED_SIZE.unwrap(),
            ResourceType::default().to_bytes().len()
        );
    }
}
