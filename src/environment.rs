use microtype::SecretMicrotype;

use std::fs::read_to_string;
use std::path::PathBuf;

use crate::model::ApiKey;
use crate::errors::Error;

pub const KEY_ENV_VAR_NAME: &str = "BUS_FACTOR_AUTH";
pub const KEY_FILE_PATH: &str = ".bus_factor_auth";

pub fn get_api_key() -> Result<ApiKey, Error> {
    if let Some(key) = api_key_from_env() {
        Ok(key)
    } else if let Some(key) = api_key_from_file() {
        Ok(key)
    } else {
        Err(Error::MissingAuth)
    }
}

fn api_key_from_env() -> Option<ApiKey> {
    std::env::var(KEY_ENV_VAR_NAME).map(ApiKey::new).ok()
}

fn api_key_from_file() -> Option<ApiKey> {
    let home = std::env::var("HOME").ok()?;
    let path = PathBuf::from(home).join(KEY_FILE_PATH);
    read_to_string(path).map(|s| s.trim().to_owned()).map(ApiKey::new).ok()
}

#[cfg(test)]
mod tests {
    use secrecy::ExposeSecret;

    use super::*;

    #[test]
    fn api_key_from_env_reads_env() {
        std::env::set_var(KEY_ENV_VAR_NAME, "my secret key");
        let key = api_key_from_env().unwrap();
        assert_eq!(key.expose_secret(), "my secret key");
    }

    #[test]
    fn get_works_if_env_var_set() {
        std::env::set_var(KEY_ENV_VAR_NAME, "my secret key");
        assert!(get_api_key().is_ok());
        std::env::remove_var(KEY_ENV_VAR_NAME);
        assert!(get_api_key().is_err());
    }
}
