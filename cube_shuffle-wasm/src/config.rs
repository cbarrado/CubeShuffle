use std::collections::HashMap;

use cube_shuffle_core::distribution_shuffle::Pile;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::window;

const CONFIG_KEY: &str = "cube_shuffle_config";

/// Check if running in Tauri (Windows/desktop app)
/// Returns true if the __TAURI__ global is available
pub fn is_tauri() -> bool {
    if let Some(window) = window() {
        // Check for __TAURI__ or __TAURI_INTERNALS__ global object
        let tauri_check = js_sys::Reflect::get(&window, &JsValue::from_str("__TAURI__"));
        if let Ok(val) = tauri_check {
            if !val.is_undefined() && !val.is_null() {
                return true;
            }
        }
        // Also check for __TAURI_INTERNALS__ (Tauri 2.x)
        let tauri_internals = js_sys::Reflect::get(&window, &JsValue::from_str("__TAURI_INTERNALS__"));
        if let Ok(val) = tauri_internals {
            if !val.is_undefined() && !val.is_null() {
                return true;
            }
        }
    }
    false
}

/// Application configuration that persists between sessions
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub piles: HashMap<String, Pile>,
    pub seed: String,
    pub pack_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            piles: HashMap::new(),
            seed: String::new(),
            pack_size: 15,
        }
    }
}

impl Config {
    /// Load configuration from localStorage
    /// Returns default config if no saved config exists or on error
    pub fn load() -> Self {
        let storage = match get_local_storage() {
            Some(s) => s,
            None => {
                web_sys::console::warn_1(&JsValue::from_str(
                    "localStorage not available, using default config",
                ));
                return Self::default();
            }
        };

        match storage.get_item(CONFIG_KEY) {
            Ok(Some(json)) => match serde_json::from_str::<Config>(&json) {
                Ok(config) => config,
                Err(e) => {
                    web_sys::console::warn_1(&JsValue::from_str(&format!(
                        "Failed to parse saved config: {}, using default",
                        e
                    )));
                    Self::default()
                }
            },
            Ok(None) => Self::default(),
            Err(e) => {
                web_sys::console::warn_1(&JsValue::from_str(&format!(
                    "Failed to load config: {:?}, using default",
                    e
                )));
                Self::default()
            }
        }
    }

    /// Save configuration to localStorage
    pub fn save(&self) {
        let storage = match get_local_storage() {
            Some(s) => s,
            None => {
                web_sys::console::warn_1(&JsValue::from_str(
                    "localStorage not available, cannot save config",
                ));
                return;
            }
        };

        match serde_json::to_string(self) {
            Ok(json) => {
                if let Err(e) = storage.set_item(CONFIG_KEY, &json) {
                    web_sys::console::warn_1(&JsValue::from_str(&format!(
                        "Failed to save config: {:?}",
                        e
                    )));
                }
            }
            Err(e) => {
                web_sys::console::warn_1(&JsValue::from_str(&format!(
                    "Failed to serialize config: {}",
                    e
                )));
            }
        }
    }

    /// Reset configuration to defaults and clear from localStorage
    pub fn reset() {
        if let Some(storage) = get_local_storage() {
            let _ = storage.remove_item(CONFIG_KEY);
        }
    }
}

/// Get localStorage handle
fn get_local_storage() -> Option<web_sys::Storage> {
    window()?.local_storage().ok()?
}
