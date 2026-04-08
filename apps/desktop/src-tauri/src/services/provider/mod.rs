use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod openai_compatible;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProviderRequest {
    pub request_id: Option<String>,
    pub model: Option<String>,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub max_tokens: Option<u32>,
    pub timeout_secs: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProviderUsage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderResponse {
    pub request_id: Option<String>,
    pub provider_id: String,
    pub provider_name: String,
    pub model: String,
    pub server_time_ms: Option<u64>,
    pub content: String,
    pub latency_ms: u64,
    pub finish_reason: Option<String>,
    pub usage: Option<ProviderUsage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderErrorCode {
    InvalidBaseUrl,
    InvalidEndpointPath,
    InvalidHeaderName,
    InvalidHeaderValue,
    InvalidHeaderOverride,
    InvalidProviderId,
    MissingApiKey,
    MissingModel,
    InvalidTemperature,
    InvalidTopP,
    InvalidMaxTokens,
    InvalidTimeout,
    UnsupportedAuthScheme,
    RequestBuildFailed,
    RequestFailed,
    HttpStatus,
    ResponseParseFailed,
    EmptyResponse,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderError {
    pub provider_id: String,
    pub code: ProviderErrorCode,
    pub message: String,
    pub status: Option<u16>,
    pub details: Option<String>,
}

impl ProviderError {
    pub fn new(
        provider_id: impl Into<String>,
        code: ProviderErrorCode,
        message: impl Into<String>,
    ) -> Self {
        Self {
            provider_id: provider_id.into(),
            code,
            message: message.into(),
            status: None,
            details: None,
        }
    }

    pub fn with_status(mut self, status: u16) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            Some(status) => write!(
                f,
                "{} (status {status}): {}",
                self.code_string(),
                self.message
            ),
            None => write!(f, "{}: {}", self.code_string(), self.message),
        }
    }
}

impl ProviderError {
    fn code_string(&self) -> &'static str {
        match self.code {
            ProviderErrorCode::InvalidBaseUrl => "invalid_base_url",
            ProviderErrorCode::InvalidEndpointPath => "invalid_endpoint_path",
            ProviderErrorCode::InvalidHeaderName => "invalid_header_name",
            ProviderErrorCode::InvalidHeaderValue => "invalid_header_value",
            ProviderErrorCode::InvalidHeaderOverride => "invalid_header_override",
            ProviderErrorCode::InvalidProviderId => "invalid_provider_id",
            ProviderErrorCode::MissingApiKey => "missing_api_key",
            ProviderErrorCode::MissingModel => "missing_model",
            ProviderErrorCode::InvalidTemperature => "invalid_temperature",
            ProviderErrorCode::InvalidTopP => "invalid_top_p",
            ProviderErrorCode::InvalidMaxTokens => "invalid_max_tokens",
            ProviderErrorCode::InvalidTimeout => "invalid_timeout",
            ProviderErrorCode::UnsupportedAuthScheme => "unsupported_auth_scheme",
            ProviderErrorCode::RequestBuildFailed => "request_build_failed",
            ProviderErrorCode::RequestFailed => "request_failed",
            ProviderErrorCode::HttpStatus => "http_status",
            ProviderErrorCode::ResponseParseFailed => "response_parse_failed",
            ProviderErrorCode::EmptyResponse => "empty_response",
        }
    }
}

impl std::error::Error for ProviderError {}

#[async_trait]
pub trait TranslationProvider: Send + Sync {
    async fn translate(&self, request: ProviderRequest) -> Result<ProviderResponse, ProviderError>;
}
