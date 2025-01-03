use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use crate::utils::client::SwarmClient as Client;
use crate::utils::pagination::PagePaginatedResource;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Build {
    pub id: String,
    pub agent_id: String,
    pub build_address: String,
    pub created: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BuildList {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Build>,
    pub total_count: u32,
    pub current_page: u32,
}

impl Build {
    pub fn api_source() -> &'static str {
        "builds"
    }

    pub async fn list(
        agent_executor_job_id: Option<String>,
        page: Option<u32>,
        page_size: Option<u8>,
    ) -> Result<PagePaginatedResource<Build>, Box<dyn Error>> {
        let mut params = HashMap::new();
        params.insert("page".to_string(), page.unwrap_or(1).to_string());
        params.insert("page_size".to_string(), page_size.unwrap_or(10).to_string());
        if let Some(agent_executor_job_id) = agent_executor_job_id {
            params.insert("agent_executor_job_id".to_string(), agent_executor_job_id);
        }

        let response = Client::request_action::<BuildList>(
            "GET",
            &format!("{}/", Self::api_source()),
            Some(params),
            None,
        )
        .await
        .map_err(|e| format!("Error listing Builds: {}", e))?;

        Ok(PagePaginatedResource {
            next_url: response.next,
            previous_url: response.previous,
            results: response.results,
            total_count: response.total_count,
            current_page: response.current_page,
            resource_class: "Build",
        })
    }

    pub async fn retrieve(id: &str) -> Result<Build, Box<dyn Error>> {
        let build = Client::request_action::<Build>(
            "GET",
            &format!("{}/{}/", Self::api_source(), id),
            None,
            None,
        )
        .await
        .map_err(|e| format!("Error retrieving Build: {}", e))?;

        Ok(build)
    }
}
