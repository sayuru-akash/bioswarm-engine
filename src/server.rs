use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use bioswarm_engine::config::{default_agents, Cli, Commands, ExportFormat, RuntimeConfig};
use serde::Deserialize;
use std::path::PathBuf;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "bioswarm-engine",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

#[derive(Deserialize)]
struct RunRequest {
    query: Option<String>,
}

#[post("/api/v1/swarm/run")]
async fn run_swarm(req: web::Json<RunRequest>) -> impl Responder {
    let cli = Cli {
        config: None,
        command: Commands::Status,
    };
    let config = RuntimeConfig::load(
        &cli,
        req.query
            .clone()
            .unwrap_or_else(|| "api-triggered run".to_string()),
        Some(PathBuf::from("outputs")),
        Some(PathBuf::from("bioswarm.db")),
        Some(2),
        Some(vec![ExportFormat::Markdown, ExportFormat::Json]),
        Some(default_agents()),
    );

    match config {
        Ok(config) => match bioswarm_engine::execute_run(config).await {
            Ok(report) => HttpResponse::Ok().json(serde_json::json!({
                "status": "completed",
                "execution_id": report.execution_id,
                "confidence": report.confidence_score,
            })),
            Err(err) => HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": err.to_string()})),
        },
        Err(err) => HttpResponse::BadRequest().json(serde_json::json!({"error": err.to_string()})),
    }
}

#[get("/api/v1/swarm/status")]
async fn status() -> impl Responder {
    match bioswarm_engine::status(&PathBuf::from("bioswarm.db")).await {
        Ok((runs, checkpoint)) => HttpResponse::Ok().json(serde_json::json!({
            "runs": runs,
            "checkpoint": checkpoint,
        })),
        Err(err) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": err.to_string()}))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(run_swarm)
            .service(status)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
