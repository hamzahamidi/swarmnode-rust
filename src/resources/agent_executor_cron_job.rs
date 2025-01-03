use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

use crate::utils::client::SwarmClient as Client;
use crate::utils::pagination::PagePaginatedResource;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AgentExecutorCronJob {
    pub id: String,
    pub agent_id: String,
    pub execution_address: String,
    pub created: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AgentExecutorCronJobList {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<AgentExecutorCronJob>,
    pub total_count: u32,
    pub current_page: u32,
}

impl AgentExecutorCronJob {
    pub fn api_source() -> &'static str {
        "agent-executor-cron-jobs"
    }

    pub async fn list(
        agent_id: Option<String>,
        page: Option<u32>,
        page_size: Option<u8>,
    ) -> Result<PagePaginatedResource<AgentExecutorCronJob>, Box<dyn Error>> {
        let mut params = HashMap::new();
        params.insert("page".to_string(), page.unwrap_or(1).to_string());
        params.insert("page_size".to_string(), page_size.unwrap_or(10).to_string());
        if let Some(agent_id) = agent_id {
            params.insert("agent_id".to_string(), agent_id);
        }

        let response = Client::request_action::<AgentExecutorCronJobList>(
            "GET",
            &format!("{}/", Self::api_source()),
            Some(params),
            None,
        )
        .await
        .map_err(|e| format!("Error listing AgentExecutorCronJobs: {}", e))?;

        Ok(PagePaginatedResource {
            next_url: response.next,
            previous_url: response.previous,
            results: response.results,
            total_count: response.total_count,
            current_page: response.current_page,
            resource_class: "AgentExecutorCronJob",
        })
    }

    pub async fn retrieve(id: &str) -> Result<AgentExecutorCronJob, Box<dyn Error>> {
        let agent_executor_cron_job = Client::request_action::<AgentExecutorCronJob>(
            "GET",
            &format!("{}/{}/", Self::api_source(), id),
            None,
            None,
        )
        .await
        .map_err(|e| format!("Error retrieving AgentExecutorCronJob: {}", e))?;

        Ok(agent_executor_cron_job)
    }

    pub async fn create(
        agent_id: &str,
        name: &str,
        expression: &str,
    ) -> Result<AgentExecutorCronJob, Box<dyn Error>> {
        let mut data = HashMap::new();
        data.insert("agent_id".to_string(), agent_id.to_string());
        data.insert("name".to_string(), name.to_string());
        data.insert("expression".to_string(), expression.to_string());

        let agent_executor_cron_job = Client::request_action::<AgentExecutorCronJob>(
            "POST",
            &format!("{}/create/", Self::api_source()),
            None,
            Some(data),
        )
        .await
        .map_err(|e| format!("Error creating AgentExecutorCronJob: {}", e))?;

        Ok(agent_executor_cron_job)
    }

    pub async fn update(
        id: &str,
        name: Option<&str>,
        payload: Option<HashMap<String, Value>>,
    ) -> Result<AgentExecutorCronJob, Box<dyn Error>> {
        let mut data = HashMap::new();
        if let Some(name) = name {
            data.insert("name".to_string(), name.to_string());
        }
        if let Some(payload) = payload {
            for (key, value) in payload {
                data.insert(key, serde_json::to_string(&value).unwrap());
            }
        }

        let agent_executor_cron_job = Client::request_action::<AgentExecutorCronJob>(
            "PATCH",
            &format!("{}/{}/update/", Self::api_source(), id),
            None,
            Some(data),
        )
        .await
        .map_err(|e| format!("Error updating AgentExecutorCronJob: {}", e))?;

        Ok(agent_executor_cron_job)
    }

    pub async fn delete(id: &str) -> Result<(), Box<dyn Error>> {
        Client::request_action::<Value>(
            "DELETE",
            &format!("{}/{}/delete/", Self::api_source(), id),
            None,
            None,
        )
        .await
        .map_err(|e| format!("Error deleting AgentExecutorCronJob: {}", e))?;

        Ok(())
    }

}
