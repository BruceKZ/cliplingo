use std::{fs, io::Write};

use serde::Serialize;

use crate::{models::config::DebugConfigRecord, storage::fs_paths::ConfigPathProvider};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEvent {
    pub level: &'static str,
    pub event: &'static str,
    pub message: String,
    pub text_preview: Option<String>,
}

#[derive(Debug)]
pub enum LoggingServiceError {
    PathUnavailable,
    Io(String),
    Serialize(String),
}

impl std::fmt::Display for LoggingServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PathUnavailable => f.write_str("log path is unavailable"),
            Self::Io(error) => write!(f, "log I/O failed: {error}"),
            Self::Serialize(error) => write!(f, "log serialization failed: {error}"),
        }
    }
}

impl std::error::Error for LoggingServiceError {}

pub struct LoggingService<P> {
    path_provider: P,
}

impl<P> LoggingService<P>
where
    P: ConfigPathProvider,
{
    pub fn new(path_provider: P) -> Self {
        Self { path_provider }
    }

    pub fn log(
        &self,
        debug_config: &DebugConfigRecord,
        event: LogEvent,
    ) -> Result<(), LoggingServiceError> {
        let path = self
            .path_provider
            .log_path()
            .map_err(|_| LoggingServiceError::PathUnavailable)?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| LoggingServiceError::Io(error.to_string()))?;
        }

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|error| LoggingServiceError::Io(error.to_string()))?;
        let serialized = serialize_event(debug_config, event)?;
        file.write_all(serialized.as_bytes())
            .map_err(|error| LoggingServiceError::Io(error.to_string()))?;
        file.write_all(b"\n")
            .map_err(|error| LoggingServiceError::Io(error.to_string()))
    }
}

fn serialize_event(
    debug_config: &DebugConfigRecord,
    mut event: LogEvent,
) -> Result<String, LoggingServiceError> {
    if !debug_config.log_raw_network_errors {
        event.message = redact_text(&event.message);
    }

    if let Some(preview) = event.text_preview.take() {
        event.text_preview = Some(redact_text(&preview));
    }

    serde_json::to_string(&event).map_err(|error| LoggingServiceError::Serialize(error.to_string()))
}

fn redact_text(value: &str) -> String {
    let collapsed = value.split_whitespace().collect::<Vec<_>>().join(" ");
    format!("[redacted:{} chars]", collapsed.chars().count())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::{models::config::DebugConfigRecord, storage::fs_paths::ConfigPathProvider};

    #[derive(Clone)]
    struct FixedPathProvider {
        path: PathBuf,
    }

    impl ConfigPathProvider for FixedPathProvider {
        fn config_path(&self) -> Result<PathBuf, crate::services::config::ConfigServiceError> {
            Ok(self.path.with_file_name("config.toml"))
        }

        fn history_path(&self) -> Result<PathBuf, crate::services::config::ConfigServiceError> {
            Ok(self.path.with_file_name("history.json"))
        }

        fn log_path(&self) -> Result<PathBuf, crate::services::config::ConfigServiceError> {
            Ok(self.path.clone())
        }
    }

    #[test]
    fn log_output_redacts_sensitive_text_by_default() {
        let root = std::env::temp_dir().join(format!(
            "cliplingo-log-redaction-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time")
                .as_nanos()
        ));
        let path = root.join("app.log");
        let service = LoggingService::new(FixedPathProvider { path: path.clone() });
        let debug = DebugConfigRecord::default();

        service
            .log(
                &debug,
                LogEvent {
                    level: "error",
                    event: "provider-error",
                    message: "Authorization: Bearer sk-secret-value".to_string(),
                    text_preview: Some("private input text".to_string()),
                },
            )
            .expect("write log");

        let written = fs::read_to_string(path).expect("read log");
        assert!(written.contains("[redacted:"));
        assert!(!written.contains("sk-secret-value"));
        assert!(!written.contains("private input text"));
    }
}
