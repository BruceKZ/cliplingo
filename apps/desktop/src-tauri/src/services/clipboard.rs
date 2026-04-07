use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

pub const DEFAULT_MAX_CHARS: usize = 8_000;
pub const HARD_MAX_CHARS: usize = 50_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClipboardLimits {
    pub max_chars: usize,
}

impl Default for ClipboardLimits {
    fn default() -> Self {
        Self {
            max_chars: DEFAULT_MAX_CHARS,
        }
    }
}

impl ClipboardLimits {
    pub fn new(max_chars: usize) -> Result<Self, ClipboardServiceError> {
        if max_chars == 0 {
            return Err(ClipboardServiceError::InvalidLimit {
                max_chars,
                hard_max_chars: HARD_MAX_CHARS,
            });
        }

        if max_chars > HARD_MAX_CHARS {
            return Err(ClipboardServiceError::InvalidLimit {
                max_chars,
                hard_max_chars: HARD_MAX_CHARS,
            });
        }

        Ok(Self { max_chars })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardText {
    pub text: String,
    pub character_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardServiceError {
    EmptyText,
    UnsupportedContent,
    TooLong {
        character_count: usize,
        max_chars: usize,
    },
    InvalidLimit {
        max_chars: usize,
        hard_max_chars: usize,
    },
    ReadFailed(String),
}

impl std::fmt::Display for ClipboardServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyText => f.write_str("clipboard text is empty"),
            Self::UnsupportedContent => f.write_str("clipboard content is not plain text"),
            Self::TooLong {
                character_count,
                max_chars,
            } => write!(
                f,
                "clipboard text exceeds the configured limit: {character_count} > {max_chars}"
            ),
            Self::InvalidLimit {
                max_chars,
                hard_max_chars,
            } => write!(
                f,
                "invalid clipboard limit {max_chars}; hard maximum is {hard_max_chars}"
            ),
            Self::ReadFailed(message) => write!(f, "failed to read clipboard text: {message}"),
        }
    }
}

impl std::error::Error for ClipboardServiceError {}

pub trait ClipboardReader {
    fn read_text(&self) -> Result<Option<String>, ClipboardServiceError>;
}

pub struct ClipboardService<R> {
    reader: R,
    limits: ClipboardLimits,
}

impl<R> ClipboardService<R>
where
    R: ClipboardReader,
{
    pub fn new(reader: R, limits: ClipboardLimits) -> Self {
        Self { reader, limits }
    }

    pub fn read_normalized_text(&self) -> Result<ClipboardText, ClipboardServiceError> {
        let Some(raw_text) = self.reader.read_text()? else {
            return Err(ClipboardServiceError::UnsupportedContent);
        };

        let text = normalize_clipboard_text(&raw_text);
        if text.is_empty() {
            return Err(ClipboardServiceError::EmptyText);
        }

        let character_count = text.chars().count();
        if character_count > self.limits.max_chars {
            return Err(ClipboardServiceError::TooLong {
                character_count,
                max_chars: self.limits.max_chars,
            });
        }

        Ok(ClipboardText {
            text,
            character_count,
        })
    }
}

pub struct TauriClipboardReader {
    app: AppHandle,
}

impl TauriClipboardReader {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }
}

impl ClipboardReader for TauriClipboardReader {
    fn read_text(&self) -> Result<Option<String>, ClipboardServiceError> {
        let text = self
            .app
            .clipboard()
            .read_text()
            .map_err(|error| ClipboardServiceError::ReadFailed(error.to_string()))?;

        if text.is_empty() {
            return Ok(None);
        }

        Ok(Some(text))
    }
}

pub fn read_clipboard_text(app: AppHandle) -> Result<ClipboardText, ClipboardServiceError> {
    read_clipboard_text_with_limits(app, ClipboardLimits::default())
}

pub fn read_clipboard_text_with_limits(
    app: AppHandle,
    limits: ClipboardLimits,
) -> Result<ClipboardText, ClipboardServiceError> {
    let reader = TauriClipboardReader::new(app);
    let service = ClipboardService::new(reader, limits);
    service.read_normalized_text()
}

pub fn normalize_clipboard_text(input: &str) -> String {
    let normalized_newlines = input.replace("\r\n", "\n").replace('\r', "\n");
    let filtered: String = normalized_newlines
        .chars()
        .filter(|character| is_allowed_clipboard_character(*character))
        .collect();

    let mut compacted = String::with_capacity(filtered.len());
    for character in filtered.chars() {
        if character == '\n' {
            while matches!(compacted.chars().last(), Some(' ' | '\t')) {
                compacted.pop();
            }
        }

        compacted.push(character);
    }

    compacted.trim().to_owned()
}

fn is_allowed_clipboard_character(character: char) -> bool {
    matches!(character, '\n' | '\t')
        || (!character.is_control()
            && character != '\u{feff}'
            && character != '\u{200b}'
            && character != '\u{200c}'
            && character != '\u{200d}'
            && character != '\u{2060}')
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeClipboardReader {
        result: Result<Option<String>, ClipboardServiceError>,
    }

    impl ClipboardReader for FakeClipboardReader {
        fn read_text(&self) -> Result<Option<String>, ClipboardServiceError> {
            self.result.clone()
        }
    }

    #[test]
    fn normalize_clipboard_text_strips_control_characters_and_standardizes_newlines() {
        let input = "\u{feff}  hello\r\nworld\u{200b}\rtest\tvalue\u{0000}  ";

        let output = normalize_clipboard_text(input);

        assert_eq!(output, "hello\nworld\ntest\tvalue");
    }

    #[test]
    fn normalize_clipboard_text_preserves_paragraph_structure() {
        let input = " first line \r\n\r\nsecond line \r third line ";

        let output = normalize_clipboard_text(input);

        assert_eq!(output, "first line\n\nsecond line\n third line");
    }

    #[test]
    fn read_normalized_text_rejects_empty_text() {
        let reader = FakeClipboardReader {
            result: Ok(Some("   \r\n\t".to_string())),
        };
        let service = ClipboardService::new(reader, ClipboardLimits::default());

        let error = service.read_normalized_text().unwrap_err();

        assert_eq!(error, ClipboardServiceError::EmptyText);
    }

    #[test]
    fn read_normalized_text_rejects_non_text_clipboard_content() {
        let reader = FakeClipboardReader { result: Ok(None) };
        let service = ClipboardService::new(reader, ClipboardLimits::default());

        let error = service.read_normalized_text().unwrap_err();

        assert_eq!(error, ClipboardServiceError::UnsupportedContent);
    }

    #[test]
    fn read_normalized_text_rejects_over_limit_payloads() {
        let reader = FakeClipboardReader {
            result: Ok(Some("a".repeat(9_000))),
        };
        let service =
            ClipboardService::new(reader, ClipboardLimits::new(8_000).expect("valid limit"));

        let error = service.read_normalized_text().unwrap_err();

        assert_eq!(
            error,
            ClipboardServiceError::TooLong {
                character_count: 9_000,
                max_chars: 8_000,
            }
        );
    }

    #[test]
    fn read_normalized_text_returns_normalized_clipboard_text() {
        let reader = FakeClipboardReader {
            result: Ok(Some("  hello\r\nworld  ".to_string())),
        };
        let service = ClipboardService::new(reader, ClipboardLimits::default());

        let output = service.read_normalized_text().expect("normalized text");

        assert_eq!(
            output,
            ClipboardText {
                text: "hello\nworld".to_string(),
                character_count: 11,
            }
        );
    }

    #[test]
    fn clipboard_limits_rejects_zero_and_hard_max_overflow() {
        assert_eq!(
            ClipboardLimits::new(0).unwrap_err(),
            ClipboardServiceError::InvalidLimit {
                max_chars: 0,
                hard_max_chars: HARD_MAX_CHARS,
            }
        );

        assert_eq!(
            ClipboardLimits::new(HARD_MAX_CHARS + 1).unwrap_err(),
            ClipboardServiceError::InvalidLimit {
                max_chars: HARD_MAX_CHARS + 1,
                hard_max_chars: HARD_MAX_CHARS,
            }
        );
    }
}
