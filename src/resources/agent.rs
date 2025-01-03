use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

use crate::utils::client::SwarmClient as Client;
use crate::utils::pagination::PagePaginatedResource;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Agent {
    pub id: String,
    pub agent_id: String,
    pub execution_address: String,
    pub created: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AgentList {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Agent>,
    pub total_count: u32,
    pub current_page: u32,
}

impl Agent {
    pub fn api_source() -> &'static str {
        "agents"
    }

    pub async fn list(
        page: Option<u32>,
        page_size: Option<u8>,
    ) -> Result<PagePaginatedResource<Agent>, Box<dyn Error>> {
        let mut params = HashMap::new();
        params.insert("page".to_string(), page.unwrap_or(1).to_string());
        params.insert("page_size".to_string(), page_size.unwrap_or(10).to_string());

        let response = Client::request_action::<AgentList>(
            "GET",
            &format!("{}/", Self::api_source()),
            Some(params),
            None,
        )
        .await
        .map_err(|e| format!("Error listing Agents: {}", e))?;

        Ok(PagePaginatedResource {
            next_url: response.next,
            previous_url: response.previous,
            results: response.results,
            total_count: response.total_count,
            current_page: response.current_page,
            resource_class: "Agent",
        })
    }

    pub async fn retrieve(id: &str) -> Result<Agent, Box<dyn Error>> {
        let agent_executor_cron_job = Client::request_action::<Agent>(
            "GET",
            &format!("{}/{}/", Self::api_source(), id),
            None,
            None,
        )
        .await
        .map_err(|e| format!("Error retrieving Agent: {}", e))?;

        Ok(agent_executor_cron_job)
    }

    pub async fn create(
        name: &str,
        script: &str,
        python_version: &str,
        store_id: &str,
        requirements: Option<&str>,
        env_vars: Option<&str>,
    ) -> Result<Agent, Box<dyn Error>> {
        let mut data = HashMap::new();
        data.insert("name".to_string(), name.to_string());
        data.insert("script".to_string(), script.to_string());
        data.insert("python_version".to_string(), python_version.to_string());
        data.insert("store_id".to_string(), store_id.to_string());
        if let Some(requirements) = requirements {
            data.insert("requirements".to_string(), requirements.to_string());
        }
        if let Some(env_vars) = env_vars {
            data.insert("env_vars".to_string(), env_vars.to_string());
        }

        let agent = Client::request_action::<Agent>(
            "POST",
            &format!("{}/create/", Self::api_source()),
            None,
            Some(data),
        )
        .await
        .map_err(|e| format!("Error creating Agent: {}", e))?;

        Ok(agent)
    }

    pub async fn update(
        id: &str,
        name: Option<&str>,
        script: Option<&str>,
        python_version: Option<&str>,
        store_id: Option<&str>,
        requirements: Option<&str>,
        env_vars: Option<&str>,
    ) -> Result<Agent, Box<dyn Error>> {
        let mut data = HashMap::new();
        if let Some(name) = name {
            data.insert("name".to_string(), name.to_string());
        }
        if let Some(script) = script {
            data.insert("script".to_string(), script.to_string());
        }
        if let Some(python_version) = python_version {
            data.insert("python_version".to_string(), python_version.to_string());
        }
        if let Some(store_id) = store_id {
            data.insert("store_id".to_string(), store_id.to_string());
        }
        if let Some(requirements) = requirements {
            data.insert("requirements".to_string(), requirements.to_string());
        }
        if let Some(env_vars) = env_vars {
            data.insert("env_vars".to_string(), env_vars.to_string());
        }

        let agent = Client::request_action::<Agent>(
            "PATCH",
            &format!("{}/{}/update/", Self::api_source(), id),
            None,
            Some(data),
        )
        .await
        .map_err(|e| format!("Error updating Agent: {}", e))?;

        Ok(agent)
    }

    pub async fn delete(id: &str) -> Result<(), Box<dyn Error>> {
        Client::request_action::<Value>(
            "DELETE",
            &format!("{}/{}/delete/", Self::api_source(), id),
            None,
            None,
        )
        .await
        .map_err(|e| format!("Error deleting Agent: {}", e))?;

        Ok(())
    }

}
