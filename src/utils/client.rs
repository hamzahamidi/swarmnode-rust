use crate::{get_api_base, get_api_key};
use async_stream::stream;
use futures_util::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client as ReqwestClient;
use reqwest::Response;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use tokio_tungstenite::connect_async;

#[derive(Debug)]
pub enum SwarmNodeError {
    BadRequest(String),
    Unauthenticated(String),
    NotFound(String),
    ApiKeyNotSet,
    Other(String),
}

impl fmt::Display for SwarmNodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SwarmNodeError::BadRequest(ref msg) => write!(f, "Bad Request: {}", msg),
            SwarmNodeError::Unauthenticated(ref msg) => write!(f, "Unauthenticated: {}", msg),
            SwarmNodeError::NotFound(ref msg) => write!(f, "Not Found: {}", msg),
            SwarmNodeError::ApiKeyNotSet => write!(f, "API Key not set"),
            SwarmNodeError::Other(ref msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

impl SwarmNodeError {
    fn from_response(response: &Response) -> Self {
        let status = response.status();
        let status_code = status.as_u16();
        let status_text = status.canonical_reason().unwrap_or("Unknown status");

        SwarmNodeError::Other(format!("HTTP {}: {}", status_code, status_text))
    }
}

pub struct SwarmClient;

impl SwarmClient {
    fn get_http_headers() -> HeaderMap {
        let api_key = get_api_key().expect("API key not set!"); // Expecting the API key to be set
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );
        headers
    }

    fn get_ws_headers() -> Vec<(String, String)> {
        let api_key = get_api_key().expect("API key not set!"); // Expecting the API key to be set
        vec![("Authorization".to_string(), format!("Bearer {}", api_key))]
    }

    pub async fn request_action<T: for<'de> Deserialize<'de>>(
        method: &str,
        action_path: &str,
        params: Option<HashMap<String, String>>,
        data: Option<HashMap<String, String>>,
    ) -> Result<T, SwarmNodeError> {
        let client = ReqwestClient::new();
        let url = format!("https://{}/v1/{}", get_api_base(), action_path);
        let mut request = client
            .request(method.parse().unwrap(), &url)
            .headers(SwarmClient::get_http_headers());

        if let Some(p) = params {
            request = request.query(&p);
        }

        if let Some(d) = data {
            request = request.json(&d);
        }

        let response = request
            .send()
            .await
            .map_err(|e| SwarmNodeError::Other(format!("Request failed: {}", e)))?;

        if response.status().is_success() {
            // Deserialize the response body into the type T
            response
                .json::<T>()
                .await
                .map_err(|e| SwarmNodeError::Other(format!("Failed to parse response body: {}", e)))
        } else {
            Err(SwarmNodeError::from_response(&response))
        }
    }

    pub async fn request_url(
        method: &str,
        url: &str,
        data: Option<HashMap<String, String>>,
    ) -> Result<reqwest::Response, SwarmNodeError> {
        let client = ReqwestClient::new();
        let mut request = client
            .request(method.parse().unwrap(), url)
            .headers(SwarmClient::get_http_headers());

        if let Some(d) = data {
            request = request.json(&d);
        }

        let response = request
            .send()
            .await
            .map_err(|e| SwarmNodeError::Other(format!("Request failed: {}", e)))?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(SwarmNodeError::from_response(&response))
        }
    }

    // Listen to a specific execution via WebSocket
    pub async fn listen_to_execution(address: &str) -> Result<String, Box<dyn Error>> {
        let api_base = get_api_base();
        let headers = SwarmClient::get_ws_headers();
        let url = format!("wss://{}/ws/v1/execution/{}/", api_base, address);

        let (mut ws_stream, _) = connect_async(url)
            .await
            .expect("Failed to connect to WebSocket");

        let message = ws_stream.next().await.unwrap()?;
        Ok(message.to_string())
    }

    // Listen to execution stream via WebSocket
    pub async fn listen_to_execution_stream(
        address: &str,
    ) -> impl futures_util::Stream<Item = Result<String, Box<dyn Error>>> {
        let api_base = get_api_base();
        let headers = SwarmClient::get_ws_headers();
        let url = format!("wss://{}/ws/v1/execution-stream/{}/", api_base, address);

        let (mut ws_stream, _) = connect_async(url)
            .await
            .expect("Failed to connect to WebSocket");

        stream! {
            while let Some(message) = ws_stream.next().await {
                yield Ok(message?.to_string());
            }
        }
    }
}
