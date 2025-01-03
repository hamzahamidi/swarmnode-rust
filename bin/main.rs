use serde_json::json;
use std::error::Error;
use swarmnode::{resources::agent_executor_job::AgentExecutorJob, set_config, SwarmNodeConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set configuration
    set_config(SwarmNodeConfig {
        api_key: Some("your_api_key".to_string()),
        api_base: None,
    });

    // Example: List AgentExecutorJobs
    println!("Fetching agent executor jobs...");
    match AgentExecutorJob::list(None).await {
        Ok(result) => {
            println!("AgentExecutorJobs fetched successfully:");
            for job in result.results {
                println!("{:?}", job);
            }
        }
        Err(e) => {
            println!("Failed to fetch AgentExecutorJobs: {}", e);
        }
    }

    // Example: Create a new AgentExecutorJob
    println!("Creating a new agent executor job...");
    let payload = json!({
        "command": "echo 'Hello, World!'",
        "env": {
            "FOO": "BAR"
        }
    });
    match AgentExecutorJob::create("agent_id_example", Some(payload)).await {
        Ok(job) => {
            println!("AgentExecutorJob created successfully: {:?}", job);
        }
        Err(e) => {
            println!("Failed to create AgentExecutorJob: {}", e);
        }
    }

    // Example: Retrieve a specific AgentExecutorJob by ID
    println!("Retrieving an agent executor job...");
    match AgentExecutorJob::retrieve("job_id_example").await {
        Ok(job) => {
            println!("AgentExecutorJob retrieved successfully: {:?}", job);
        }
        Err(e) => {
            println!("Failed to retrieve AgentExecutorJob: {}", e);
        }
    }

    Ok(())
}
