use crate::error::AppErrorStatic;
use crate::map::Hex;
use crate::player::Player;
use crate::sync::SyncTrait;

#[derive(Clone)]
pub struct SyncGame {
    pub map: SyncMap,
    pub players: Vec<Player>,
}

impl SyncTrait for SyncGame {
    fn to_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(self.map.to_bytes());
        out.extend((self.players.len() as u16).to_bytes());
        out.extend(self.players.iter().map(|player| player.to_bytes()).flatten());
        out
    }

    fn try_deserialize(value: &[u8]) -> Result<(usize, Self), AppErrorStatic> {
        let mut offset: usize = 0;
        let map: (usize, SyncMap) = SyncMap::try_deserialize(&value[offset..])?;
        offset += map.0;

        let (increment, players): (usize, Vec<Player>) = Vec::<Player>::try_deserialize(&value[offset..])?;
        offset += increment;

        Ok((offset, SyncGame { map: map.1, players }))
    }
}

#[derive(Clone)]
pub struct SyncMap {
    pub hexes: Vec<Hex>,
}

impl SyncTrait for SyncMap {
    fn to_bytes(&self) -> Vec<u8> {
        let size: usize = Hex::SYNC_FIXED_SIZE.unwrap() * self.hexes.len();
        let mut out: Vec<u8> = Vec::with_capacity(size);

        out.extend((self.hexes.len() as u16).to_bytes());
        out.extend(self.hexes.iter().map(|hex| hex.to_bytes()).flatten());
        out
    }

    fn try_deserialize(value: &[u8]) -> Result<(usize, Self), AppErrorStatic> {
        let mut offset: usize = 0;
        let (increment, hexes): (usize, Vec<Hex>) = Vec::<Hex>::try_deserialize(&value[offset..])?;
        offset += increment;

        Ok((offset, SyncMap { hexes }))
    }
}

impl SyncMap {
    pub fn serial_size(&self) -> usize {
        self.hexes.len() * Hex::SYNC_FIXED_SIZE.unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod correct_size {
        use super::*;

        #[test]
        fn sync_game() {
            assert_eq!(
                4,
                SyncGame {
                    map: SyncMap { hexes: Vec::new() },
                    players: Vec::new()
                }
                .to_bytes()
                .len()
            );
        }

        #[test]
        fn sync_map() {
            assert_eq!(2, SyncMap { hexes: Vec::new() }.to_bytes().len());
        }
    }
}
