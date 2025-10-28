use crate::map::Hex;
use crate::player::Player;
use crate::sync::SyncTrait;

pub struct SyncGame {
    pub map: SyncMap,
    pub players: Vec<Player>,
}

impl SyncTrait for SyncGame {
    fn as_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend_from_slice(self.map.as_bytes().as_slice());
        out.extend_from_slice((self.players.len() as u16).as_bytes().as_slice());
        out.extend(
            self.players
                .iter()
                .map(|player| player.as_bytes())
                .flatten(),
        );
        out
    }
}

pub struct SyncMap {
    pub hexes: Vec<Hex>,
}

impl SyncTrait for SyncMap {
    fn as_bytes(&self) -> Vec<u8> {
        if self.hexes.is_empty() {
            return Vec::with_capacity(0);
        }

        let size: usize = self.hexes[0].fixed_size().unwrap() * self.hexes.len();
        let mut out: Vec<u8> = Vec::with_capacity(size);

        out.extend_from_slice((self.hexes.len() as u16).as_bytes().as_slice());
        out.extend(self.hexes.iter().map(|hex| hex.as_bytes()).flatten());
        out
    }
}
