use anyhow::Result;
use bioswarm_engine::config::{Cli, Commands, RuntimeConfig};
use clap::Parser;
use comfy_table::{presets::UTF8_FULL, Table};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .compact()
        .init();

    let cli = Cli::parse();

    let config_path = cli.config.clone();
    match cli.command {
        Commands::Run {
            query,
            output_dir,
            database_path,
            depth,
            formats,
            agents,
            backend,
            model,
            api_base_url,
            api_key_env,
        } => {
            let cli_stub = Cli {
                config: config_path.clone(),
                command: Commands::Status,
            };
            let config = RuntimeConfig::load(
                &cli_stub,
                query,
                output_dir,
                database_path,
                Some(depth),
                Some(formats),
                agents,
                backend,
                model,
                api_base_url,
                api_key_env,
            )?;
            let progress = indicatif::ProgressBar::new_spinner();
            progress.set_message(format!("running bioswarm agents via {} / {}", config.backend, config.model));
            progress.enable_steady_tick(std::time::Duration::from_millis(120));
            let report = bioswarm_engine::execute_run(config).await?;
            progress.finish_with_message(format!("completed {}", report.execution_id));
            println!("{}", report.to_markdown());
        }
        Commands::Resume { execution_id: _ } => {
            println!("Resume is handled automatically when a matching checkpoint exists. Use `bioswarm status` to inspect checkpoints.");
        }
        Commands::Export {
            execution_id,
            formats,
            output_dir,
        } => {
            let db_path = std::env::var("DATABASE_PATH")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|_| "bioswarm.db".into());
            let out = output_dir.unwrap_or_else(|| "outputs".into());
            let written =
                bioswarm_engine::export_existing(&execution_id, &db_path, &out, &formats).await?;
            for path in written {
                println!("wrote {}", path.display());
            }
        }
        Commands::History { limit } => {
            let db_path = std::env::var("DATABASE_PATH")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|_| "bioswarm.db".into());
            let runs = bioswarm_engine::history(&db_path, limit).await?;
            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_header(vec![
                "Execution",
                "Query",
                "Status",
                "Confidence",
                "Duration(ms)",
            ]);
            for run in runs {
                table.add_row(vec![
                    run.execution_id,
                    run.query,
                    run.status,
                    run.confidence_score.to_string(),
                    run.duration_ms.to_string(),
                ]);
            }
            println!("{table}");
        }
        Commands::Status => {
            let db_path = std::env::var("DATABASE_PATH")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|_| "bioswarm.db".into());
            let (runs, checkpoint) = bioswarm_engine::status(&db_path).await?;
            println!("recent runs: {}", runs.len());
            if let Some(run) = runs.first() {
                println!("latest status: {} | confidence: {}", run.status, run.confidence_score);
            }
            if let Some(checkpoint) = checkpoint {
                println!(
                    "checkpoint: {} (remaining agents: {})",
                    checkpoint.execution_id,
                    checkpoint.remaining_agents.len()
                );
            } else {
                println!("checkpoint: none");
            }
        }
    }

    Ok(())
}
