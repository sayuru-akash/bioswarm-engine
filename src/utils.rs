use crate::models::{AgentOutput, EnhancedReport, SwarmResults, TrendAnalysis};
use anyhow::Result;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub async fn generate_executive_summary(
    results: &SwarmResults,
    trends: &TrendAnalysis,
) -> Result<String> {
    let avg_conf = average_confidence(results);
    let top_agents = results
        .agent_outputs
        .values()
        .take(3)
        .map(|output| format!("{} ({})", output.agent_name, output.confidence))
        .collect::<Vec<_>>()
        .join(", ");
    let top_agents = if top_agents.is_empty() {
        "none".to_string()
    } else {
        top_agents
    };

    let mut out = format!(
        "## Executive summary\n\n- Query: {}\n- Agents completed: {}\n- Agents failed: {}\n- Duration: {} ms\n- Confidence score: {}\n- Strongest contributors: {}\n",
        results.query,
        results.agent_outputs.len(),
        results.failed_agents.len(),
        results.duration_ms,
        avg_conf,
        top_agents
    );

    if !results.failed_agents.is_empty() {
        out.push_str("\n### Failures\n");
        for failure in results.failed_agents.iter().take(5) {
            out.push_str(&format!("- {}: {}\n", failure.agent_name, failure.error));
        }
    }

    if !trends.delta_summary.is_empty() {
        out.push_str("\n### Trend deltas\n");
        for line in &trends.delta_summary {
            out.push_str(&format!("- {}\n", line));
        }
    }

    Ok(out)
}

pub async fn extract_action_items(results: &SwarmResults) -> Result<Vec<String>> {
    let mut unique = BTreeSet::new();
    for output in results.agent_outputs.values() {
        for insight in &output.insights {
            for action in &insight.action_items {
                unique.insert(action.clone());
            }
        }
    }
    Ok(unique.into_iter().collect())
}

pub async fn generate_ascii_charts(results: &SwarmResults) -> Result<BTreeMap<String, String>> {
    let mut charts = BTreeMap::new();
    let mut confidence = String::from("Confidence by agent\n\n");
    for output in results.agent_outputs.values() {
        let bars = "#".repeat((output.confidence / 5).max(1) as usize);
        confidence.push_str(&format!(
            "{:<22} {} {}\n",
            output.agent_name, bars, output.confidence
        ));
    }
    charts.insert("confidence".to_string(), confidence);
    Ok(charts)
}

pub fn deduplicate_agent_outputs(
    outputs: BTreeMap<String, AgentOutput>,
) -> BTreeMap<String, AgentOutput> {
    let mut seen = BTreeSet::new();
    let mut deduped = BTreeMap::new();
    for (key, value) in outputs {
        let hash = hash_content(&value.content);
        if seen.insert(hash) {
            deduped.insert(key, value);
        }
    }
    deduped
}

pub fn confidence_score(results: &SwarmResults) -> u8 {
    average_confidence(results)
}

fn average_confidence(results: &SwarmResults) -> u8 {
    if results.agent_outputs.is_empty() {
        return 0;
    }
    let total: u64 = results
        .agent_outputs
        .values()
        .map(|output| output.confidence as u64)
        .sum();
    (total / results.agent_outputs.len() as u64) as u8
}

fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

impl EnhancedReport {
    pub fn to_markdown(&self) -> String {
        let mut out = format!("# {}\n\n{}\n\n", self.title, self.summary);
        if !self.trends.trends.is_empty() {
            out.push_str("## Trends\n\n");
            for (name, trend) in &self.trends.trends {
                out.push_str(&format!(
                    "- **{}** current {:.2}, change {:.1}%\n",
                    name, trend.current_value, trend.change_percent
                ));
            }
            out.push('\n');
        }
        if !self.action_items.is_empty() {
            out.push_str("## Action items\n\n");
            for item in &self.action_items {
                out.push_str(&format!("- {}\n", item));
            }
            out.push('\n');
        }
        if !self.agent_outputs.is_empty() {
            out.push_str("## Agent findings\n\n");
        }
        for output in self.agent_outputs.values() {
            out.push_str(&format!(
                "### {}\n\n{}\n\n",
                output.agent_name, output.content
            ));
        }
        out
    }

    pub fn to_html(&self) -> String {
        format!(
            "<html><body><pre>{}</pre></body></html>",
            self.to_markdown()
        )
    }

    pub fn to_csv(&self) -> String {
        let mut writer = csv::Writer::from_writer(vec![]);
        writer
            .write_record(["agent_name", "confidence", "depth", "content"])
            .unwrap();
        for output in self.agent_outputs.values() {
            writer
                .write_record([
                    output.agent_name.as_str(),
                    &output.confidence.to_string(),
                    &output.recursive_depth.to_string(),
                    output.content.as_str(),
                ])
                .unwrap();
        }
        String::from_utf8(writer.into_inner().unwrap()).unwrap()
    }
}
