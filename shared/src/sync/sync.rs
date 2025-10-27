use crate::map::Hex;

#[derive(Debug)]
pub struct SyncPlayer {}

#[derive(Debug)]
pub struct SyncFacility {}

#[derive(Debug)]
pub struct SyncMap {
    pub hexes: Vec<Hex>,
}
