use uuid::Uuid;

pub fn generate_uuid() -> u128 {
    Uuid::new_v4().as_u128()
}
