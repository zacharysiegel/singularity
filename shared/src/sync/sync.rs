use crate::map::Hex;
use crate::player::Player;

pub trait SyncTrait {
    fn as_bytes(&self) -> Vec<u8>;
}

pub struct SyncGame {
    pub map: SyncMap,
    pub players: Vec<Player>
}

impl SyncTrait for SyncGame {
    fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

pub struct SyncMap {
    pub hexes: Vec<Hex>,
}

impl SyncTrait for SyncMap {
    fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }
}
