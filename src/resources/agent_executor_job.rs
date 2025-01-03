use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

use crate::utils::client::SwarmClient as Client;
use crate::utils::pagination::CursorPaginatedResource;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AgentExecutorJob {
    pub id: String,
    pub agent_id: String,
    pub execution_address: String,
    pub created: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AgentExecutorJobList {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<AgentExecutorJob>,
}

impl AgentExecutorJob {
    pub fn api_source() -> &'static str {
        "agent-executor-jobs"
    }

    pub async fn list(
        agent_id: Option<String>,
    ) -> Result<CursorPaginatedResource<AgentExecutorJob>, Box<dyn Error>> {
        let mut params = HashMap::new();
        if let Some(agent_id) = agent_id {
            params.insert("agent_id".to_string(), agent_id);
        }

        let response = Client::request_action::<AgentExecutorJobList>(
            "GET",
            &format!("{}/", Self::api_source()),
            Some(params),
            None,
        )
        .await
        .map_err(|e| format!("Error listing AgentExecutorJobs: {}", e))?;

        Ok(CursorPaginatedResource {
            next_url: response.next,
            previous_url: response.previous,
            results: response.results,
            resource_class: "AgentExecutorJob",
        })
    }

    pub async fn retrieve(id: &str) -> Result<AgentExecutorJob, Box<dyn Error>> {
        let agent_executor_job = Client::request_action::<AgentExecutorJob>(
            "GET",
            &format!("{}/{}/", Self::api_source(), id),
            None,
            None,
        )
        .await
        .map_err(|e| format!("Error retrieving AgentExecutorJob: {}", e))?;

        Ok(agent_executor_job)
    }

    pub async fn create(
        agent_id: &str,
        payload: Option<Value>,
    ) -> Result<AgentExecutorJob, Box<dyn Error>> {
        let mut data = HashMap::new();
        data.insert("agent_id".to_string(), agent_id.to_string());
        if let Some(p) = payload {
            data.insert("payload".to_string(), serde_json::to_string(&p)?);
        }

        let agent_executor_job = Client::request_action::<AgentExecutorJob>(
            "POST",
            &format!("{}/create/", Self::api_source()),
            None,
            Some(data),
        )
        .await
        .map_err(|e| format!("Error creating AgentExecutorJob: {}", e))?;

        Ok(agent_executor_job)
    }
}
