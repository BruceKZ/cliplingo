use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::config::{AuthScheme, KeyValuePairRecord, ProviderConfigRecord};

use super::{
    ChatMessage, ProviderError, ProviderErrorCode, ProviderRequest, ProviderResponse,
    ProviderUsage, TranslationProvider,
};

const DEFAULT_CHAT_COMPLETIONS_PATH: &str = "/chat/completions";
const OPENAI_ORGANIZATION_HEADER: &str = "OpenAI-Organization";
const MIN_TIMEOUT_SECS: u64 = 3;
const MAX_TIMEOUT_SECS: u64 = 120;
const MIN_TEMPERATURE: f64 = 0.0;
const MAX_TEMPERATURE: f64 = 2.0;
const MIN_TOP_P: f64 = 0.0;
const MAX_TOP_P: f64 = 1.0;

#[derive(Debug, Clone)]
pub struct OpenAiCompatibleProvider {
    provider_id: String,
    provider_name: String,
    endpoint: Url,
    auth_scheme: AuthScheme,
    api_key: Option<String>,
    organization: Option<String>,
    default_model: Option<String>,
    default_temperature: f64,
    default_top_p: f64,
    default_max_tokens: Option<u32>,
    default_timeout_secs: u64,
    custom_headers: HeaderMap,
    client: Client,
}

impl OpenAiCompatibleProvider {
    pub fn try_from_config(
        config: ProviderConfigRecord,
        api_key: Option<String>,
    ) -> Result<Self, ProviderError> {
        let provider_id = normalized_provider_id(&config.id)?;
        let provider_name = normalized_name(&config.name, &provider_id);
        let endpoint = build_endpoint(&provider_id, &config.base_url, &config.path)?;
        let custom_headers = build_custom_headers(&provider_id, &config.custom_headers)?;
        let default_model = normalized_optional_string(&config.model);
        let default_temperature = validate_temperature(&provider_id, config.temperature)?;
        let default_top_p = validate_top_p(&provider_id, config.top_p)?;
        let default_max_tokens = config.max_tokens;
        let default_timeout_secs = validate_timeout_secs(&provider_id, config.timeout_secs)?;
        let client = Client::builder().build().map_err(|error| {
            provider_error(
                &provider_id,
                ProviderErrorCode::RequestBuildFailed,
                "failed to build reqwest client",
            )
            .with_details(error.to_string())
        })?;

        Ok(Self {
            provider_id,
            provider_name,
            endpoint,
            auth_scheme: config.auth_scheme,
            api_key: api_key.filter(|value| !value.trim().is_empty()),
            organization: normalized_optional_string_option(config.organization),
            default_model,
            default_temperature,
            default_top_p,
            default_max_tokens,
            default_timeout_secs,
            custom_headers,
            client,
        })
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    fn resolve_settings(
        &self,
        request: &ProviderRequest,
    ) -> Result<ResolvedRequestSettings, ProviderError> {
        let model = request
            .model
            .as_deref()
            .and_then(non_empty_string)
            .or_else(|| self.default_model.as_deref())
            .ok_or_else(|| {
                provider_error(
                    &self.provider_id,
                    ProviderErrorCode::MissingModel,
                    "model is required",
                )
            })?
            .to_string();

        let temperature = validate_temperature(
            &self.provider_id,
            request.temperature.unwrap_or(self.default_temperature),
        )?;
        let top_p = validate_top_p(
            &self.provider_id,
            request.top_p.unwrap_or(self.default_top_p),
        )?;
        let max_tokens = match request.max_tokens.or(self.default_max_tokens) {
            Some(value) => Some(validate_max_tokens(&self.provider_id, value)?),
            None => None,
        };
        let timeout_secs = validate_timeout_secs(
            &self.provider_id,
            request.timeout_secs.unwrap_or(self.default_timeout_secs),
        )?;

        Ok(ResolvedRequestSettings {
            model,
            temperature,
            top_p,
            max_tokens,
            timeout_secs,
        })
    }

    fn build_request_body(
        &self,
        request: &ProviderRequest,
        settings: &ResolvedRequestSettings,
    ) -> OpenAiChatCompletionRequest {
        OpenAiChatCompletionRequest {
            model: settings.model.clone(),
            messages: request.messages.clone(),
            temperature: settings.temperature,
            top_p: settings.top_p,
            max_tokens: settings.max_tokens,
        }
    }
}

#[async_trait::async_trait]
impl TranslationProvider for OpenAiCompatibleProvider {
    async fn translate(&self, request: ProviderRequest) -> Result<ProviderResponse, ProviderError> {
        if request.messages.is_empty() {
            return Err(provider_error(
                &self.provider_id,
                ProviderErrorCode::RequestBuildFailed,
                "at least one chat message is required",
            ));
        }

        let settings = self.resolve_settings(&request)?;
        let request_body = self.build_request_body(&request, &settings);
        let mut http_request = self.client.post(self.endpoint.clone()).json(&request_body);
        http_request = http_request.timeout(Duration::from_secs(settings.timeout_secs));

        http_request = match self.auth_scheme {
            AuthScheme::Bearer => {
                let api_key = self.api_key.as_deref().ok_or_else(|| {
                    provider_error(
                        &self.provider_id,
                        ProviderErrorCode::MissingApiKey,
                        "bearer auth requires an API key",
                    )
                })?;
                http_request.bearer_auth(api_key)
            }
            AuthScheme::None => http_request,
        };

        if let Some(organization) = self.organization.as_deref() {
            http_request = http_request.header(OPENAI_ORGANIZATION_HEADER, organization);
        }

        for (name, value) in &self.custom_headers {
            http_request = http_request.header(name, value);
        }

        let started_at = Instant::now();
        let response = http_request.send().await.map_err(|error| {
            provider_error(
                &self.provider_id,
                ProviderErrorCode::RequestFailed,
                "request failed",
            )
            .with_details(error.to_string())
        })?;

        let latency_ms = started_at.elapsed().as_millis() as u64;
        let status = response.status();
        let response_text = response.text().await.map_err(|error| {
            provider_error(
                &self.provider_id,
                ProviderErrorCode::ResponseParseFailed,
                "failed to read provider response",
            )
            .with_details(error.to_string())
        })?;

        if !status.is_success() {
            return Err(provider_error(
                &self.provider_id,
                ProviderErrorCode::HttpStatus,
                format!("provider returned HTTP {status}"),
            )
            .with_status(status.as_u16())
            .with_details(response_text));
        }

        let parsed: OpenAiChatCompletionResponse =
            serde_json::from_str(&response_text).map_err(|error| {
                provider_error(
                    &self.provider_id,
                    ProviderErrorCode::ResponseParseFailed,
                    "failed to parse OpenAI-compatible response",
                )
                .with_details(error.to_string())
            })?;

        let choice = parsed.choices.into_iter().find_map(|choice| {
            choice
                .message
                .content
                .map(|content| (content, choice.finish_reason))
        });

        let Some((content, finish_reason)) = choice else {
            return Err(provider_error(
                &self.provider_id,
                ProviderErrorCode::EmptyResponse,
                "provider response did not contain translated text",
            ));
        };

        let content = content.trim().to_string();
        if content.is_empty() {
            return Err(provider_error(
                &self.provider_id,
                ProviderErrorCode::EmptyResponse,
                "provider response contained empty content",
            ));
        }

        Ok(ProviderResponse {
            request_id: request.request_id,
            provider_id: self.provider_id.clone(),
            provider_name: self.provider_name.clone(),
            model: parsed.model.unwrap_or(settings.model),
            content,
            latency_ms,
            finish_reason,
            usage: parsed.usage.map(|usage| ProviderUsage {
                prompt_tokens: usage.prompt_tokens,
                completion_tokens: usage.completion_tokens,
                total_tokens: usage.total_tokens,
            }),
        })
    }
}

#[derive(Debug, Serialize)]
struct OpenAiChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f64,
    top_p: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChatCompletionResponse {
    model: Option<String>,
    choices: Vec<OpenAiChoice>,
    usage: Option<OpenAiUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAiMessage {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAiUsage {
    prompt_tokens: Option<u32>,
    completion_tokens: Option<u32>,
    total_tokens: Option<u32>,
}

#[derive(Debug, Clone)]
struct ResolvedRequestSettings {
    model: String,
    temperature: f64,
    top_p: f64,
    max_tokens: Option<u32>,
    timeout_secs: u64,
}

fn provider_error(
    provider_id: &str,
    code: ProviderErrorCode,
    message: impl Into<String>,
) -> ProviderError {
    ProviderError::new(provider_id.to_string(), code, message)
}

fn normalized_provider_id(provider_id: &str) -> Result<String, ProviderError> {
    let trimmed = provider_id.trim();
    if trimmed.is_empty() {
        return Err(provider_error(
            provider_id,
            ProviderErrorCode::InvalidProviderId,
            "provider id must not be empty",
        ));
    }

    Ok(trimmed.to_string())
}

fn normalized_name(name: &str, fallback: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        fallback.to_string()
    } else {
        trimmed.to_string()
    }
}

fn normalized_optional_string(value: &str) -> Option<String> {
    non_empty_string(value).map(ToOwned::to_owned)
}

fn normalized_optional_string_option(value: Option<String>) -> Option<String> {
    value.and_then(|value| non_empty_string(&value).map(ToOwned::to_owned))
}

fn non_empty_string(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

fn build_endpoint(provider_id: &str, base_url: &str, path: &str) -> Result<Url, ProviderError> {
    let normalized_base = base_url.trim();
    if normalized_base.is_empty() {
        return Err(provider_error(
            provider_id,
            ProviderErrorCode::InvalidBaseUrl,
            "base URL is required",
        ));
    }

    let parsed_base = Url::parse(normalized_base).map_err(|error| {
        provider_error(
            provider_id,
            ProviderErrorCode::InvalidBaseUrl,
            "base URL is invalid",
        )
        .with_details(error.to_string())
    })?;

    let scheme = parsed_base.scheme();
    let host = parsed_base.host_str().unwrap_or_default();
    let is_local_http = scheme == "http" && is_loopback_host(host);
    if scheme != "https" && !is_local_http {
        return Err(provider_error(
            provider_id,
            ProviderErrorCode::InvalidBaseUrl,
            "base URL must use https, or http only for localhost/loopback development endpoints",
        ));
    }

    let normalized_path = if path.trim().is_empty() {
        DEFAULT_CHAT_COMPLETIONS_PATH
    } else {
        path.trim()
    };
    let endpoint = parsed_base
        .join(normalized_path.trim_start_matches('/'))
        .map_err(|error| {
            provider_error(
                provider_id,
                ProviderErrorCode::InvalidEndpointPath,
                "endpoint path is invalid",
            )
            .with_details(error.to_string())
        })?;

    Ok(endpoint)
}

fn is_loopback_host(host: &str) -> bool {
    if host.eq_ignore_ascii_case("localhost") {
        return true;
    }

    host.parse::<std::net::IpAddr>()
        .map(|ip_addr| ip_addr.is_loopback())
        .unwrap_or(false)
}

fn build_custom_headers(
    provider_id: &str,
    headers: &[KeyValuePairRecord],
) -> Result<HeaderMap, ProviderError> {
    let mut header_map = HeaderMap::new();
    let mut seen_names = HashSet::new();

    for header in headers {
        let name = header.name.trim();
        let value = header.value.trim();

        if name.is_empty() {
            return Err(provider_error(
                provider_id,
                ProviderErrorCode::InvalidHeaderName,
                "custom header name is required",
            ));
        }

        if value.is_empty() {
            return Err(provider_error(
                provider_id,
                ProviderErrorCode::InvalidHeaderValue,
                format!("custom header '{name}' has an empty value"),
            ));
        }

        let lower_name = name.to_ascii_lowercase();
        if !seen_names.insert(lower_name.clone()) {
            return Err(provider_error(
                provider_id,
                ProviderErrorCode::InvalidHeaderOverride,
                format!("custom header '{name}' is duplicated"),
            ));
        }

        if is_blocked_header_name(&lower_name) {
            return Err(provider_error(
                provider_id,
                ProviderErrorCode::InvalidHeaderOverride,
                format!("custom header '{name}' is not allowed"),
            ));
        }

        if value.contains('\r') || value.contains('\n') {
            return Err(provider_error(
                provider_id,
                ProviderErrorCode::InvalidHeaderValue,
                format!("custom header '{name}' contains line breaks"),
            ));
        }

        let header_name = HeaderName::from_bytes(name.as_bytes()).map_err(|error| {
            provider_error(
                provider_id,
                ProviderErrorCode::InvalidHeaderName,
                format!("custom header '{name}' is invalid"),
            )
            .with_details(error.to_string())
        })?;
        let header_value = HeaderValue::from_str(value).map_err(|error| {
            provider_error(
                provider_id,
                ProviderErrorCode::InvalidHeaderValue,
                format!("custom header '{name}' has an invalid value"),
            )
            .with_details(error.to_string())
        })?;

        header_map.insert(header_name, header_value);
    }

    Ok(header_map)
}

fn is_blocked_header_name(name: &str) -> bool {
    matches!(
        name,
        "authorization"
            | "connection"
            | "content-length"
            | "content-type"
            | "host"
            | "proxy-authorization"
            | "transfer-encoding"
            | "upgrade"
    )
}

fn validate_temperature(provider_id: &str, value: f64) -> Result<f64, ProviderError> {
    if !value.is_finite() || !(MIN_TEMPERATURE..=MAX_TEMPERATURE).contains(&value) {
        return Err(provider_error(
            provider_id,
            ProviderErrorCode::InvalidTemperature,
            "temperature must be between 0 and 2",
        ));
    }

    Ok(value)
}

fn validate_top_p(provider_id: &str, value: f64) -> Result<f64, ProviderError> {
    if !value.is_finite() || !(MIN_TOP_P..=MAX_TOP_P).contains(&value) {
        return Err(provider_error(
            provider_id,
            ProviderErrorCode::InvalidTopP,
            "top_p must be between 0 and 1",
        ));
    }

    Ok(value)
}

fn validate_max_tokens(provider_id: &str, value: u32) -> Result<u32, ProviderError> {
    if value == 0 {
        return Err(provider_error(
            provider_id,
            ProviderErrorCode::InvalidMaxTokens,
            "max_tokens must be greater than zero",
        ));
    }

    Ok(value)
}

fn validate_timeout_secs(provider_id: &str, value: u64) -> Result<u64, ProviderError> {
    if !(MIN_TIMEOUT_SECS..=MAX_TIMEOUT_SECS).contains(&value) {
        return Err(provider_error(
            provider_id,
            ProviderErrorCode::InvalidTimeout,
            "timeout must be between 3 and 120 seconds",
        ));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{Read, Write},
        net::TcpListener,
        sync::{Arc, Mutex},
        thread,
    };

    use crate::{
        models::config::{AuthScheme, ProviderConfigRecord},
        services::provider::ChatRole,
    };

    fn provider_config(base_url: String) -> ProviderConfigRecord {
        ProviderConfigRecord {
            id: "provider-1".to_string(),
            name: "OpenAI-compatible".to_string(),
            kind: crate::models::config::ProviderKind::OpenAiCompatible,
            base_url,
            path: DEFAULT_CHAT_COMPLETIONS_PATH.to_string(),
            auth_scheme: AuthScheme::Bearer,
            api_key_ref: Some("provider/provider-1".to_string()),
            organization: Some("org-123".to_string()),
            model: "gpt-4o-mini".to_string(),
            temperature: 0.3,
            top_p: 0.95,
            max_tokens: Some(256),
            timeout_secs: 5,
            custom_headers: vec![KeyValuePairRecord {
                name: "X-Test".to_string(),
                value: "value".to_string(),
            }],
            enabled: true,
        }
    }

    fn start_test_server(
        response_body: &'static str,
    ) -> (String, Arc<Mutex<Option<String>>>, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
        let addr = listener.local_addr().expect("local addr");
        let captured_request = Arc::new(Mutex::new(None));
        let captured_request_clone = Arc::clone(&captured_request);

        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept request");
            let mut request = Vec::new();
            let mut buffer = [0u8; 1024];

            loop {
                let read = stream.read(&mut buffer).expect("read request");
                if read == 0 {
                    break;
                }
                request.extend_from_slice(&buffer[..read]);
                if request.windows(4).any(|window| window == b"\r\n\r\n") {
                    break;
                }
            }

            let header_end = request
                .windows(4)
                .position(|window| window == b"\r\n\r\n")
                .expect("headers end");
            let headers = String::from_utf8(request[..header_end].to_vec()).expect("utf8 headers");
            let content_length = headers
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    if name.trim().eq_ignore_ascii_case("content-length") {
                        Some(value.trim().parse::<usize>().ok())
                    } else {
                        None
                    }
                })
                .flatten()
                .expect("content length");
            let mut body = request[header_end + 4..].to_vec();

            while body.len() < content_length {
                let read = stream.read(&mut buffer).expect("read body");
                if read == 0 {
                    break;
                }
                body.extend_from_slice(&buffer[..read]);
            }

            let full_request = format!(
                "{}\r\n\r\n{}",
                headers,
                String::from_utf8(body.clone()).expect("utf8 body")
            );
            *captured_request_clone.lock().expect("lock request") = Some(full_request);

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            stream
                .write_all(response.as_bytes())
                .expect("write response");
        });

        (format!("http://{}", addr), captured_request, handle)
    }

    #[test]
    fn try_from_config_rejects_invalid_base_url() {
        let mut config = provider_config("ftp://example.com".to_string());
        config.custom_headers.clear();

        let error = OpenAiCompatibleProvider::try_from_config(config, Some("sk-test".to_string()))
            .expect_err("invalid base url");

        assert_eq!(error.code, ProviderErrorCode::InvalidBaseUrl);
    }

    #[test]
    fn try_from_config_rejects_header_injection_and_blocked_headers() {
        let mut config = provider_config("https://example.com".to_string());
        config.custom_headers = vec![KeyValuePairRecord {
            name: "Host".to_string(),
            value: "evil.example.com".to_string(),
        }];

        let error = OpenAiCompatibleProvider::try_from_config(config, Some("sk-test".to_string()))
            .expect_err("blocked host header");

        assert_eq!(error.code, ProviderErrorCode::InvalidHeaderOverride);

        let mut config = provider_config("https://example.com".to_string());
        config.custom_headers = vec![KeyValuePairRecord {
            name: "X-Test".to_string(),
            value: "bad\nvalue".to_string(),
        }];

        let error = OpenAiCompatibleProvider::try_from_config(config, Some("sk-test".to_string()))
            .expect_err("header newline injection");

        assert_eq!(error.code, ProviderErrorCode::InvalidHeaderValue);
    }

    #[tokio::test]
    async fn translate_sends_expected_payload_and_parses_response() {
        let (base_url, captured_request, server_handle) = start_test_server(
            r#"{
                "model": "gpt-4o-mini",
                "choices": [
                    {
                        "message": { "content": "translated output" },
                        "finish_reason": "stop"
                    }
                ],
                "usage": {
                    "prompt_tokens": 10,
                    "completion_tokens": 5,
                    "total_tokens": 15
                }
            }"#,
        );

        let provider = OpenAiCompatibleProvider::try_from_config(
            provider_config(base_url),
            Some("sk-test".to_string()),
        )
        .expect("provider");

        let response = provider
            .translate(ProviderRequest {
                request_id: Some("req-1".to_string()),
                model: None,
                messages: vec![
                    ChatMessage {
                        role: ChatRole::System,
                        content: "You are a translator.".to_string(),
                    },
                    ChatMessage {
                        role: ChatRole::User,
                        content: "Hello".to_string(),
                    },
                ],
                temperature: Some(0.4),
                top_p: Some(0.9),
                max_tokens: Some(128),
                timeout_secs: Some(5),
            })
            .await
            .expect("response");

        server_handle.join().expect("server thread");

        assert_eq!(response.request_id.as_deref(), Some("req-1"));
        assert_eq!(response.provider_id, "provider-1");
        assert_eq!(response.provider_name, "OpenAI-compatible");
        assert_eq!(response.model, "gpt-4o-mini");
        assert_eq!(response.content, "translated output");
        assert_eq!(response.finish_reason.as_deref(), Some("stop"));
        assert_eq!(
            response.usage,
            Some(ProviderUsage {
                prompt_tokens: Some(10),
                completion_tokens: Some(5),
                total_tokens: Some(15),
            })
        );

        let captured = captured_request
            .lock()
            .expect("request")
            .clone()
            .expect("captured");
        let captured_lower = captured.to_ascii_lowercase();
        assert!(captured_lower.contains("post /chat/completions http/1.1"));
        assert!(captured_lower.contains("authorization: bearer sk-test"));
        assert!(captured_lower.contains("openai-organization: org-123"));
        assert!(captured_lower.contains("x-test: value"));
        assert!(captured_lower.contains("\"model\":\"gpt-4o-mini\""));
        assert!(captured_lower.contains("\"temperature\":0.4"));
        assert!(captured_lower.contains("\"top_p\":0.9"));
        assert!(captured_lower.contains("\"max_tokens\":128"));
        assert!(captured_lower.contains("\"role\":\"system\""));
        assert!(captured_lower.contains("\"role\":\"user\""));
    }

    #[tokio::test]
    async fn translate_rejects_missing_api_key_for_bearer_auth() {
        let mut config = provider_config("https://example.com".to_string());
        config.custom_headers.clear();

        let provider = OpenAiCompatibleProvider::try_from_config(config, None).expect("provider");
        let error = provider
            .translate(ProviderRequest {
                messages: vec![ChatMessage {
                    role: ChatRole::User,
                    content: "Hello".to_string(),
                }],
                ..ProviderRequest::default()
            })
            .await
            .expect_err("missing api key");

        assert_eq!(error.code, ProviderErrorCode::MissingApiKey);
    }

    #[tokio::test]
    async fn translate_surfaces_http_status_errors() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
        let addr = listener.local_addr().expect("local addr");

        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept request");
            let mut request = [0u8; 1024];
            let _ = stream.read(&mut request).expect("read request");
            let body = "{\"error\":\"unauthorized\"}";
            let response = format!(
                "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream
                .write_all(response.as_bytes())
                .expect("write response");
        });

        let provider = OpenAiCompatibleProvider::try_from_config(
            provider_config(format!("http://{addr}")),
            Some("sk-test".to_string()),
        )
        .expect("provider");

        let error = provider
            .translate(ProviderRequest {
                messages: vec![ChatMessage {
                    role: ChatRole::User,
                    content: "Hello".to_string(),
                }],
                ..ProviderRequest::default()
            })
            .await
            .expect_err("http status error");

        handle.join().expect("server thread");

        assert_eq!(error.code, ProviderErrorCode::HttpStatus);
        assert_eq!(error.status, Some(401));
        assert!(error
            .details
            .as_deref()
            .unwrap_or_default()
            .contains("unauthorized"));
    }
}
