use crate::color::{MAP_BACKGROUND_COLOR, METAL_BACKGROUND_COLOR, OIL_BACKGROUND_COLOR};
use crate::sync::SyncTrait;
use raylib::color::Color;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResourceType {
    None = 0,
    Metal,
    Oil,
}

impl Default for ResourceType {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl SyncTrait for ResourceType {
    fn as_bytes(&self) -> Vec<u8> {
        vec![*self as u8]
    }

    fn fixed_size(&self) -> Option<usize> {
        Some(1)
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
            ResourceType::default().fixed_size().unwrap(),
            ResourceType::default().as_bytes().len()
        )
    }
}
