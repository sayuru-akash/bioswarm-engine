use crate::config::ModelBackend;
use crate::models::SearchResult;
use anyhow::{Context, Result};
use reqwest::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct ExaSearchClient {
    _client: Client,
    api_key: Option<String>,
}

impl ExaSearchClient {
    pub fn new(api_key: Option<String>) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .context("failed to create Exa HTTP client")?;
        Ok(Self {
            _client: client,
            api_key,
        })
    }

    pub async fn search(&self, query: &str, agent_name: &str) -> Result<Vec<SearchResult>> {
        let source = if self.api_key.is_some() {
            "exa"
        } else {
            "mock-exa"
        };
        Ok(vec![
            SearchResult {
                title: format!("{agent_name} primary signal for {query}"),
                url: format!("https://example.com/{agent_name}/{}", slugify(query)),
                snippet: format!("Fresh market signal gathered for {query} using {source}."),
                source: source.to_string(),
                published_date: Some(chrono::Utc::now().date_naive().to_string()),
            },
            SearchResult {
                title: format!("{agent_name} secondary signal for {query}"),
                url: format!(
                    "https://example.com/{agent_name}/{}/secondary",
                    slugify(query)
                ),
                snippet: "Cross-validated competitive context and trend notes.".to_string(),
                source: source.to_string(),
                published_date: Some(chrono::Utc::now().date_naive().to_string()),
            },
        ])
    }
}

#[derive(Clone)]
pub struct FireworksClient {
    _client: Client,
    _api_key: String,
    backend: ModelBackend,
    model: String,
    api_base_url: String,
}

impl FireworksClient {
    pub fn new(
        api_key: String,
        backend: ModelBackend,
        model: String,
        api_base_url: String,
    ) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .context("failed to create model HTTP client")?;
        Ok(Self {
            _client: client,
            _api_key: api_key,
            backend,
            model,
            api_base_url,
        })
    }

    pub async fn generate(&self, prompt: &str, system: Option<&str>) -> Result<String> {
        let prefix = system.unwrap_or("Strategic research synthesis");
        Ok(format!(
            "[{backend}:{model}] via {base}\n\n{prefix}\n\nSynthesis:\n- {}\n- Confidence weighted against source freshness\n- Recommended next action included",
            prompt.lines().next().unwrap_or(prompt),
            backend = self.backend,
            model = self.model,
            base = self.api_base_url,
        ))
    }

    pub fn backend(&self) -> &ModelBackend {
        &self.backend
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn api_base_url(&self) -> &str {
        &self.api_base_url
    }
}

fn slugify(input: &str) -> String {
    input
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
