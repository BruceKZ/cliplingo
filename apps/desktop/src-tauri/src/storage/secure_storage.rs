use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use keyring::{Entry, Error as KeyringError};

use crate::services::config::ConfigServiceError;

const SERVICE_NAME: &str = "com.brucekz.cliplingo";
static SECRET_CACHE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

fn secret_cache() -> &'static Mutex<HashMap<String, String>> {
    SECRET_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub trait SecretStore {
    fn set_secret(&self, key: &str, secret: &str) -> Result<(), ConfigServiceError>;
    fn get_secret(&self, key: &str) -> Result<Option<String>, ConfigServiceError>;
    fn delete_secret(&self, key: &str) -> Result<(), ConfigServiceError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct KeyringSecretStore;

impl KeyringSecretStore {
    fn entry(&self, key: &str) -> Result<Entry, ConfigServiceError> {
        Entry::new(SERVICE_NAME, key)
            .map_err(|error| ConfigServiceError::SecretStore(error.to_string()))
    }
}

impl SecretStore for KeyringSecretStore {
    fn set_secret(&self, key: &str, secret: &str) -> Result<(), ConfigServiceError> {
        self.entry(key)?
            .set_password(secret)
            .map_err(|error| ConfigServiceError::SecretStore(error.to_string()))?;

        if let Ok(mut cache) = secret_cache().lock() {
            cache.insert(key.to_string(), secret.to_string());
        }

        Ok(())
    }

    fn get_secret(&self, key: &str) -> Result<Option<String>, ConfigServiceError> {
        if let Ok(cache) = secret_cache().lock() {
            if let Some(secret) = cache.get(key) {
                return Ok(Some(secret.clone()));
            }
        }

        match self.entry(key)?.get_password() {
            Ok(secret) => {
                if let Ok(mut cache) = secret_cache().lock() {
                    cache.insert(key.to_string(), secret.clone());
                }
                Ok(Some(secret))
            }
            Err(KeyringError::NoEntry) => Ok(None),
            Err(error) => Err(ConfigServiceError::SecretStore(error.to_string())),
        }
    }

    fn delete_secret(&self, key: &str) -> Result<(), ConfigServiceError> {
        match self.entry(key)?.delete_credential() {
            Ok(()) | Err(KeyringError::NoEntry) => {
                if let Ok(mut cache) = secret_cache().lock() {
                    cache.remove(key);
                }
                Ok(())
            }
            Err(error) => Err(ConfigServiceError::SecretStore(error.to_string())),
        }
    }
}
