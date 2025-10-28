use crate::map::Hex;
use crate::player::Player;
use crate::sync::{SyncBytes, SyncTrait};

#[derive(Clone)]
pub struct SyncGame {
    pub map: SyncMap,
    pub players: Vec<Player>,
}

impl SyncTrait for SyncGame {}

impl From<SyncGame> for SyncBytes {
    fn from(value: SyncGame) -> Self {
        let mut out: Vec<u8> = Vec::new();
        out.extend_from_slice(SyncBytes::from(value.map).as_slice());
        out.extend_from_slice(SyncBytes::from(value.players.len() as u16).as_slice());
        out.extend(value.players.iter().map(|player| SyncBytes::from(player.clone())).flatten());
        SyncBytes::new(out)
    }
}

#[derive(Clone)]
pub struct SyncMap {
    pub hexes: Vec<Hex>,
}

impl SyncTrait for SyncMap {}

impl From<SyncMap> for SyncBytes {
    fn from(value: SyncMap) -> Self {
        if value.hexes.is_empty() {
            return SyncBytes::new(Vec::with_capacity(0));
        }

        let size: usize = value.hexes[0].fixed_size().unwrap() * value.hexes.len();
        let mut out: Vec<u8> = Vec::with_capacity(size);

        out.extend_from_slice(SyncBytes::from(value.hexes.len() as u16).as_slice());
        out.extend(value.hexes.iter().map(|hex| SyncBytes::from(hex.clone())).flatten());
        SyncBytes::new(out)
    }
}
