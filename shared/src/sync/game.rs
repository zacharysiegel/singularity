use crate::error::AppErrorStatic;
use crate::map::Hex;
use crate::player::Player;
use crate::sync::{SyncBytes, SyncTrait};

#[derive(Clone)]
pub struct SyncGame {
    pub map: SyncMap,
    pub players: Vec<Player>,
}

impl SyncTrait for SyncGame {
    fn try_deserialize(value: &[u8]) -> Result<(usize, Self), AppErrorStatic> {
        let mut offset: usize = 0;
        let map: (usize, SyncMap) = SyncMap::try_deserialize(&value[offset..])?;
        offset += map.0;

        let (increment, players): (usize, Vec<Player>) = Vec::<Player>::try_deserialize(&value[offset..])?;
        offset += increment;

        Ok((offset, SyncGame { map: map.1, players }))
    }
}

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

impl SyncTrait for SyncMap {
    fn try_deserialize(value: &[u8]) -> Result<(usize, Self), AppErrorStatic> {
        let mut offset: usize = 0;
        let (increment, hexes): (usize, Vec<Hex>) = Vec::<Hex>::try_deserialize(&value[offset..])?;
        offset += increment;

        Ok((offset, SyncMap { hexes }))
    }
}

impl From<SyncMap> for SyncBytes {
    fn from(value: SyncMap) -> Self {
        let size: usize = Hex::SYNC_FIXED_SIZE.unwrap() * value.hexes.len();
        let mut out: Vec<u8> = Vec::with_capacity(size);

        out.extend_from_slice(SyncBytes::from(value.hexes.len() as u16).as_slice());
        out.extend(value.hexes.iter().map(|hex| SyncBytes::from(hex.clone())).flatten());
        SyncBytes::new(out)
    }
}

impl SyncMap {
    pub fn serial_size(&self) -> usize {
        self.hexes.len() * Hex::SYNC_FIXED_SIZE.unwrap()
    }
}
