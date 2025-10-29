use crate::error::AppErrorStatic;
use crate::map::Hex;
use crate::player::Player;
use crate::sync;
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

impl TryFrom<SyncBytes> for SyncGame {
    type Error = AppErrorStatic;

    fn try_from(value: SyncBytes) -> Result<Self, Self::Error> {
        let mut start: usize = 0;
        let map: SyncMap = SyncMap::try_from(SyncBytes::from(&value[start..]))?;
        start += map.serial_size();

        let (_increment, players): (usize, Vec<Player>) = sync::parse_vec(&value[start..])?;
        Ok(SyncGame { map, players })
    }
}

#[derive(Clone)]
pub struct SyncMap {
    pub hexes: Vec<Hex>,
}

impl SyncTrait for SyncMap {}

impl From<SyncMap> for SyncBytes {
    fn from(value: SyncMap) -> Self {
        let size: usize = Hex::SYNC_FIXED_SIZE.unwrap() * value.hexes.len();
        let mut out: Vec<u8> = Vec::with_capacity(size);

        out.extend_from_slice(SyncBytes::from(value.hexes.len() as u16).as_slice());
        out.extend(value.hexes.iter().map(|hex| SyncBytes::from(hex.clone())).flatten());
        SyncBytes::new(out)
    }
}

impl TryFrom<SyncBytes> for SyncMap {
    type Error = AppErrorStatic;

    fn try_from(value: SyncBytes) -> Result<Self, Self::Error> {
        let start: usize = 0;
        let (_increment, hexes): (usize, Vec<Hex>) = sync::parse_vec(&value[start..])?;
        Ok(SyncMap { hexes })
    }
}

impl SyncMap {
    pub fn serial_size(&self) -> usize {
        self.hexes.len() * Hex::SYNC_FIXED_SIZE.unwrap()
    }
}
