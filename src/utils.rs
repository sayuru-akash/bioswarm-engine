//! Utility functions for BioSwarm v3.0
use anyhow::Result;
use std::collections::HashMap;

pub async fn generate_executive_summary(
    results: &crate::orchestrator::SwarmResults,
    trends: &crate::models::TrendAnalysis,
) -> Result<String> {
    let mut summary = String::new();
    
    summary.push_str(&format!(
        "## Executive Summary\n\n\
        **Execution ID:** {}\n\
        **Agents Deployed:** {}\n\
        **Duration:** {} seconds\n\
        **Data Sources:** Exa Web Search + Fireworks AI\n\n",
        results.execution_id,
        results.agent_outputs.len(),
        results.duration_ms / 1000
    ));
    
    // Add trend highlights
    if !trends.trends.is_empty() {
        summary.push_str("### Key Trends\n\n");
        for (metric, trend) in &trends.trends {
            let direction = if trend.change_percent > 0.0 { "📈" } else { "📉" };
            summary.push_str(&format!(
                "{} **{}:** {:.1}% change (current: {:.2})\n",
                direction, metric, trend.change_percent, trend.current_value
            ));
        }
        summary.push('\n');
    }
    
    Ok(summary)
}

pub async fn extract_action_items(
    results: &crate::orchestrator::SwarmResults,
) -> Result<Vec<String>> {
    let mut actions = Vec::new();
    
    // Parse agent outputs for action items
    for (agent_name, output) in &results.agent_outputs {
        // Look for numbered lists, bullet points with action verbs
        for line in output.lines() {
            let line = line.trim();
            if line.starts_with("1.") || line.starts_with("2.") || 
               line.starts_with("3.") || line.starts_with("4.") || line.starts_with("5.") ||
               line.to_lowercase().contains("recommend") ||
               line.to_lowercase().contains("should") ||
               line.to_lowercase().contains("must") {
                actions.push(format!("[{}] {}", agent_name, line));
            }
        }
    }
    
    // Remove duplicates
    actions.sort();
    actions.dedup();
    
    Ok(actions)
}

pub async fn generate_ascii_charts(
    results: &crate::orchestrator::SwarmResults,
) -> Result<HashMap<String, String>> {
    let mut charts = HashMap::new();
    
    // Agent performance chart
    let mut chart = String::new();
    chart.push_str("### Agent Performance\n\n");
    chart.push_str("```\n");
    
    for (agent_name, _) in &results.agent_outputs {
        let bar_len = 20; // Fixed for now
        let bar: String = std::iter::repeat('█').take(bar_len).collect();
        chart.push_str(&format!("{:<20} {}\n", agent_name, bar));
    }
    
    chart.push_str("```\n");
    charts.insert("agent_performance".to_string(), chart);
    
    Ok(charts)
}
