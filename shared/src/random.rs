use uuid::Uuid;

pub fn random_uuid() -> Uuid {
    Uuid::now_v7()
}
