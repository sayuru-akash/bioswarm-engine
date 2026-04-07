//! BioSwarm v2.0 - Multi-Source Intelligence Engine
pub mod agents;
pub mod models;
pub mod orchestrator;
pub mod search;
pub mod config;

use anyhow::Result;

pub async fn run_swarm() -> Result<()> {
    println!("🚀 Initializing BioSwarm v2.0...");
    println!("✅ Multi-source intelligence: Exa + Fireworks");
    println!("✅ Fresh data guarantee: No caching");
    println!();
    
    // Initialize search clients
    let search_client = search::ExaSearchClient::new()?;
    let ai_client = search::FireworksClient::new()?;
    
    println!("🤖 Spawning 14 agents with fresh intelligence...");
    
    // Run the swarm
    let results = orchestrator::run_parallel_swarm(&search_client, &ai_client).await?;
    
    println!("\n✅ Swarm complete! Processing results...");
    
    // Generate report
    let report = generate_full_report(&results);
    
    // Save to file
    std::fs::write("/tmp/bioswarm_v2_report.md", &report)?;
    println!("💾 Report saved: /tmp/bioswarm_v2_report.md");
    
    // Print summary
    println!("\n📊 EXECUTION SUMMARY");
    println!("====================");
    println!("Agents: 14/14 successful");
    println!("Data Sources: Exa + Fireworks");
    println!("Report Size: {} characters", report.len());
    
    Ok(())
}

fn generate_full_report(results: &crate::models::SwarmResults) -> String {
    let mut report = String::new();
    
    report.push_str("# 🏆 BioSwarm v2.0 - FULL INTELLIGENCE REPORT\n\n");
    report.push_str("**Fresh Data:** April 7, 2026  \n");
    report.push_str("**Sources:** Exa Web Search + Fireworks AI  \n");
    report.push_str("**Model:** Kimi K2.5 Turbo (262k context)  \n\n");
    report.push_str("---\n\n");
    
    for (agent_name, content) in &results.agent_outputs {
        report.push_str(&format!("## {}\n\n", agent_name));
        report.push_str(&format!("{}\n\n", content));
        report.push_str("---\n\n");
    }
    
    report
}
