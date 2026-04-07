//! Data models for BioSwarm v2.0
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SwarmResults {
    pub execution_id: String,
    pub timestamp: String,
    pub agent_outputs: HashMap<String, String>,
    pub total_tokens: u64,
    pub duration_ms: u64,
}

#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub name: String,
    pub description: String,
    pub system_prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub published_date: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IntelligenceReport {
    pub title: String,
    pub content: String,
    pub confidence: u8,
    pub sources: Vec<String>,
}
