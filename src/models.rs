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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnhancedReport {
    pub title: String,
    pub execution_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub summary: String,
    pub trends: TrendAnalysis,
    pub action_items: Vec<String>,
    pub charts: std::collections::HashMap<String, String>,
    pub agent_outputs: std::collections::HashMap<String, String>,
    pub export_formats: Vec<String>,
}

impl EnhancedReport {
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str(&format!("# {}\n\n", self.title));
        md.push_str(&self.summary);
        md.push('\n');
        
        for (name, output) in &self.agent_outputs {
            md.push_str(&format!("## {}\n\n{}\n\n", name, output));
        }
        
        md
    }
    
    pub fn to_html(&self) -> String {
        // Convert markdown to HTML
        format!("<html><body>{}</body></html>", self.to_markdown().replace('\n', "<br>"))
    }
    
    pub fn to_csv(&self) -> String {
        // Create CSV from data
        "Agent,Output\n".to_string() + 
        &self.agent_outputs.iter()
            .map(|(k, v)| format!("{},\"{}\"", k, v.replace('"', "\"\"")))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrendAnalysis {
    pub trends: std::collections::HashMap<String, Trend>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Trend {
    pub current_value: f64,
    pub change_percent: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Checkpoint {
    pub execution_id: String,
    pub completed_agents: Vec<String>,
    pub remaining_agents: Vec<String>,
    pub partial_results: std::collections::HashMap<String, String>,
}
