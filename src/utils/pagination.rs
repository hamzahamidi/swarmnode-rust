use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;

use super::client::SwarmClient as Client;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CursorPaginatedResource<T> {
    pub next_url: Option<String>,
    pub previous_url: Option<String>,
    pub resource_class: &'static str,
    pub results: Vec<T>,
}

impl<T> CursorPaginatedResource<T>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    pub async fn next(&self) -> Option<Self> {
        if self.next_url.is_none() {
            return None;
        }

        let url = self.next_url.as_ref().unwrap();
        let response = Client::request_url("GET", url, None).await.unwrap(); // handle error properly

        let json: Value = response.json().await.unwrap(); // handle error properly
        let next_url = json["next"].as_str().map(|s| s.to_string());
        let previous_url = json["previous"].as_str().map(|s| s.to_string());

        let results: Vec<T> = serde_json::from_value(json["results"].clone()).unwrap(); // handle error properly

        Some(CursorPaginatedResource {
            next_url,
            previous_url: previous_url,
            resource_class: self.resource_class,
            results,
        })
    }

    pub async fn previous(&self) -> Option<Self> {
        if self.previous_url.is_none() {
            return None;
        }

        let url = self.previous_url.as_ref().unwrap();
        let response = Client::request_url("GET", url, None).await.unwrap(); // handle error properly

        let json: Value = response.json().await.unwrap(); // handle error properly
        let next_url = json["next"].as_str().map(|s| s.to_string());
        let previous_url = json["previous"].as_str().map(|s| s.to_string());

        let results: Vec<T> = serde_json::from_value(json["results"].clone()).unwrap(); // handle error properly

        Some(CursorPaginatedResource {
            next_url,
            previous_url: previous_url,
            resource_class: self.resource_class,
            results,
        })
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for CursorPaginatedResource<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}(results={:?})",
            std::any::type_name::<Self>(),
            self.results
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PagePaginatedResource<T> {
    pub next_url: Option<String>,
    pub previous_url: Option<String>,
    pub resource_class: &'static str, // Class name as a string
    pub total_count: u32,
    pub current_page: u32,
    pub results: Vec<T>,
}

impl<T> PagePaginatedResource<T>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    pub async fn next(&self) -> Option<Self> {
        if self.next_url.is_none() {
            return None;
        }

        let url = self.next_url.as_ref().unwrap();
        let response = Client::request_url("GET", url, None).await.unwrap(); // handle error properly

        let json: Value = response.json().await.unwrap(); // handle error properly
        let next_url = json["next"].as_str().map(|s| s.to_string());
        let previous_url = json["previous"].as_str().map(|s| s.to_string());

        let total_count = json["total_count"].as_u64().unwrap_or(0) as u32;
        let current_page = json["current_page"].as_u64().unwrap_or(0) as u32;

        let results: Vec<T> = serde_json::from_value(json["results"].clone()).unwrap(); // handle error properly

        Some(PagePaginatedResource {
            next_url: next_url,
            previous_url: previous_url,
            resource_class: self.resource_class,
            total_count,
            current_page,
            results,
        })
    }

    pub async fn previous(&self) -> Option<Self> {
        if self.previous_url.is_none() {
            return None;
        }

        let url = self.previous_url.as_ref().unwrap();
        let response = Client::request_url("GET", url, None).await.unwrap(); // handle error properly

        let json: Value = response.json().await.unwrap(); // handle error properly
        let next_url = json["next"].as_str().map(|s| s.to_string());
        let previous_url = json["previous"].as_str().map(|s| s.to_string());

        let total_count = json["total_count"].as_u64().unwrap_or(0) as u32;
        let current_page = json["current_page"].as_u64().unwrap_or(0) as u32;

        let results: Vec<T> = serde_json::from_value(json["results"].clone()).unwrap(); // handle error properly

        Some(PagePaginatedResource {
            next_url: next_url,
            previous_url: previous_url,
            resource_class: self.resource_class,
            total_count,
            current_page,
            results,
        })
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for PagePaginatedResource<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}(total_count={}, current_page={}, results={:?})",
            std::any::type_name::<Self>(),
            self.total_count,
            self.current_page,
            self.results
        )
    }
}
