use microtype::SecretMicrotype;

use std::fs::read_to_string;
use std::path::PathBuf;

use crate::errors::Error;
use crate::model::ApiKey;

pub const KEY_ENV_VAR_NAME: &str = "BUS_FACTOR_AUTH";

pub fn get_api_key(key_file: &Option<PathBuf>) -> Result<ApiKey, Error> {
    if let Some(key) = api_key_from_env() {
        info!("using API key from env var");
        Ok(key)
    } else if let Some(key) = api_key_from_file(key_file) {
        info!("using API key from file: {:?}", key_file);
        Ok(key)
    } else {
        bail!(crate::errors::ErrorKind::MissingAuth)
    }
}

fn api_key_from_env() -> Option<ApiKey> {
    std::env::var(KEY_ENV_VAR_NAME).map(ApiKey::new).ok()
}

fn api_key_from_file(key_file: &Option<PathBuf>) -> Option<ApiKey> {
    if let Some(path) = key_file {
        read_to_string(path)
            .map(|s| s.trim().to_owned())
            .map(ApiKey::new)
            .ok()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use secrecy::ExposeSecret;

    use crate::tests::test_file_path;

    use super::*;

    #[test]
    fn api_key_from_env_reads_env() {
        std::env::set_var(KEY_ENV_VAR_NAME, "my secret key");
        let key = api_key_from_env().unwrap();
        assert_eq!(key.expose_secret(), "my secret key");
    }

    #[test]
    fn api_key_from_file_none() {
        assert!(matches!(api_key_from_file(&None), None));
    }

    #[test]
    fn api_key_from_file_some() {
        let path = test_file_path("example_key_file");
        let key = api_key_from_file(&Some(path)).unwrap();
        assert_eq!(key.expose_secret(), "example key");
    }

    #[test]
    fn api_key_from_file_missing_file() {
        let path = test_file_path("doesnt_exist");
        let key = api_key_from_file(&Some(path));
        assert!(key.is_none());
    }


}
