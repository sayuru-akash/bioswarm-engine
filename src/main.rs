//! BioSwarm v2.0 - Multi-Source Intelligence Engine
//! Main entry point for CLI

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🏆 BioSwarm v2.0 - Multi-Source Intelligence Engine");
    println!("Using: Exa Web Search + Fireworks AI (Kimi K2.5 Turbo)");
    println!();
    
    // Run the enhanced swarm
    bioswarm_engine::run_enhanced_swarm().await?;
    
    Ok(())
}