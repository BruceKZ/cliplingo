use serde::{Deserialize, Serialize};

pub const DEFAULT_DOUBLE_COPY_WINDOW_MS: u64 = 450;
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
pub const DEFAULT_HISTORY_LIMIT: u32 = 100;
pub const DEFAULT_MAX_TOKENS: u32 = 4_096;
pub const DEFAULT_PROVIDER_PATH: &str = "/chat/completions";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ProviderKind {
    #[default]
    #[serde(rename = "openai-compatible", alias = "open-ai-compatible")]
    OpenAiCompatible,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum AuthScheme {
    #[default]
    Bearer,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct KeyValuePairRecord {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum LanguageRoutingRuleRecord {
    Bidirectional {
        primary_source_language: String,
        primary_target_languages: Vec<String>,
        secondary_source_language: String,
        secondary_target_languages: Vec<String>,
    },
    Branching {
        english_source_language: String,
        english_target_languages: Vec<String>,
        chinese_source_language: String,
        chinese_target_languages: Vec<String>,
        fallback_target_languages: Vec<String>,
    },
    Fixed {
        target_languages: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiConfigRecord {
    pub theme_mode: ThemeMode,
    pub show_tray_icon: bool,
}

impl Default for UiConfigRecord {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::System,
            show_tray_icon: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TriggerConfigRecord {
    pub double_copy_window_ms: u64,
    pub fallback_shortcut: String,
    pub replace_popup_on_new_trigger: bool,
}

impl Default for TriggerConfigRecord {
    fn default() -> Self {
        Self {
            double_copy_window_ms: DEFAULT_DOUBLE_COPY_WINDOW_MS,
            fallback_shortcut: "CmdOrCtrl+Shift+Y".to_string(),
            replace_popup_on_new_trigger: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TranslationConfigRecord {
    pub routing_rule: LanguageRoutingRuleRecord,
    pub user_rules: Option<String>,
    pub preserve_paragraphs: bool,
}

impl Default for TranslationConfigRecord {
    fn default() -> Self {
        Self {
            routing_rule: LanguageRoutingRuleRecord::Branching {
                english_source_language: "en".to_string(),
                english_target_languages: vec!["zh-CN".to_string()],
                chinese_source_language: "zh-CN".to_string(),
                chinese_target_languages: vec!["en".to_string()],
                fallback_target_languages: vec!["en".to_string(), "zh-CN".to_string()],
            },
            user_rules: None,
            preserve_paragraphs: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HistoryConfigRecord {
    pub enabled: bool,
    pub max_items: u32,
    pub store_full_text: bool,
}

impl Default for HistoryConfigRecord {
    fn default() -> Self {
        Self {
            enabled: true,
            max_items: DEFAULT_HISTORY_LIMIT,
            store_full_text: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DebugConfigRecord {
    pub enabled: bool,
    pub log_raw_network_errors: bool,
}

impl Default for DebugConfigRecord {
    fn default() -> Self {
        Self {
            enabled: true,
            log_raw_network_errors: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderConfigRecord {
    pub id: String,
    pub name: String,
    pub kind: ProviderKind,
    pub base_url: String,
    pub path: String,
    pub auth_scheme: AuthScheme,
    pub api_key_ref: Option<String>,
    pub organization: Option<String>,
    pub model: String,
    pub temperature: f64,
    pub top_p: f64,
    pub max_tokens: Option<u32>,
    pub timeout_secs: u64,
    pub custom_headers: Vec<KeyValuePairRecord>,
    pub enabled: bool,
    pub verified_at: Option<u64>,
}

impl Default for ProviderConfigRecord {
    fn default() -> Self {
        Self {
            id: "provider-default".to_string(),
            name: "OpenAI-compatible".to_string(),
            kind: ProviderKind::OpenAiCompatible,
            base_url: String::new(),
            path: DEFAULT_PROVIDER_PATH.to_string(),
            auth_scheme: AuthScheme::Bearer,
            api_key_ref: None,
            organization: None,
            model: String::new(),
            temperature: 0.2,
            top_p: 1.0,
            max_tokens: Some(DEFAULT_MAX_TOKENS),
            timeout_secs: DEFAULT_TIMEOUT_SECS,
            custom_headers: Vec::new(),
            enabled: true,
            verified_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigRecord {
    pub schema_version: u32,
    pub ui: UiConfigRecord,
    pub trigger: TriggerConfigRecord,
    pub providers: Vec<ProviderConfigRecord>,
    pub active_provider_id: Option<String>,
    pub translation: TranslationConfigRecord,
    pub history: HistoryConfigRecord,
    pub debug: DebugConfigRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderDirectoryRecord {
    pub providers: Vec<ProviderConfigRecord>,
    pub active_provider_id: Option<String>,
}

impl Default for AppConfigRecord {
    fn default() -> Self {
        Self {
            schema_version: 1,
            ui: UiConfigRecord::default(),
            trigger: TriggerConfigRecord::default(),
            providers: Vec::new(),
            active_provider_id: None,
            translation: TranslationConfigRecord::default(),
            history: HistoryConfigRecord::default(),
            debug: DebugConfigRecord::default(),
        }
    }
}

impl AppConfigRecord {
    pub fn normalize(mut self) -> Self {
        self.schema_version = 1;
        self.providers = normalize_provider_list(self.providers);

        self.active_provider_id = self.active_provider_id.filter(|provider_id| {
            self.providers
                .iter()
                .any(|provider| provider.id == *provider_id && provider.verified_at.is_some())
        });

        self
    }

    pub fn provider_directory(&self) -> ProviderDirectoryRecord {
        ProviderDirectoryRecord {
            providers: self.providers.clone(),
            active_provider_id: self.active_provider_id.clone(),
        }
    }
}

fn normalize_provider_list(providers: Vec<ProviderConfigRecord>) -> Vec<ProviderConfigRecord> {
    let mut seen = std::collections::BTreeSet::new();

    providers
        .into_iter()
        .map(normalize_provider)
        .filter(|provider| seen.insert(provider.id.clone()))
        .collect()
}

fn normalize_provider(mut provider: ProviderConfigRecord) -> ProviderConfigRecord {
    provider.id = provider.id.trim().to_string();
    provider.name = provider.name.trim().to_string();
    provider.base_url = provider.base_url.trim().to_string();
    provider.path = if provider.path.trim().is_empty() {
        DEFAULT_PROVIDER_PATH.to_string()
    } else {
        provider.path.trim().to_string()
    };
    provider.organization = provider.organization.map(|value| value.trim().to_string());
    provider.model = provider.model.trim().to_string();
    provider.custom_headers = provider
        .custom_headers
        .into_iter()
        .map(|header| KeyValuePairRecord {
            name: header.name.trim().to_string(),
            value: header.value.trim().to_string(),
        })
        .collect();

    provider
}
