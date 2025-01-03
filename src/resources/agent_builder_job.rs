use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use crate::utils::client::SwarmClient as Client;
use crate::utils::pagination::PagePaginatedResource;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AgentBuilderJob {
    pub id: String,
    pub agent_id: String,
    pub execution_address: String,
    pub created: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AgentBuilderJobList {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<AgentBuilderJob>,
    pub total_count: u32,
    pub current_page: u32,
}

impl AgentBuilderJob {
    pub fn api_source() -> &'static str {
        "agent-builder-jobs"
    }

    pub async fn list(
        agent_id: Option<String>,
        page: Option<u32>,
        page_size: Option<u8>,
    ) -> Result<PagePaginatedResource<AgentBuilderJob>, Box<dyn Error>> {
        let mut params = HashMap::new();
        params.insert("page".to_string(), page.unwrap_or(1).to_string());
        params.insert("page_size".to_string(), page_size.unwrap_or(10).to_string());
        if let Some(agent_id) = agent_id {
            params.insert("agent_id".to_string(), agent_id);
        }

        let response = Client::request_action::<AgentBuilderJobList>(
            "GET",
            &format!("{}/", Self::api_source()),
            Some(params),
            None,
        )
        .await
        .map_err(|e| format!("Error listing AgentBuilderJobs: {}", e))?;

        Ok(PagePaginatedResource {
            next_url: response.next,
            previous_url: response.previous,
            results: response.results,
            total_count: response.total_count,
            current_page: response.current_page,
            resource_class: "AgentBuilderJob",
        })
    }

    pub async fn retrieve(id: &str) -> Result<AgentBuilderJob, Box<dyn Error>> {
        let agent_executor_cron_job = Client::request_action::<AgentBuilderJob>(
            "GET",
            &format!("{}/{}/", Self::api_source(), id),
            None,
            None,
        )
        .await
        .map_err(|e| format!("Error retrieving AgentBuilderJob: {}", e))?;

        Ok(agent_executor_cron_job)
    }
}
