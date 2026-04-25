use serde_json::Value;
use std::collections::HashMap;
use tauri::AppHandle;

/// Default configuration values matching the original Electron app
fn defaults() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("zoomFactor", "1.0");
    m.insert("homepageUrl", "https://outlook.live.com/mail");
    m.insert("windowFrameWidth", "1400");
    m.insert("windowFrameHeight", "900");
    m.insert("windowFrameX", "100");
    m.insert("windowFrameY", "100");
    m.insert("showWindowFrame", "true");
    m.insert("verticalAdsClass", "pBKjV");
    m.insert("smallAdsClass", "X1Kvq");
    m.insert("premiumAdsClass", "VPtFl");
    m.insert("unreadMessageClass", "");
    m
}

pub fn get_config(app: AppHandle, key: String, default: Option<String>) -> Result<String, String> {
    let store = tauri_plugin_store::StoreBuilder::new(&app, "Settings")
        .build()
        .map_err(|e| e.to_string())?;

    let defaults = defaults();
    let fallback = default
        .as_deref()
        .or_else(|| defaults.get(key.as_str()).copied())
        .unwrap_or("");

    match store.get(&key) {
        Some(value) => {
            if let Some(s) = value.as_str() {
                Ok(s.to_string())
            } else {
                Ok(value.to_string())
            }
        }
        None => Ok(fallback.to_string()),
    }
}

pub fn set_config(app: AppHandle, key: String, value: String) -> Result<(), String> {
    let store = tauri_plugin_store::StoreBuilder::new(&app, "Settings")
        .build()
        .map_err(|e| e.to_string())?;

    store.set(key, Value::String(value));
    store.save().map_err(|e| e.to_string())
}

pub fn delete_config(app: AppHandle, key: String) -> Result<(), String> {
    let store = tauri_plugin_store::StoreBuilder::new(&app, "Settings")
        .build()
        .map_err(|e| e.to_string())?;

    store.delete(&key);
    store.save().map_err(|e| e.to_string())
}

pub fn get_configs(app: AppHandle) -> Result<Value, String> {
    let store = tauri_plugin_store::StoreBuilder::new(&app, "Settings")
        .build()
        .map_err(|e| e.to_string())?;

    let mut result = serde_json::Map::new();
    let defaults = defaults();

    // Start with defaults
    for (k, v) in &defaults {
        result.insert(k.to_string(), Value::String(v.to_string()));
    }

    // Override with stored values
    for (key, value) in store.entries() {
        result.insert(key.clone(), value.clone());
    }

    Ok(Value::Object(result))
}
