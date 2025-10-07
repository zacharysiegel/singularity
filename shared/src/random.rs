use rand::{RngCore, rngs::ThreadRng};
use uuid::Uuid;

pub fn random_uuid() -> Uuid {
    let mut rng: ThreadRng = rand::rng();
    let mut random_bytes: [u8; 128 >> 3] = [0; 128 >> 3]; // 128 bits
    rng.fill_bytes(&mut random_bytes); // ThreadRng::fill_bytes never panics

    assert_eq!(random_bytes.len(), 16);
    Uuid::from_slice(&random_bytes[..]).unwrap() // Err is only returned for non 16 byte length
}
