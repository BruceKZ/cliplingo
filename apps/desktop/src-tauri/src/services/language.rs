use serde::Serialize;
use whatlang::{detect, Lang, Script};

use crate::models::config::LanguageRoutingRuleRecord;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageDetectionResult {
    pub source_language: Option<String>,
    pub script: Option<String>,
    pub reliable: bool,
    pub fallback_used: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageRoutingResult {
    pub detected_source_language: Option<String>,
    pub target_languages: Vec<String>,
    pub rule_kind: String,
    pub fallback_used: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LanguageDetectionService;

impl LanguageDetectionService {
    pub fn new() -> Self {
        Self
    }

    pub fn detect(&self, text: &str) -> LanguageDetectionResult {
        let normalized = text.trim();
        if normalized.is_empty() {
            return LanguageDetectionResult {
                source_language: None,
                script: None,
                reliable: false,
                fallback_used: true,
            };
        }

        if let Some(code) = detect_by_script(normalized) {
            return LanguageDetectionResult {
                source_language: Some(code),
                script: Some("Han".to_string()),
                reliable: true,
                fallback_used: false,
            };
        }

        let detected = detect(normalized);
        match detected {
            Some(info) if info.is_reliable() => LanguageDetectionResult {
                source_language: Some(map_detected_language(info.lang())),
                script: Some(script_name(info.script())),
                reliable: true,
                fallback_used: false,
            },
            Some(info) => {
                let code = map_detected_language(info.lang());
                let resolved = if code.is_empty() { None } else { Some(code) };
                let fallback_used = resolved.is_none();

                LanguageDetectionResult {
                    source_language: resolved,
                    script: Some(script_name(info.script())),
                    reliable: false,
                    fallback_used,
                }
            }
            None => LanguageDetectionResult {
                source_language: None,
                script: None,
                reliable: false,
                fallback_used: true,
            },
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LanguageRouter;

impl LanguageRouter {
    pub fn resolve(
        &self,
        rule: &LanguageRoutingRuleRecord,
        detected_source_language: Option<&str>,
    ) -> LanguageRoutingResult {
        let normalized_source = detected_source_language.and_then(normalize_language_code);

        match rule {
            LanguageRoutingRuleRecord::Bidirectional {
                primary_source_language,
                primary_target_languages,
                secondary_source_language,
                secondary_target_languages,
            } => {
                let fallback = merge_unique(
                    primary_target_languages.iter().cloned(),
                    secondary_target_languages.iter().cloned(),
                );

                let selected = match normalized_source.as_deref() {
                    Some(source) if language_eq(source, primary_source_language) => {
                        primary_target_languages.iter().cloned().collect()
                    }
                    Some(source) if language_eq(source, secondary_source_language) => {
                        secondary_target_languages.iter().cloned().collect()
                    }
                    _ => fallback,
                };
                let fallback_used = normalized_source.is_none();

                LanguageRoutingResult {
                    detected_source_language: normalized_source,
                    target_languages: selected,
                    rule_kind: "bidirectional".to_string(),
                    fallback_used,
                }
            }
            LanguageRoutingRuleRecord::Branching {
                english_source_language,
                english_target_languages,
                chinese_source_language,
                chinese_target_languages,
                fallback_target_languages,
            } => {
                let selected = match normalized_source.as_deref() {
                    Some(source) if language_eq(source, english_source_language) => {
                        english_target_languages.iter().cloned().collect()
                    }
                    Some(source) if language_eq(source, chinese_source_language) => {
                        chinese_target_languages.iter().cloned().collect()
                    }
                    _ => fallback_target_languages.iter().cloned().collect(),
                };
                let fallback_used = normalized_source.as_deref().map_or(true, |source| {
                    !(language_eq(source, english_source_language)
                        || language_eq(source, chinese_source_language))
                });

                LanguageRoutingResult {
                    detected_source_language: normalized_source,
                    target_languages: selected,
                    rule_kind: "branching".to_string(),
                    fallback_used,
                }
            }
            LanguageRoutingRuleRecord::Fixed { target_languages } => {
                let fallback_used = normalized_source.is_none();
                LanguageRoutingResult {
                    detected_source_language: normalized_source,
                    target_languages: target_languages.iter().cloned().collect(),
                    rule_kind: "fixed".to_string(),
                    fallback_used,
                }
            }
        }
    }

    pub fn resolve_for_text(
        &self,
        rule: &LanguageRoutingRuleRecord,
        detector: &LanguageDetectionService,
        text: &str,
    ) -> (LanguageDetectionResult, LanguageRoutingResult) {
        let detection = detector.detect(text);
        let routing = self.resolve(rule, detection.source_language.as_deref());
        (detection, routing)
    }
}

fn detect_by_script(text: &str) -> Option<String> {
    let cjk_count = text.chars().filter(|ch| is_cjk(*ch)).count();
    let latin_count = text.chars().filter(|ch| ch.is_ascii_alphabetic()).count();

    if cjk_count == 0 {
        return None;
    }

    if cjk_count >= latin_count {
        Some("zh-CN".to_string())
    } else {
        None
    }
}

fn is_cjk(character: char) -> bool {
    matches!(
        character,
        '\u{4E00}'..='\u{9FFF}'
            | '\u{3400}'..='\u{4DBF}'
            | '\u{F900}'..='\u{FAFF}'
            | '\u{3040}'..='\u{30FF}'
            | '\u{AC00}'..='\u{D7AF}'
    )
}

fn map_detected_language(language: Lang) -> String {
    match language {
        Lang::Eng => "en".to_string(),
        Lang::Cmn => "zh-CN".to_string(),
        Lang::Jpn => "ja".to_string(),
        Lang::Kor => "ko".to_string(),
        _ => language.code().to_string(),
    }
}

fn script_name(script: Script) -> String {
    format!("{script:?}")
}

fn normalize_language_code(value: &str) -> Option<String> {
    let normalized = value.trim().replace('_', "-");
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

fn language_eq(left: &str, right: &str) -> bool {
    normalize_code_for_compare(left) == normalize_code_for_compare(right)
}

fn normalize_code_for_compare(value: &str) -> String {
    value.trim().replace('_', "-").to_ascii_lowercase()
}

fn merge_unique(
    primary: impl IntoIterator<Item = String>,
    secondary: impl IntoIterator<Item = String>,
) -> Vec<String> {
    let mut merged: Vec<String> = Vec::new();
    for value in primary.into_iter().chain(secondary) {
        if !merged.iter().any(|existing| language_eq(existing, &value)) {
            merged.push(value);
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_english_text() {
        let service = LanguageDetectionService::new();
        let result = service.detect(
            "This is a longer English sentence that provides enough context for reliable detection.",
        );

        assert_eq!(result.source_language.as_deref(), Some("en"));
        assert!(!result.fallback_used);
    }

    #[test]
    fn detects_chinese_text() {
        let service = LanguageDetectionService::new();
        let result = service.detect("这是一个中文句子。");

        assert_eq!(result.source_language.as_deref(), Some("zh-CN"));
        assert!(result.reliable);
        assert!(!result.fallback_used);
    }

    #[test]
    fn empty_text_falls_back_safely() {
        let service = LanguageDetectionService::new();
        let result = service.detect("   ");

        assert_eq!(result.source_language, None);
        assert!(result.fallback_used);
    }

    #[test]
    fn branching_routes_english_and_chinese_and_falls_back_for_others() {
        let router = LanguageRouter::default();
        let rule = LanguageRoutingRuleRecord::Branching {
            english_source_language: "en".to_string(),
            english_target_languages: vec!["zh-CN".to_string()],
            chinese_source_language: "zh-CN".to_string(),
            chinese_target_languages: vec!["en".to_string()],
            fallback_target_languages: vec!["en".to_string(), "zh-CN".to_string()],
        };

        let english = router.resolve(&rule, Some("en"));
        assert_eq!(english.target_languages, vec!["zh-CN"]);
        assert!(!english.fallback_used);

        let chinese = router.resolve(&rule, Some("zh-CN"));
        assert_eq!(chinese.target_languages, vec!["en"]);
        assert!(!chinese.fallback_used);

        let other = router.resolve(&rule, Some("fr"));
        assert_eq!(other.target_languages, vec!["en", "zh-CN"]);
        assert!(other.fallback_used);
    }

    #[test]
    fn bidirectional_routes_primary_secondary_and_fallback_union() {
        let router = LanguageRouter::default();
        let rule = LanguageRoutingRuleRecord::Bidirectional {
            primary_source_language: "en".to_string(),
            primary_target_languages: vec!["zh-CN".to_string()],
            secondary_source_language: "zh-CN".to_string(),
            secondary_target_languages: vec!["en".to_string()],
        };

        let primary = router.resolve(&rule, Some("en"));
        assert_eq!(primary.target_languages, vec!["zh-CN"]);

        let secondary = router.resolve(&rule, Some("zh-CN"));
        assert_eq!(secondary.target_languages, vec!["en"]);

        let fallback = router.resolve(&rule, Some("fr"));
        assert_eq!(fallback.target_languages, vec!["zh-CN", "en"]);
    }

    #[test]
    fn fixed_routes_to_configured_targets() {
        let router = LanguageRouter::default();
        let rule = LanguageRoutingRuleRecord::Fixed {
            target_languages: vec!["en".to_string(), "zh-CN".to_string()],
        };

        let routing = router.resolve(&rule, None);
        assert_eq!(routing.target_languages, vec!["en", "zh-CN"]);
        assert!(routing.fallback_used);
    }
}
