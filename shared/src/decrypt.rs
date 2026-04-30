use crate::error::AppError;
use base64::Engine;
use secr::store::SecretStore;
use secr::{cryptography, load, BASE64};
use std::path::Path;
use std::sync::LazyLock;

const STORE: LazyLock<SecretStore> = LazyLock::new(|| load_secrets().expect("loading secret store"));

/// ## Standard usage:
/// ```
/// let runtime_environment: RuntimeEnvironment = RuntimeEnvironment::default();
/// let key: String = format!("<key>.{runtime_environment}");
/// let secret: Vec<u8> = decrypt::master_decrypt(&key)?;
/// let secret: String = String::from_utf8(secret)?;
/// ```
pub fn master_decrypt(secret_name: &str) -> Result<Vec<u8>, AppError> {
    let master_secret: String = dotenvy::var("MASTER_SECRET")?;
    let master_secret: Vec<u8> = BASE64.decode(master_secret)?;
    let secret: Vec<u8> = cryptography::decrypt(&STORE, &master_secret, secret_name)?;
    Ok(secret)
}

fn load_secrets() -> Result<SecretStore, AppError> {
    let store_path: String = dotenvy::var("SECR__STORE_PATH")?;
    let store: SecretStore = load::load_secrets_from_file(Path::new(&store_path))?;
    Ok(store)
}
