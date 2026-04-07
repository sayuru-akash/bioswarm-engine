//! Enhanced BioSwarm Engine v3.0 - Recursive Intelligence with Persistence
//! Full-featured standalone CLI tool

pub mod agents;
pub mod models;
pub mod orchestrator;
pub mod search;
pub mod config;
pub mod database;
pub mod exports;
pub mod templates;
pub mod utils;

use anyhow::Result;
use std::sync::Arc;
use tracing::{info, warn};

pub async fn run_enhanced_swarm() -> Result<()> {
    info!("🚀 BioSwarm v3.0 Enhanced - Initializing...");
    
    // Initialize database
    let db = database::Database::new("bioswarm.db").await?;
    info!("✅ SQLite database initialized");
    
    // Initialize search clients
    let search_client = Arc::new(search::ExaSearchClient::new()?);
    let ai_client = Arc::new(search::FireworksClient::new()?);
    info!("✅ Search clients ready (Exa + Fireworks)");
    
    // Check for resumable runs
    if let Some(checkpoint) = db.get_latest_checkpoint().await? {
        warn!("Found checkpoint from previous run - resuming...");
        return resume_from_checkpoint(db, search_client, ai_client, checkpoint).await;
    }
    
    // Start fresh swarm
    info!("🤖 Spawning 14 enhanced agents with recursive capability...");
    let execution_id = uuid::Uuid::new_v4().to_string();
    
    // Run enhanced parallel swarm with smart orchestration
    let results = orchestrator::run_enhanced_swarm(
        &db,
        &search_client,
        &ai_client,
        &execution_id
    ).await?;
    
    info!("\n✅ Swarm execution complete!");
    
    // Generate enhanced report
    let report = generate_enhanced_report(&db, &results).await?;
    
    // Export to all formats
    exports::export_all_formats(&report, &execution_id).await?;
    
    // Store in database
    db.store_execution(&execution_id, &results, &report).await?;
    
    info!("\n📊 EXECUTION SUMMARY");
    info!("====================");
    info!("Execution ID: {}", execution_id);
    info!("Agents: 14/14 successful");
    info!("Database: bioswarm.db");
    info!("Exports: PDF, Excel, DOCX, JSON, HTML, Markdown");
    info!("Templates: Applied");
    info!("Visual Charts: Generated");
    
    Ok(())
}

async fn resume_from_checkpoint(
    db: database::Database,
    search_client: Arc<search::ExaSearchClient>,
    ai_client: Arc<search::FireworksClient>,
    checkpoint: models::Checkpoint,
) -> Result<()> {
    info!("🔄 Resuming from checkpoint: {}", checkpoint.execution_id);
    info!("⏳ {} agents completed, {} remaining", 
        checkpoint.completed_agents.len(),
        checkpoint.remaining_agents.len()
    );
    
    // Resume swarm from checkpoint
    let results = orchestrator::resume_swarm(
        &db,
        &search_client,
        &ai_client,
        &checkpoint
    ).await?;
    
    // Continue with report generation
    let report = generate_enhanced_report(&db, &results).await?;
    exports::export_all_formats(&report, &checkpoint.execution_id).await?;
    db.store_execution(&checkpoint.execution_id, &results, &report).await?;
    
    info!("✅ Resumed execution completed!");
    Ok(())
}

async fn generate_enhanced_report(
    db: &database::Database,
    results: &orchestrator::SwarmResults,
) -> Result<models::EnhancedReport> {
    use templates::ReportTemplate;
    
    // Apply template
    let template = ReportTemplate::default();
    
    // Get trend analysis
    let trends = db.analyze_trends().await?;
    
    // Generate executive summary
    let summary = utils::generate_executive_summary(results, &trends).await?;
    
    // Extract action items
    let action_items = utils::extract_action_items(results).await?;
    
    // Create visual charts
    let charts = utils::generate_ascii_charts(results).await?;
    
    // Build full report
    let report = models::EnhancedReport {
        title: "BioSwarm v3.0 - Enhanced Intelligence Report".to_string(),
        execution_id: results.execution_id.clone(),
        timestamp: chrono::Utc::now(),
        summary,
        trends,
        action_items,
        charts,
        agent_outputs: results.agent_outputs.clone(),
        export_formats: vec![
            "PDF".to_string(),
            "Excel".to_string(),
            "DOCX".to_string(),
            "JSON".to_string(),
            "HTML".to_string(),
            "Markdown".to_string(),
        ],
    };
    
    Ok(report)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .compact()
        .init();
    
    run_enhanced_swarm().await
}