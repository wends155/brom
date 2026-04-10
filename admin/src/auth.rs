use gloo_net::http::Request;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthContext {
    pub token: RwSignal<Option<String>>,
}

/// Helper to get the token from localStorage
pub fn get_token_from_storage() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    storage.get_item("admin_token").ok()?
}

/// Helper to save the token to localStorage
pub fn save_token_to_storage(token: &str) {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.set_item("admin_token", token);
    }
}

/// Wrapper around gloo_net::http::Request that injects the Bearer token.
pub async fn auth_fetch(
    url: &str,
    method: &str,
    body: Option<impl Serialize>,
) -> Result<gloo_net::http::Response, String> {
    let mut request = match method {
        "GET" => Request::get(url),
        "POST" => Request::post(url),
        "PUT" => Request::put(url),
        "DELETE" => Request::delete(url),
        _ => return Err(format!("Unsupported method: {}", method)),
    };

    if let Some(token) = get_token_from_storage() {
        request = request.header("Authorization", &format!("Bearer {}", token));
    }

    let result = if let Some(body) = body {
        match request.json(&body) {
            Ok(req) => req.send().await,
            Err(e) => return Err(e.to_string()),
        }
    } else {
        request.send().await
    };

    result.map_err(|e| e.to_string())
}
