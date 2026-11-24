use serde_json::Value;

use crate::net::client::{message_of, Client};
use crate::support::consts::app::{KEY_LABEL, KEY_PREFIX};
use crate::support::consts::net::PATH_KEYS;
use crate::support::error::{refusal, AppError, AppResult};

#[derive(Debug, Clone)]
pub struct Key {
    pub value: String,
    pub label: String,
    pub created_at: String,
}

pub async fn fetch(client: &Client) -> AppResult<Vec<Key>> {
    let (status, value) = client.get(PATH_KEYS).await?;
    if !status.is_success() {
        return Err(AppError::api(refusal(&message_of(&value, ""))));
    }
    let rows = match value {
        Value::Array(rows) => rows,
        Value::Object(ref map) => match map.get("keys") {
            Some(Value::Array(rows)) => rows.clone(),
            _ => Vec::new(),
        },
        _ => Vec::new(),
    };
    Ok(rows.iter().filter_map(read).collect())
}

pub fn default_of(keys: &[Key]) -> Option<Key> {
    keys.iter()
        .find(|key| key.label.eq_ignore_ascii_case(KEY_LABEL))
        .cloned()
        .or_else(|| keys.first().cloned())
}

fn read(row: &Value) -> Option<Key> {
    let value = row.get("key").and_then(Value::as_str)?.trim().to_string();
    if !value.starts_with(KEY_PREFIX) {
        return None;
    }
    Some(Key {
        value,
        label: row
            .get("label")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
        created_at: row
            .get("created_at")
            .or_else(|| row.get("createdAt"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_label_is_chosen() {
        let keys = vec![
            Key {
                value: "gp_other".to_string(),
                label: "bot".to_string(),
                created_at: String::new(),
            },
            Key {
                value: "gp_default".to_string(),
                label: "Default Private Key".to_string(),
                created_at: String::new(),
            },
        ];
        assert_eq!(default_of(&keys).unwrap().value, "gp_default");
    }

    #[test]
    fn the_first_key_is_used_when_no_default_exists() {
        let keys = vec![Key {
            value: "gp_only".to_string(),
            label: "custom".to_string(),
            created_at: String::new(),
        }];
        assert_eq!(default_of(&keys).unwrap().value, "gp_only");
    }

    #[test]
    fn no_keys_reads_none() {
        assert!(default_of(&[]).is_none());
    }

    #[test]
    fn a_row_without_the_prefix_is_skipped() {
        let row: Value = serde_json::json!({"key": "other_123", "label": "x"});
        assert!(read(&row).is_none());
    }

    #[test]
    fn a_row_keeps_the_created_time() {
        let row: Value = serde_json::json!({"key": "gp_abc", "label": "x", "created_at": "2026-01-01T00:00:00Z"});
        assert_eq!(read(&row).unwrap().created_at, "2026-01-01T00:00:00Z");
    }

    #[test]
    fn the_camel_case_time_is_read_too() {
        let row: Value =
            serde_json::json!({"key": "gp_abc", "label": "x", "createdAt": "2026-02-02T00:00:00Z"});
        assert_eq!(read(&row).unwrap().created_at, "2026-02-02T00:00:00Z");
    }
}
