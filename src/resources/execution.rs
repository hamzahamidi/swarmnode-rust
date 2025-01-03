use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use crate::utils::client::SwarmClient as Client;
use crate::utils::pagination::CursorPaginatedResource;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Execution {
    pub id: String,
    pub agent_id: String,
    pub execution_address: String,
    pub created: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExecutionList {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Execution>,
}

impl Execution {
    pub fn api_source() -> &'static str {
        "executions"
    }

    pub async fn list(
        agent_id: Option<String>,
        agent_executor_job_id: Option<String>,
        agent_executor_cron_job_id: Option<String>,
    ) -> Result<CursorPaginatedResource<Execution>, Box<dyn Error>> {
        let mut params = HashMap::new();
        if let Some(agent_id) = agent_id {
            params.insert("agent_id".to_string(), agent_id);
        }
        if let Some(agent_executor_job_id) = agent_executor_job_id {
            params.insert("agent_executor_job_id".to_string(), agent_executor_job_id);
        }
        if let Some(agent_executor_cron_job_id) = agent_executor_cron_job_id {
            params.insert(
                "agent_executor_cron_job_id".to_string(),
                agent_executor_cron_job_id,
            );
        }

        let response = Client::request_action::<ExecutionList>(
            "GET",
            &format!("{}/", Self::api_source()),
            Some(params),
            None,
        )
        .await
        .map_err(|e| format!("Error listing Executions: {}", e))?;

        Ok(CursorPaginatedResource {
            next_url: response.next,
            previous_url: response.previous,
            results: response.results,
            resource_class: "Execution",
        })
    }

    pub async fn retrieve(id: &str) -> Result<Execution, Box<dyn Error>> {
        let execution = Client::request_action::<Execution>(
            "GET",
            &format!("{}/{}/", Self::api_source(), id),
            None,
            None,
        )
        .await
        .map_err(|e| format!("Error retrieving Execution: {}", e))?;

        Ok(execution)
    }
}
