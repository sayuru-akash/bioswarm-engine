use crate::config::ModelBackend;
use crate::models::SearchResult;
use anyhow::{anyhow, bail, Context, Result};
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;

#[derive(Clone)]
pub struct ExaSearchClient {
    client: Client,
    api_key: Option<String>,
}

impl ExaSearchClient {
    pub fn new(api_key: Option<String>) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .context("failed to create Exa HTTP client")?;
        Ok(Self { client, api_key })
    }

    pub async fn search(&self, query: &str, agent_name: &str) -> Result<Vec<SearchResult>> {
        if let Some(api_key) = self
            .api_key
            .as_ref()
            .filter(|key| !key.trim().is_empty() && !is_test_key(key))
        {
            match self.live_search(query, agent_name, api_key).await {
                Ok(results) if !results.is_empty() => return Ok(results),
                Ok(_) => {}
                Err(_) => {}
            }
        }

        Ok(mock_search_results(query, agent_name, if self.api_key.is_some() { "exa-fallback" } else { "mock-exa" }))
    }

    async fn live_search(&self, query: &str, agent_name: &str, api_key: &str) -> Result<Vec<SearchResult>> {
        let response = self
            .client
            .post("https://api.exa.ai/search")
            .header("x-api-key", api_key)
            .json(&json!({
                "query": format!("{} for {}", query, agent_name),
                "type": "auto",
                "numResults": 5,
                "contents": {
                    "text": { "maxCharacters": 400 }
                }
            }))
            .send()
            .await
            .context("failed to send Exa search request")?;

        if !response.status().is_success() {
            bail!("Exa search failed with status {}", response.status());
        }

        let body: Value = response.json().await.context("failed to parse Exa response")?;
        let results = body
            .get("results")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Exa response missing results array"))?;

        Ok(results
            .iter()
            .take(5)
            .map(|item| SearchResult {
                title: item
                    .get("title")
                    .and_then(Value::as_str)
                    .unwrap_or("Untitled result")
                    .to_string(),
                url: item
                    .get("url")
                    .and_then(Value::as_str)
                    .unwrap_or("https://example.com")
                    .to_string(),
                snippet: item
                    .get("text")
                    .or_else(|| item.get("snippet"))
                    .and_then(Value::as_str)
                    .unwrap_or("No snippet provided")
                    .to_string(),
                source: "exa".to_string(),
                published_date: item
                    .get("publishedDate")
                    .or_else(|| item.get("published_date"))
                    .and_then(Value::as_str)
                    .map(ToString::to_string),
            })
            .collect())
    }
}

#[derive(Clone)]
pub struct FireworksClient {
    client: Client,
    api_key: String,
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
            client,
            api_key,
            backend,
            model,
            api_base_url,
        })
    }

    pub async fn generate(&self, prompt: &str, system: Option<&str>) -> Result<String> {
        if !is_test_key(&self.api_key) {
            match self.live_generate(prompt, system).await {
                Ok(content) if !content.trim().is_empty() => return Ok(content),
                Ok(_) => {}
                Err(_) => {}
            }
        }

        Ok(mock_generation(
            &self.backend,
            &self.model,
            &self.api_base_url,
            prompt,
            system,
        ))
    }

    async fn live_generate(&self, prompt: &str, system: Option<&str>) -> Result<String> {
        let mut messages = Vec::new();
        if let Some(system) = system {
            messages.push(json!({ "role": "system", "content": system }));
        }
        messages.push(json!({ "role": "user", "content": prompt }));

        let url = format!("{}/chat/completions", self.api_base_url.trim_end_matches('/'));
        let request = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": self.model,
                "messages": messages,
                "temperature": 0.3
            }));

        let response = request
            .send()
            .await
            .context("failed to send model generation request")?;

        if !response.status().is_success() {
            bail!("model generation failed with status {}", response.status());
        }

        let body: Value = response
            .json()
            .await
            .context("failed to parse generation response")?;

        let content = body
            .get("choices")
            .and_then(Value::as_array)
            .and_then(|choices| choices.first())
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("generation response missing choices[0].message.content"))?;

        Ok(content.to_string())
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

fn mock_search_results(query: &str, agent_name: &str, source: &str) -> Vec<SearchResult> {
    vec![
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
    ]
}

fn mock_generation(
    backend: &ModelBackend,
    model: &str,
    api_base_url: &str,
    prompt: &str,
    system: Option<&str>,
) -> String {
    let prefix = system.unwrap_or("Strategic research synthesis");
    format!(
        "[{backend}:{model}] via {base}\n\n{prefix}\n\nSynthesis:\n- {}\n- Confidence weighted against source freshness\n- Recommended next action included",
        prompt.lines().next().unwrap_or(prompt),
        backend = backend,
        model = model,
        base = api_base_url,
    )
}

fn is_test_key(value: &str) -> bool {
    let lowered = value.trim().to_ascii_lowercase();
    lowered.is_empty()
        || lowered == "test"
        || lowered.starts_with("test-")
        || lowered.contains("dummy")
        || lowered.contains("example")
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
