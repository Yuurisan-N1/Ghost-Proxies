use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::StatusCode;
use serde_json::Value;

use crate::net::egress::Proxy;
use crate::support::consts::net::{CONNECT_TIMEOUT_SECS, READ_TIMEOUT_SECS};
use crate::support::device::Device;
use crate::support::error::{AppError, AppResult};

#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    base: String,
    device: Device,
}

impl Client {
    pub fn open(base: &str, proxy: Option<&Proxy>, device: &Device) -> AppResult<Client> {
        let mut builder = reqwest::Client::builder()
            .cookie_store(true)
            .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT_SECS))
            .timeout(Duration::from_secs(READ_TIMEOUT_SECS))
            .user_agent(device.agent.clone());

        if let Some(proxy) = proxy {
            builder = builder.proxy(
                reqwest::Proxy::all(proxy.url())
                    .map_err(|_| AppError::network("the proxy could not be used"))?,
            );
        }

        let http = builder
            .build()
            .map_err(|_| AppError::network("the http client could not be built"))?;

        Ok(Client {
            http,
            base: base.trim_end_matches('/').to_string(),
            device: device.clone(),
        })
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub async fn get(&self, path: &str) -> AppResult<(StatusCode, Value)> {
        let url = self.url(path);
        let reply = self
            .http
            .get(url)
            .headers(self.headers(path))
            .send()
            .await?;
        read(reply).await
    }

    pub async fn post(&self, path: &str, body: &Value) -> AppResult<(StatusCode, Value)> {
        let url = self.url(path);
        let reply = self
            .http
            .post(url)
            .headers(self.headers(path))
            .json(body)
            .send()
            .await?;
        read(reply).await
    }

    fn url(&self, path: &str) -> String {
        if path.starts_with("http") {
            path.to_string()
        } else {
            format!("{}{}", self.base, path)
        }
    }

    fn headers(&self, path: &str) -> HeaderMap {
        let mut map = HeaderMap::new();
        let device = &self.device;
        insert(&mut map, "accept", "application/json, text/plain, */*");
        insert(&mut map, "accept-language", &device.language);
        insert(&mut map, "origin", &self.base);
        insert(&mut map, "referer", &format!("{}{}", self.base, path));
        insert(&mut map, "sec-fetch-site", "same-origin");
        insert(&mut map, "sec-fetch-mode", "cors");
        insert(&mut map, "sec-fetch-dest", "empty");
        if !device.brands.is_empty() {
            insert(&mut map, "sec-ch-ua", &device.brands);
        }
        if !device.mobile.is_empty() {
            insert(&mut map, "sec-ch-ua-mobile", &device.mobile);
        }
        if !device.platform.is_empty() {
            insert(&mut map, "sec-ch-ua-platform", &device.platform);
        }
        map
    }
}

async fn read(reply: reqwest::Response) -> AppResult<(StatusCode, Value)> {
    let status = reply.status();
    let text = reply.text().await.unwrap_or_default();
    let value: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
    Ok((status, value))
}

fn insert(map: &mut HeaderMap, key: &str, value: &str) {
    if let (Ok(name), Ok(value)) = (
        HeaderName::from_bytes(key.as_bytes()),
        HeaderValue::from_str(value),
    ) {
        map.insert(name, value);
    }
}

pub fn message_of(value: &Value, text: &str) -> String {
    for key in ["error", "message", "detail"] {
        if let Some(found) = value.get(key).and_then(Value::as_str) {
            if !found.trim().is_empty() {
                return found.to_string();
            }
        }
    }
    text.to_string()
}

pub fn code_of(value: &Value) -> String {
    value
        .get("code")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_ascii_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_message_prefers_the_error_field() {
        let value: Value = serde_json::json!({"error": "Unauthorized", "message": "other"});
        assert_eq!(message_of(&value, "raw"), "Unauthorized");
    }

    #[test]
    fn the_message_falls_back_to_the_raw_body() {
        assert_eq!(message_of(&Value::Null, "plain text"), "plain text");
    }

    #[test]
    fn an_empty_field_is_skipped() {
        let value: Value = serde_json::json!({"error": "  ", "message": "kept"});
        assert_eq!(message_of(&value, "raw"), "kept");
    }

    #[test]
    fn the_code_is_upper_case() {
        let value: Value = serde_json::json!({"code": "email_not_verified"});
        assert_eq!(code_of(&value), "EMAIL_NOT_VERIFIED");
    }

    #[test]
    fn a_missing_code_reads_empty() {
        assert_eq!(code_of(&serde_json::json!({})), "");
    }
}
