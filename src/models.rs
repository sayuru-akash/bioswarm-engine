use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub source: String,
    pub published_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub summary: String,
    pub confidence: u8,
    pub action_items: Vec<String>,
    pub sources: Vec<SearchResult>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOutput {
    pub agent_name: String,
    pub query_type: String,
    pub content: String,
    pub confidence: u8,
    pub duration_ms: u64,
    pub recursive_depth: u8,
    pub sources: Vec<SearchResult>,
    pub insights: Vec<Insight>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmResults {
    pub execution_id: String,
    pub timestamp: DateTime<Utc>,
    pub query: String,
    pub agent_outputs: BTreeMap<String, AgentOutput>,
    pub total_tokens: u64,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trend {
    pub current_value: f64,
    pub previous_value: Option<f64>,
    pub change_percent: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub trends: BTreeMap<String, Trend>,
    pub delta_summary: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunMetadata {
    pub query: String,
    pub agent_count: usize,
    pub formats: Vec<String>,
    pub output_dir: String,
    pub resumed: bool,
    pub backend: String,
    pub model: String,
    pub api_base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedReport {
    pub title: String,
    pub execution_id: String,
    pub timestamp: DateTime<Utc>,
    pub summary: String,
    pub trends: TrendAnalysis,
    pub action_items: Vec<String>,
    pub charts: BTreeMap<String, String>,
    pub agent_outputs: BTreeMap<String, AgentOutput>,
    pub export_formats: Vec<String>,
    pub confidence_score: u8,
    pub metadata: RunMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub execution_id: String,
    pub query: String,
    pub completed_agents: Vec<String>,
    pub remaining_agents: Vec<String>,
    pub partial_results: BTreeMap<String, AgentOutput>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSummary {
    pub execution_id: String,
    pub timestamp: DateTime<Utc>,
    pub query: String,
    pub status: String,
    pub duration_ms: u64,
    pub total_tokens: u64,
    pub confidence_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplateConfig {
    pub name: String,
    pub include_trends: bool,
    pub include_action_items: bool,
    pub include_charts: bool,
    pub include_sources: bool,
}
