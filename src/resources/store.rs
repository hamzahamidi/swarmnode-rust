use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

use crate::utils::client::SwarmClient as Client;
use crate::utils::pagination::PagePaginatedResource;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Store {
    pub id: String,
    pub agent_id: String,
    pub store_address: String,
    pub created: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StoreList {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Store>,
    pub total_count: u32,
    pub current_page: u32,
}

impl Store {
    pub fn api_source() -> &'static str {
        "stores"
    }

    pub async fn list(
        agent_id: Option<String>,
        page: Option<u32>,
        page_size: Option<u8>,
    ) -> Result<PagePaginatedResource<Store>, Box<dyn Error>> {
        let mut params = HashMap::new();
        params.insert("page".to_string(), page.unwrap_or(1).to_string());
        params.insert("page_size".to_string(), page_size.unwrap_or(10).to_string());
        if let Some(agent_id) = agent_id {
            params.insert("agent_id".to_string(), agent_id);
        }

        let response = Client::request_action::<StoreList>(
            "GET",
            &format!("{}/", Self::api_source()),
            Some(params),
            None,
        )
        .await
        .map_err(|e| format!("Error listing stores: {}", e))?;

        Ok(PagePaginatedResource {
            next_url: response.next,
            previous_url: response.previous,
            results: response.results,
            total_count: response.total_count,
            current_page: response.current_page,
            resource_class: "Store",
        })
    }

    pub async fn retrieve(id: &str) -> Result<Store, Box<dyn Error>> {
        let store = Client::request_action::<Store>(
            "GET",
            &format!("{}/{}/", Self::api_source(), id),
            None,
            None,
        )
        .await
        .map_err(|e| format!("Error retrieving Store: {}", e))?;

        Ok(store)
    }

    pub async fn create(name: &str) -> Result<Store, Box<dyn Error>> {
        let mut data = HashMap::new();
        data.insert("name".to_string(), name.to_string());

        let agent_executor_cron_job = Client::request_action::<Store>(
            "POST",
            &format!("{}/create/", Self::api_source()),
            None,
            Some(data),
        )
        .await
        .map_err(|e| format!("Error creating Store: {}", e))?;

        Ok(agent_executor_cron_job)
    }

    pub async fn update(
      id: &str,
      payload: Option<HashMap<String, Value>>,
  ) -> Result<Store, Box<dyn Error>> {
      let mut data = HashMap::new();
      if let Some(payload) = payload {
          for (key, value) in payload {
              data.insert(key, serde_json::to_string(&value).unwrap());
          }
      }

      let agent_executor_cron_job = Client::request_action::<Store>(
          "PATCH",
          &format!("{}/{}/update/", Self::api_source(), id),
          None,
          Some(data),
      )
      .await
      .map_err(|e| format!("Error updating Store: {}", e))?;

      Ok(agent_executor_cron_job)
  }

  pub async fn delete(id: &str) -> Result<(), Box<dyn Error>> {
      Client::request_action::<()>(
          "DELETE",
          &format!("{}/{}/delete/", Self::api_source(), id),
          None,
          None,
      )
      .await
      .map_err(|e| format!("Error deleting Store: {}", e))?;

      Ok(())
  }

}
