use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;

use crate::{
    models::config::{AppConfigRecord, ProviderConfigRecord},
    storage::{fs_paths::ConfigPathProvider, secure_storage::SecretStore},
};

#[derive(Debug, Clone)]
pub struct ResolvedProviderConfig {
    pub provider: ProviderConfigRecord,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSecretStatus {
    pub provider_id: String,
    pub has_secret: bool,
}

#[derive(Debug)]
pub enum ConfigServiceError {
    ConfigPathUnavailable,
    Io(String),
    Serialize(String),
    SecretStore(String),
    ProviderNotFound(String),
    ProviderNotVerified(String),
    InvalidProviderId,
}

impl std::fmt::Display for ConfigServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigPathUnavailable => f.write_str("config path is unavailable"),
            Self::Io(error) => write!(f, "config I/O failed: {error}"),
            Self::Serialize(error) => write!(f, "config serialization failed: {error}"),
            Self::SecretStore(error) => write!(f, "secure storage failed: {error}"),
            Self::ProviderNotFound(provider_id) => {
                write!(f, "provider not found: {provider_id}")
            }
            Self::ProviderNotVerified(provider_id) => {
                write!(
                    f,
                    "provider must pass a connectivity check before activation: {provider_id}"
                )
            }
            Self::InvalidProviderId => f.write_str("provider id must not be empty"),
        }
    }
}

impl std::error::Error for ConfigServiceError {}

fn provider_requires_reverification(
    existing: &ProviderConfigRecord,
    next: &ProviderConfigRecord,
) -> bool {
    existing.kind != next.kind
        || existing.base_url != next.base_url
        || existing.path != next.path
        || existing.auth_scheme != next.auth_scheme
        || existing.organization != next.organization
        || existing.model != next.model
        || existing.temperature != next.temperature
        || existing.top_p != next.top_p
        || existing.max_tokens != next.max_tokens
        || existing.timeout_secs != next.timeout_secs
        || existing.custom_headers != next.custom_headers
}

pub struct ConfigService<P, S> {
    path_provider: P,
    secret_store: S,
}

impl<P, S> ConfigService<P, S>
where
    P: ConfigPathProvider,
    S: SecretStore,
{
    pub fn new(path_provider: P, secret_store: S) -> Self {
        Self {
            path_provider,
            secret_store,
        }
    }

    pub fn load(&self) -> Result<AppConfigRecord, ConfigServiceError> {
        let path = self.path_provider.config_path()?;
        eprintln!(
            "[config] load:start path={} exists={}",
            path.display(),
            path.exists()
        );
        let loaded = self.load_from_path(&path)?;
        eprintln!(
            "[config] load:done active_provider_id={:?} providers={}",
            loaded.active_provider_id,
            loaded.providers.len()
        );
        Ok(loaded)
    }

    pub fn save(&self, config: AppConfigRecord) -> Result<AppConfigRecord, ConfigServiceError> {
        let normalized = config.normalize();
        let path = self.path_provider.config_path()?;
        eprintln!(
            "[config] save:start path={} payload={}",
            path.display(),
            serde_json::to_string_pretty(&normalized)
                .unwrap_or_else(|_| "<serialize failed>".to_string())
        );
        self.write_config(&path, &normalized)?;
        eprintln!(
            "[config] save:done active_provider_id={:?} providers={}",
            normalized.active_provider_id,
            normalized.providers.len()
        );
        Ok(normalized)
    }

    pub fn upsert_provider(
        &self,
        provider: ProviderConfigRecord,
    ) -> Result<AppConfigRecord, ConfigServiceError> {
        let provider_id = provider.id.trim().to_string();
        if provider_id.is_empty() {
            return Err(ConfigServiceError::InvalidProviderId);
        }

        eprintln!(
            "[config] upsert_provider:start provider_id={} payload={}",
            provider_id,
            serde_json::to_string_pretty(&provider)
                .unwrap_or_else(|_| "<serialize failed>".to_string())
        );

        let mut config = self.load()?;
        if let Some(existing) = config
            .providers
            .iter_mut()
            .find(|existing| existing.id == provider_id)
        {
            let previous_verified_at = existing.verified_at;
            let api_key_ref = existing.api_key_ref.clone();
            let requires_reverification = provider_requires_reverification(existing, &provider);
            *existing = provider;
            existing.api_key_ref = existing.api_key_ref.clone().or(api_key_ref);
            existing.verified_at = if requires_reverification {
                None
            } else {
                previous_verified_at
            };
        } else {
            let mut provider = provider;
            provider.verified_at = None;
            config.providers.push(provider);
        }

        let saved = self.save(config)?;
        eprintln!(
            "[config] upsert_provider:done provider_id={} active_provider_id={:?} providers={}",
            provider_id,
            saved.active_provider_id,
            saved.providers.len()
        );
        Ok(saved)
    }

    pub fn remove_provider(
        &self,
        provider_id: &str,
    ) -> Result<AppConfigRecord, ConfigServiceError> {
        eprintln!("[config] remove_provider:start provider_id={provider_id}");
        let mut config = self.load()?;
        if let Some(position) = config
            .providers
            .iter()
            .position(|provider| provider.id == provider_id)
        {
            if let Some(secret_ref) = config.providers[position].api_key_ref.clone() {
                self.secret_store.delete_secret(&secret_ref)?;
            }

            config.providers.remove(position);
            if config.active_provider_id.as_deref() == Some(provider_id) {
                config.active_provider_id = None;
            }

            let saved = self.save(config)?;
            eprintln!(
                "[config] remove_provider:done provider_id={} active_provider_id={:?} providers={}",
                provider_id,
                saved.active_provider_id,
                saved.providers.len()
            );
            Ok(saved)
        } else {
            Err(ConfigServiceError::ProviderNotFound(
                provider_id.to_string(),
            ))
        }
    }

    pub fn set_active_provider(
        &self,
        provider_id: Option<String>,
    ) -> Result<AppConfigRecord, ConfigServiceError> {
        eprintln!(
            "[config] set_active_provider:start provider_id={:?}",
            provider_id
        );
        let mut config = self.load()?;

        if let Some(provider_id) = provider_id {
            if !config
                .providers
                .iter()
                .any(|provider| provider.id == provider_id)
            {
                return Err(ConfigServiceError::ProviderNotFound(provider_id));
            }
            if let Some(provider) = config
                .providers
                .iter()
                .find(|provider| provider.id == provider_id)
            {
                if provider.verified_at.is_none() {
                    return Err(ConfigServiceError::ProviderNotVerified(provider_id));
                }
            }
            config.active_provider_id = Some(provider_id);
        } else {
            config.active_provider_id = None;
        }

        let saved = self.save(config)?;
        eprintln!(
            "[config] set_active_provider:done active_provider_id={:?}",
            saved.active_provider_id
        );
        Ok(saved)
    }

    pub fn set_provider_secret(
        &self,
        provider_id: &str,
        api_key: &str,
    ) -> Result<ProviderSecretStatus, ConfigServiceError> {
        eprintln!(
            "[config] set_provider_secret:start provider_id={} api_key_len={}",
            provider_id,
            api_key.chars().count()
        );
        let mut config = self.load()?;
        let provider = config
            .providers
            .iter_mut()
            .find(|provider| provider.id == provider_id)
            .ok_or_else(|| ConfigServiceError::ProviderNotFound(provider_id.to_string()))?;

        let secret_ref = provider
            .api_key_ref
            .clone()
            .unwrap_or_else(|| format!("provider/{provider_id}"));
        self.secret_store.set_secret(&secret_ref, api_key)?;
        provider.api_key_ref = Some(secret_ref);
        provider.verified_at = None;
        self.save(config)?;

        let status = ProviderSecretStatus {
            provider_id: provider_id.to_string(),
            has_secret: true,
        };
        eprintln!("[config] set_provider_secret:done provider_id={provider_id} has_secret=true");
        Ok(status)
    }

    pub fn get_provider_secret_status(
        &self,
        provider_id: &str,
    ) -> Result<ProviderSecretStatus, ConfigServiceError> {
        eprintln!("[config] get_provider_secret_status:start provider_id={provider_id}");
        let config = self.load()?;
        let provider = config
            .providers
            .iter()
            .find(|provider| provider.id == provider_id)
            .ok_or_else(|| ConfigServiceError::ProviderNotFound(provider_id.to_string()))?;

        let has_secret = if let Some(secret_ref) = provider.api_key_ref.as_deref() {
            self.secret_store.get_secret(secret_ref)?.is_some()
        } else {
            false
        };

        let status = ProviderSecretStatus {
            provider_id: provider_id.to_string(),
            has_secret,
        };
        eprintln!(
            "[config] get_provider_secret_status:done provider_id={} has_secret={}",
            provider_id,
            status.has_secret
        );
        Ok(status)
    }

    pub fn delete_provider_secret(
        &self,
        provider_id: &str,
    ) -> Result<ProviderSecretStatus, ConfigServiceError> {
        eprintln!("[config] delete_provider_secret:start provider_id={provider_id}");
        let mut config = self.load()?;
        let provider = config
            .providers
            .iter_mut()
            .find(|provider| provider.id == provider_id)
            .ok_or_else(|| ConfigServiceError::ProviderNotFound(provider_id.to_string()))?;

        if let Some(secret_ref) = provider.api_key_ref.as_deref() {
            self.secret_store.delete_secret(secret_ref)?;
        }

        provider.api_key_ref = None;
        provider.verified_at = None;
        self.save(config)?;

        let status = ProviderSecretStatus {
            provider_id: provider_id.to_string(),
            has_secret: false,
        };
        eprintln!("[config] delete_provider_secret:done provider_id={provider_id}");
        Ok(status)
    }

    pub fn resolve_provider_config(
        &self,
        provider_id: Option<&str>,
    ) -> Result<ResolvedProviderConfig, ConfigServiceError> {
        eprintln!(
            "[config] resolve_provider_config:start requested_provider_id={:?}",
            provider_id
        );
        let config = self.load()?;
        let resolved_provider_id = provider_id
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .or_else(|| config.active_provider_id.clone())
            .ok_or_else(|| ConfigServiceError::ProviderNotFound("active provider".to_string()))?;

        let provider = config
            .providers
            .iter()
            .find(|provider| provider.id == resolved_provider_id)
            .cloned()
            .ok_or_else(|| ConfigServiceError::ProviderNotFound(resolved_provider_id.clone()))?;

        let api_key = provider
            .api_key_ref
            .as_deref()
            .map(|secret_ref| self.secret_store.get_secret(secret_ref))
            .transpose()?
            .flatten();

        eprintln!(
            "[config] resolve_provider_config:done resolved_provider_id={} has_secret={}",
            provider.id,
            api_key.is_some()
        );

        Ok(ResolvedProviderConfig { provider, api_key })
    }

    pub fn mark_provider_verified_at(
        &self,
        provider_id: &str,
        verified_at: u64,
    ) -> Result<AppConfigRecord, ConfigServiceError> {
        eprintln!("[config] mark_provider_verified:start provider_id={provider_id}");
        let mut config = self.load()?;
        let provider = config
            .providers
            .iter_mut()
            .find(|provider| provider.id == provider_id)
            .ok_or_else(|| ConfigServiceError::ProviderNotFound(provider_id.to_string()))?;

        provider.verified_at = Some(verified_at);

        let saved = self.save(config)?;
        eprintln!("[config] mark_provider_verified:done provider_id={provider_id}");
        Ok(saved)
    }

    fn load_from_path(&self, path: &Path) -> Result<AppConfigRecord, ConfigServiceError> {
        if !path.exists() {
            return Ok(AppConfigRecord::default());
        }

        let raw =
            fs::read_to_string(path).map_err(|error| ConfigServiceError::Io(error.to_string()))?;
        match toml::from_str::<AppConfigRecord>(&raw) {
            Ok(config) => Ok(config.normalize()),
            Err(_) => Ok(AppConfigRecord::default()),
        }
    }

    fn write_config(
        &self,
        path: &Path,
        config: &AppConfigRecord,
    ) -> Result<(), ConfigServiceError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| ConfigServiceError::Io(error.to_string()))?;
        }

        let serialized = toml::to_string_pretty(config)
            .map_err(|error| ConfigServiceError::Serialize(error.to_string()))?;

        let temp_path = temporary_path(path);
        let mut file = fs::File::create(&temp_path)
            .map_err(|error| ConfigServiceError::Io(error.to_string()))?;
        file.write_all(serialized.as_bytes())
            .map_err(|error| ConfigServiceError::Io(error.to_string()))?;
        file.sync_all()
            .map_err(|error| ConfigServiceError::Io(error.to_string()))?;
        fs::rename(&temp_path, path).map_err(|error| ConfigServiceError::Io(error.to_string()))
    }
}

fn temporary_path(path: &Path) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();

    path.with_extension(format!("tmp-{nonce}.toml"))
}

#[cfg(test)]
mod tests {
    use std::{
        collections::BTreeMap,
        path::PathBuf,
        sync::{Arc, Mutex},
    };

    use super::*;
    use crate::{
        models::config::{AppConfigRecord, ProviderConfigRecord},
        storage::fs_paths::ConfigPathProvider,
    };

    #[derive(Clone)]
    struct FixedPathProvider {
        path: PathBuf,
    }

    impl ConfigPathProvider for FixedPathProvider {
        fn config_path(&self) -> Result<PathBuf, ConfigServiceError> {
            Ok(self.path.clone())
        }

        fn history_path(&self) -> Result<PathBuf, ConfigServiceError> {
            Ok(self.path.with_file_name("history.json"))
        }

        fn log_path(&self) -> Result<PathBuf, ConfigServiceError> {
            Ok(self.path.with_file_name("app.log"))
        }
    }

    #[derive(Clone, Default)]
    struct InMemorySecretStore {
        values: Arc<Mutex<BTreeMap<String, String>>>,
    }

    impl SecretStore for InMemorySecretStore {
        fn set_secret(&self, key: &str, secret: &str) -> Result<(), ConfigServiceError> {
            self.values
                .lock()
                .expect("secret store lock")
                .insert(key.to_string(), secret.to_string());
            Ok(())
        }

        fn get_secret(&self, key: &str) -> Result<Option<String>, ConfigServiceError> {
            Ok(self
                .values
                .lock()
                .expect("secret store lock")
                .get(key)
                .cloned())
        }

        fn delete_secret(&self, key: &str) -> Result<(), ConfigServiceError> {
            self.values.lock().expect("secret store lock").remove(key);
            Ok(())
        }
    }

    fn service() -> (
        ConfigService<FixedPathProvider, InMemorySecretStore>,
        PathBuf,
        InMemorySecretStore,
    ) {
        let temp_dir = std::env::temp_dir().join(format!(
            "cliplingo-config-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time")
                .as_nanos()
        ));
        let config_path = temp_dir.join("config.toml");
        let secret_store = InMemorySecretStore::default();
        (
            ConfigService::new(
                FixedPathProvider {
                    path: config_path.clone(),
                },
                secret_store.clone(),
            ),
            config_path,
            secret_store,
        )
    }

    #[test]
    fn load_missing_config_returns_defaults() {
        let (service, _, _) = service();
        let config = service.load().expect("default config");
        assert_eq!(config, AppConfigRecord::default());
    }

    #[test]
    fn load_corrupted_config_returns_defaults() {
        let (service, config_path, _) = service();
        fs::create_dir_all(config_path.parent().expect("parent")).expect("mkdir");
        fs::write(&config_path, "not = { valid toml").expect("write corrupted config");

        let config = service.load().expect("default config after corruption");
        assert_eq!(config, AppConfigRecord::default());
    }

    #[test]
    fn provider_management_updates_config_without_plaintext_secrets() {
        let (service, config_path, secret_store) = service();
        let provider = ProviderConfigRecord {
            id: "provider-one".to_string(),
            name: "Provider One".to_string(),
            ..ProviderConfigRecord::default()
        };

        let config = service.upsert_provider(provider).expect("insert provider");
        assert_eq!(config.providers.len(), 1);

        let status = service
            .set_provider_secret("provider-one", "super-secret-key")
            .expect("store secret");
        assert!(status.has_secret);

        let serialized = fs::read_to_string(config_path).expect("config contents");
        assert!(!serialized.contains("super-secret-key"));
        assert!(serialized.contains("provider-one"));
        assert_eq!(
            secret_store
                .get_secret("provider/provider-one")
                .expect("secret lookup"),
            Some("super-secret-key".to_string())
        );
    }

    #[test]
    fn provider_must_be_verified_before_activation() {
        let (service, _, _) = service();
        let provider = ProviderConfigRecord {
            id: "provider-one".to_string(),
            name: "Provider One".to_string(),
            ..ProviderConfigRecord::default()
        };

        service.upsert_provider(provider).expect("insert provider");

        let error = service
            .set_active_provider(Some("provider-one".to_string()))
            .expect_err("verification should be required");

        assert!(matches!(
            error,
            ConfigServiceError::ProviderNotVerified(provider_id) if provider_id == "provider-one"
        ));
    }

    #[test]
    fn changing_provider_config_clears_verification() {
        let (service, _, _) = service();
        let provider = ProviderConfigRecord {
            id: "provider-one".to_string(),
            name: "Provider One".to_string(),
            base_url: "https://example.com".to_string(),
            model: "gpt-test".to_string(),
            ..ProviderConfigRecord::default()
        };

        service.upsert_provider(provider).expect("insert provider");
        let verified = service
            .mark_provider_verified_at("provider-one", 1_744_202_096_000)
            .expect("mark verified");
        assert_eq!(verified.providers[0].verified_at.is_some(), true);

        let updated = service
            .upsert_provider(ProviderConfigRecord {
                id: "provider-one".to_string(),
                name: "Provider One".to_string(),
                base_url: "https://another.example.com".to_string(),
                model: "gpt-test".to_string(),
                ..ProviderConfigRecord::default()
            })
            .expect("update provider");

        assert_eq!(updated.providers[0].verified_at, None);
    }

    #[test]
    fn changing_provider_secret_clears_verification() {
        let (service, _, _) = service();
        let provider = ProviderConfigRecord {
            id: "provider-one".to_string(),
            name: "Provider One".to_string(),
            base_url: "https://example.com".to_string(),
            model: "gpt-test".to_string(),
            ..ProviderConfigRecord::default()
        };

        service.upsert_provider(provider).expect("insert provider");
        service
            .mark_provider_verified_at("provider-one", 1_744_202_096_000)
            .expect("mark verified");

        service
            .set_provider_secret("provider-one", "secret")
            .expect("set secret");
        let after_secret_change = service.load().expect("load config");
        assert_eq!(after_secret_change.providers[0].verified_at, None);
    }

    #[test]
    fn removing_provider_also_removes_secret_and_active_provider() {
        let (service, _, secret_store) = service();
        let provider = ProviderConfigRecord {
            id: "provider-one".to_string(),
            name: "Provider One".to_string(),
            ..ProviderConfigRecord::default()
        };
        service.upsert_provider(provider).expect("insert provider");
        service
            .set_provider_secret("provider-one", "secret")
            .expect("store secret");
        service
            .mark_provider_verified_at("provider-one", 1_744_202_096_000)
            .expect("mark verified");
        service
            .set_active_provider(Some("provider-one".to_string()))
            .expect("activate provider");

        let config = service
            .remove_provider("provider-one")
            .expect("remove provider");

        assert!(config.providers.is_empty());
        assert_eq!(config.active_provider_id, None);
        assert_eq!(
            secret_store
                .get_secret("provider/provider-one")
                .expect("secret lookup"),
            None
        );
    }
}
