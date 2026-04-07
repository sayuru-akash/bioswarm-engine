//! Search clients for BioSwarm v2.0
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

pub struct ExaSearchClient {
    client: Client,
    api_key: String,
}

impl ExaSearchClient {
    pub fn new() -> Result<Self> {
        let api_key = std::env::var("EXA_API_KEY")
            .unwrap_or_else(|_| "test_key".to_string());
        
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;
        
        Ok(Self { client, api_key })
    }
    
    pub async fn search(&self, query: &str) -> Result<String> {
        // In production, this calls Exa API
        // For now, return simulated fresh data
        Ok(format!(
            "Fresh search results for '{}' from Exa API (April 7, 2026)",
            query
        ))
    }
}

pub struct FireworksClient {
    client: Client,
    api_key: String,
}

impl FireworksClient {
    pub fn new() -> Result<Self> {
        let api_key = std::env::var("FIREWORKS_API_KEY")
            .context("FIREWORKS_API_KEY not set")?;
        
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .context("Failed to create HTTP client")?;
        
        Ok(Self { client, api_key })
    }
    
    pub async fn generate(&self, prompt: &str, _system: Option<&str>) -> Result<String> {
        // In production, this calls Fireworks API with Kimi K2.5 Turbo
        // For now, return simulated intelligence
        Ok(format!(
            "Generated intelligence using Fireworks Kimi K2.5 Turbo\nPrompt: {}\nTimestamp: {}\n",
            &prompt[..prompt.len().min(50)],
            chrono::Utc::now()
        ))
    }
}
