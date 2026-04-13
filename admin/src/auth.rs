use gloo_net::http::Request;
use serde::Serialize;

/// Wrapper around gloo_net::http::Request.
/// Authentication is now handled automatically via HttpOnly cookies.
pub async fn auth_fetch(
    url: &str,
    method: &str,
    body: Option<impl Serialize>,
) -> Result<gloo_net::http::Response, String> {
    let request = match method {
        "GET" => Request::get(url),
        "POST" => Request::post(url),
        "PUT" => Request::put(url),
        "DELETE" => Request::delete(url),
        _ => return Err(format!("Unsupported method: {}", method)),
    };

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
