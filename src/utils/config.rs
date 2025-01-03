use std::env;
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    // Default API base URL
    pub static ref API_BASE: RwLock<String> = RwLock::new("api.swarmnode.ai".to_string());

    // API key, fetched from environment variable or manually set
    pub static ref API_KEY: RwLock<Option<String>> = RwLock::new(None);
}

// Define a struct for the configuration
pub struct SwarmNodeConfig {
    pub api_key: Option<String>,
    pub api_base: Option<String>,
}

// Function to initialize the API key from the environment (if available)
pub fn initialize_api_key_from_env() {
    let api_key = env::var("SWARMNODE_API_KEY").ok();
    if api_key.is_some() {
        set_api_key(&api_key.unwrap());
    }
}

// Set the API base URL
pub fn set_api_base(base: &str) {
    let mut api_base = API_BASE.write().unwrap();
    *api_base = base.to_string();
}

// Get the API base URL
pub fn get_api_base() -> String {
    let api_base = API_BASE.read().unwrap();
    api_base.clone()
}

// Set the API key manually
pub fn set_api_key(key: &str) {
    let mut api_key = API_KEY.write().unwrap();
    *api_key = Some(key.to_string());
}

// Get the API key
pub fn get_api_key() -> Option<String> {
    let api_key = API_KEY.read().unwrap();
    api_key.clone()
}

// Set configuration using a struct
pub fn set_config(config: SwarmNodeConfig) {
    if let Some(key) = config.api_key {
        set_api_key(&key);
    }
    if let Some(base) = config.api_base {
        set_api_base(&base);
    }

    // Optionally, initialize API key from the environment if it's not set manually
    initialize_api_key_from_env();
}
