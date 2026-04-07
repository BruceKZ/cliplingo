use std::{
    fs,
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

use crate::{models::config::HistoryConfigRecord, storage::fs_paths::ConfigPathProvider};

use super::translation::{TranslationExecutionOutput, TranslationItemOutput};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntryRecord {
    pub id: String,
    pub created_at_ms: u64,
    pub source_language: String,
    pub target_languages: Vec<String>,
    pub source_preview: String,
    pub source_text: Option<String>,
    pub translations: Vec<HistoryTranslationRecord>,
    pub provider_id: String,
    pub provider_name: String,
    pub model: String,
    pub latency_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryTranslationRecord {
    pub target_language: String,
    pub text_preview: String,
    pub text: Option<String>,
}

#[derive(Debug)]
pub enum HistoryRepositoryError {
    PathUnavailable,
    Io(String),
    Serialize(String),
}

impl std::fmt::Display for HistoryRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PathUnavailable => f.write_str("history path is unavailable"),
            Self::Io(error) => write!(f, "history I/O failed: {error}"),
            Self::Serialize(error) => write!(f, "history serialization failed: {error}"),
        }
    }
}

impl std::error::Error for HistoryRepositoryError {}

pub struct HistoryRepository<P> {
    path_provider: P,
}

impl<P> HistoryRepository<P>
where
    P: ConfigPathProvider,
{
    pub fn new(path_provider: P) -> Self {
        Self { path_provider }
    }

    pub fn list(&self) -> Result<Vec<HistoryEntryRecord>, HistoryRepositoryError> {
        let path = self
            .path_provider
            .history_path()
            .map_err(|_| HistoryRepositoryError::PathUnavailable)?;
        self.read_entries(&path)
    }

    pub fn clear(&self) -> Result<(), HistoryRepositoryError> {
        let path = self
            .path_provider
            .history_path()
            .map_err(|_| HistoryRepositoryError::PathUnavailable)?;
        if path.exists() {
            fs::remove_file(path).map_err(|error| HistoryRepositoryError::Io(error.to_string()))?;
        }
        Ok(())
    }

    pub fn append_translation(
        &self,
        history_config: &HistoryConfigRecord,
        translation: &TranslationExecutionOutput,
    ) -> Result<Vec<HistoryEntryRecord>, HistoryRepositoryError> {
        if !history_config.enabled {
            return Ok(self.list().unwrap_or_default());
        }

        let path = self
            .path_provider
            .history_path()
            .map_err(|_| HistoryRepositoryError::PathUnavailable)?;
        let mut entries = self.read_entries(&path)?;
        entries.insert(
            0,
            entry_from_translation(translation, history_config.store_full_text),
        );
        entries.truncate(history_config.max_items as usize);
        self.write_entries(&path, &entries)?;
        Ok(entries)
    }

    fn read_entries(&self, path: &Path) -> Result<Vec<HistoryEntryRecord>, HistoryRepositoryError> {
        if !path.exists() {
            return Ok(Vec::new());
        }

        let raw = fs::read_to_string(path)
            .map_err(|error| HistoryRepositoryError::Io(error.to_string()))?;
        serde_json::from_str(&raw)
            .map_err(|error| HistoryRepositoryError::Serialize(error.to_string()))
    }

    fn write_entries(
        &self,
        path: &Path,
        entries: &[HistoryEntryRecord],
    ) -> Result<(), HistoryRepositoryError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| HistoryRepositoryError::Io(error.to_string()))?;
        }

        let serialized = serde_json::to_string_pretty(entries)
            .map_err(|error| HistoryRepositoryError::Serialize(error.to_string()))?;
        let mut file = fs::File::create(path)
            .map_err(|error| HistoryRepositoryError::Io(error.to_string()))?;
        file.write_all(serialized.as_bytes())
            .map_err(|error| HistoryRepositoryError::Io(error.to_string()))?;
        file.sync_all()
            .map_err(|error| HistoryRepositoryError::Io(error.to_string()))
    }
}

fn entry_from_translation(
    translation: &TranslationExecutionOutput,
    store_full_text: bool,
) -> HistoryEntryRecord {
    HistoryEntryRecord {
        id: translation.request_id.clone(),
        created_at_ms: now_ms(),
        source_language: translation.source_language.clone(),
        target_languages: translation.target_languages.clone(),
        source_preview: preview_text(&translation.source_text),
        source_text: store_full_text.then(|| translation.source_text.clone()),
        translations: translation
            .translations
            .iter()
            .map(|item| map_history_translation(item, store_full_text))
            .collect(),
        provider_id: translation.provider_id.clone(),
        provider_name: translation.provider_name.clone(),
        model: translation.model.clone(),
        latency_ms: translation.latency_ms,
    }
}

fn map_history_translation(
    item: &TranslationItemOutput,
    store_full_text: bool,
) -> HistoryTranslationRecord {
    HistoryTranslationRecord {
        target_language: item.target_language.clone(),
        text_preview: preview_text(&item.text),
        text: store_full_text.then(|| item.text.clone()),
    }
}

fn preview_text(value: &str) -> String {
    let collapsed = value.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut preview = collapsed.chars().take(160).collect::<String>();
    if collapsed.chars().count() > 160 {
        preview.push('…');
    }
    preview
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::storage::fs_paths::ConfigPathProvider;

    #[derive(Clone)]
    struct FixedPathProvider {
        history_path: PathBuf,
        log_path: PathBuf,
    }

    impl ConfigPathProvider for FixedPathProvider {
        fn config_path(&self) -> Result<PathBuf, crate::services::config::ConfigServiceError> {
            Ok(self.history_path.with_file_name("config.toml"))
        }

        fn history_path(&self) -> Result<PathBuf, crate::services::config::ConfigServiceError> {
            Ok(self.history_path.clone())
        }

        fn log_path(&self) -> Result<PathBuf, crate::services::config::ConfigServiceError> {
            Ok(self.log_path.clone())
        }
    }

    fn translation_output(id: &str) -> TranslationExecutionOutput {
        TranslationExecutionOutput {
            request_id: id.to_string(),
            source_text: "Sensitive source text".to_string(),
            source_language: "en".to_string(),
            target_languages: vec!["zh-CN".to_string()],
            translations: vec![TranslationItemOutput {
                target_language: "zh-CN".to_string(),
                text: "敏感译文".to_string(),
            }],
            provider_id: "provider-default".to_string(),
            provider_name: "Mock".to_string(),
            model: "gpt-test".to_string(),
            latency_ms: 10,
            fallback_used: false,
            error: None,
        }
    }

    fn repository() -> HistoryRepository<FixedPathProvider> {
        let root = std::env::temp_dir().join(format!("cliplingo-history-{}", now_ms()));
        HistoryRepository::new(FixedPathProvider {
            history_path: root.join("history.json"),
            log_path: root.join("app.log"),
        })
    }

    #[test]
    fn history_respects_retention_limit() {
        let repository = repository();
        let config = HistoryConfigRecord {
            enabled: true,
            max_items: 1,
            store_full_text: false,
        };

        repository
            .append_translation(&config, &translation_output("one"))
            .expect("append first");
        let entries = repository
            .append_translation(&config, &translation_output("two"))
            .expect("append second");

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id, "two");
    }

    #[test]
    fn history_omits_full_text_by_default() {
        let repository = repository();
        let config = HistoryConfigRecord {
            enabled: true,
            max_items: 10,
            store_full_text: false,
        };

        let entries = repository
            .append_translation(&config, &translation_output("one"))
            .expect("append");

        assert_eq!(entries[0].source_text, None);
        assert_eq!(entries[0].translations[0].text, None);
        assert!(!entries[0].source_preview.is_empty());
    }
}
