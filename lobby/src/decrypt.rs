use base64::Engine;
use secr::store::SecretStore;
use secr::{cryptography, load, BASE64};
use shared::error::AppError;
use std::path::Path;
use std::sync::LazyLock;

const STORE: LazyLock<SecretStore> = LazyLock::new(|| load_secrets().expect("loading secret store"));

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
