pub mod agents;
pub mod config;
pub mod database;
pub mod exports;
pub mod models;
pub mod orchestrator;
pub mod search;
pub mod templates;
pub mod utils;

use anyhow::Result;
use std::sync::Arc;
use tracing::info;

pub async fn execute_run(config: config::RuntimeConfig) -> Result<models::EnhancedReport> {
    let db = database::Database::new(&config.database_path).await?;
    let search_client = Arc::new(search::ExaSearchClient::new(config.exa_api_key.clone())?);
    let ai_client = Arc::new(search::FireworksClient::new(
        config.fireworks_api_key.clone(),
        config.backend.clone(),
        config.model.clone(),
        config.api_base_url.clone(),
    )?);
    let checkpoint = db.get_latest_checkpoint().await?;
    let execution_id = checkpoint
        .as_ref()
        .map(|checkpoint| checkpoint.execution_id.clone())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let results = if let Some(checkpoint) = checkpoint {
        if checkpoint.query == config.query {
            orchestrator::resume_swarm(&db, &search_client, &ai_client, &config, &checkpoint)
                .await?
        } else {
            orchestrator::run_enhanced_swarm(
                &db,
                &search_client,
                &ai_client,
                &config,
                &execution_id,
            )
            .await?
        }
    } else {
        orchestrator::run_enhanced_swarm(&db, &search_client, &ai_client, &config, &execution_id)
            .await?
    };

    let report = generate_report(&db, &results, &config).await?;
    db.store_execution(&results, &report).await?;
    let written = exports::export_formats(
        &report,
        &results.execution_id,
        &config.output_dir,
        &config.formats,
    )
    .await?;
    for path in written {
        info!("wrote {}", path.display());
    }
    Ok(report)
}

pub async fn export_existing(
    execution_id: &str,
    db_path: &std::path::Path,
    output_dir: &std::path::Path,
    formats: &[config::ExportFormat],
) -> Result<Vec<std::path::PathBuf>> {
    let db = database::Database::new(db_path).await?;
    let report = db
        .load_execution_report(execution_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("execution {execution_id} not found"))?;
    exports::export_formats(&report, execution_id, output_dir, formats).await
}

pub async fn history(db_path: &std::path::Path, limit: usize) -> Result<Vec<models::RunSummary>> {
    database::Database::new(db_path)
        .await?
        .list_runs(limit)
        .await
}

pub async fn status(
    db_path: &std::path::Path,
) -> Result<(Vec<models::RunSummary>, Option<models::Checkpoint>)> {
    let db = database::Database::new(db_path).await?;
    let runs = db.list_runs(5).await?;
    let checkpoint = db.get_latest_checkpoint().await?;
    Ok((runs, checkpoint))
}

async fn generate_report(
    db: &database::Database,
    results: &models::SwarmResults,
    config: &config::RuntimeConfig,
) -> Result<models::EnhancedReport> {
    let trends = db.analyze_trends().await?;
    let summary = utils::generate_executive_summary(results, &trends).await?;
    let action_items = utils::extract_action_items(results).await?;
    let charts = utils::generate_ascii_charts(results).await?;
    let confidence_score = utils::confidence_score(results);
    Ok(models::EnhancedReport {
        title: format!("BioSwarm v3.5 report - {}", config.query),
        execution_id: results.execution_id.clone(),
        timestamp: results.timestamp,
        summary,
        trends,
        action_items,
        charts,
        agent_outputs: results.agent_outputs.clone(),
        export_formats: config
            .formats
            .iter()
            .map(|fmt| format!("{:?}", fmt))
            .collect(),
        confidence_score,
        metadata: models::RunMetadata {
            query: config.query.clone(),
            agent_count: config.agents.len(),
            formats: config
                .formats
                .iter()
                .map(|fmt| format!("{:?}", fmt))
                .collect(),
            output_dir: config.output_dir.display().to_string(),
            resumed: false,
            backend: config.backend.to_string(),
            model: config.model.clone(),
            api_base_url: config.api_base_url.clone(),
        },
    })
}
