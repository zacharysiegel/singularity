use shared::error::AppError;

const BCRYPT_COST: u32 = 10;

pub fn hash(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, BCRYPT_COST).map_err(|error| AppError::from_error_default(Box::new(error)))
}

pub fn verify(password: &str, password_hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, password_hash).map_err(|error| AppError::from_error_default(Box::new(error)))
}
