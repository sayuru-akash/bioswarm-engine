use crate::models::{Checkpoint, EnhancedReport, RunSummary, SwarmResults, Trend, TrendAnalysis};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub async fn new<P: AsRef<Path>>(database_path: P) -> Result<Self> {
        let conn = Connection::open(database_path).context("failed to open SQLite database")?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.run_migrations().await?;
        Ok(db)
    }

    pub async fn run_migrations(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "BEGIN;
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL
            );
            INSERT OR IGNORE INTO schema_migrations (version, applied_at) VALUES (1, CURRENT_TIMESTAMP);

            CREATE TABLE IF NOT EXISTS executions (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                query TEXT NOT NULL,
                duration_ms INTEGER NOT NULL,
                total_tokens INTEGER NOT NULL,
                confidence_score INTEGER NOT NULL,
                status TEXT NOT NULL,
                report_markdown TEXT NOT NULL,
                report_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS checkpoints (
                execution_id TEXT PRIMARY KEY,
                query TEXT NOT NULL,
                completed_agents TEXT NOT NULL,
                remaining_agents TEXT NOT NULL,
                partial_results TEXT NOT NULL,
                timestamp TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS agent_outputs (
                execution_id TEXT NOT NULL,
                agent_name TEXT NOT NULL,
                content TEXT NOT NULL,
                confidence INTEGER NOT NULL,
                duration_ms INTEGER NOT NULL,
                sources_json TEXT NOT NULL,
                PRIMARY KEY (execution_id, agent_name)
            );
            COMMIT;"
        )?;

        let checkpoint_columns = {
            let mut stmt = conn.prepare("PRAGMA table_info(checkpoints)")?;
            let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;
            rows.collect::<std::result::Result<Vec<_>, _>>()?
        };
        if !checkpoint_columns.iter().any(|column| column == "query") {
            conn.execute_batch(
                "ALTER TABLE checkpoints RENAME TO checkpoints_legacy;
                CREATE TABLE checkpoints (
                    execution_id TEXT PRIMARY KEY,
                    query TEXT NOT NULL,
                    completed_agents TEXT NOT NULL,
                    remaining_agents TEXT NOT NULL,
                    partial_results TEXT NOT NULL,
                    timestamp TEXT NOT NULL
                );
                INSERT INTO checkpoints (execution_id, query, completed_agents, remaining_agents, partial_results, timestamp)
                SELECT execution_id, '', completed_agents, remaining_agents, partial_results, timestamp
                FROM checkpoints_legacy;
                DROP TABLE checkpoints_legacy;"
            )?;
        }

        let execution_columns = {
            let mut stmt = conn.prepare("PRAGMA table_info(executions)")?;
            let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;
            rows.collect::<std::result::Result<Vec<_>, _>>()?
        };
        if !execution_columns.iter().any(|column| column == "query")
            || !execution_columns.iter().any(|column| column == "confidence_score")
        {
            conn.execute_batch(
                "ALTER TABLE executions RENAME TO executions_legacy;
                CREATE TABLE executions (
                    id TEXT PRIMARY KEY,
                    timestamp TEXT NOT NULL,
                    query TEXT NOT NULL,
                    duration_ms INTEGER NOT NULL,
                    total_tokens INTEGER NOT NULL,
                    confidence_score INTEGER NOT NULL,
                    status TEXT NOT NULL,
                    report_markdown TEXT NOT NULL,
                    report_json TEXT NOT NULL
                );
                INSERT INTO executions (id, timestamp, query, duration_ms, total_tokens, confidence_score, status, report_markdown, report_json)
                SELECT id, timestamp, '', duration_ms, total_tokens, 0, status, COALESCE(report_markdown, ''), COALESCE(report_json, '{}')
                FROM executions_legacy;
                DROP TABLE executions_legacy;"
            )?;
        }

        let agent_columns = {
            let mut stmt = conn.prepare("PRAGMA table_info(agent_outputs)")?;
            let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;
            rows.collect::<std::result::Result<Vec<_>, _>>()?
        };
        if !agent_columns.iter().any(|column| column == "sources_json") {
            conn.execute("ALTER TABLE agent_outputs ADD COLUMN sources_json TEXT NOT NULL DEFAULT '[]'", [])?;
        }
        Ok(())
    }

    pub async fn store_execution(
        &self,
        results: &SwarmResults,
        report: &EnhancedReport,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO executions (id, timestamp, query, duration_ms, total_tokens, confidence_score, status, report_markdown, report_json)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                results.execution_id,
                results.timestamp.to_rfc3339(),
                results.query,
                results.duration_ms as i64,
                results.total_tokens as i64,
                report.confidence_score as i64,
                "completed",
                report.to_markdown(),
                serde_json::to_string(report)?
            ],
        )?;

        for output in results.agent_outputs.values() {
            conn.execute(
                "INSERT OR REPLACE INTO agent_outputs (execution_id, agent_name, content, confidence, duration_ms, sources_json)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    results.execution_id,
                    output.agent_name,
                    output.content,
                    output.confidence as i64,
                    output.duration_ms as i64,
                    serde_json::to_string(&output.sources)?
                ],
            )?;
        }

        conn.execute(
            "DELETE FROM checkpoints WHERE execution_id = ?1",
            params![results.execution_id.clone()],
        )?;
        Ok(())
    }

    pub async fn save_checkpoint(&self, checkpoint: &Checkpoint) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO checkpoints (execution_id, query, completed_agents, remaining_agents, partial_results, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                checkpoint.execution_id,
                checkpoint.query,
                serde_json::to_string(&checkpoint.completed_agents)?,
                serde_json::to_string(&checkpoint.remaining_agents)?,
                serde_json::to_string(&checkpoint.partial_results)?,
                checkpoint.timestamp.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub async fn get_latest_checkpoint(&self) -> Result<Option<Checkpoint>> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT execution_id, query, completed_agents, remaining_agents, partial_results, timestamp
             FROM checkpoints ORDER BY timestamp DESC LIMIT 1",
            [],
            |row| {
                let timestamp: String = row.get(5)?;
                Ok(Checkpoint {
                    execution_id: row.get(0)?,
                    query: row.get(1)?,
                    completed_agents: serde_json::from_str(&row.get::<_, String>(2)?).unwrap_or_default(),
                    remaining_agents: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
                    partial_results: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
                    timestamp: DateTime::parse_from_rfc3339(&timestamp).map(|dt| dt.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                })
            },
        ).optional().map_err(Into::into)
    }

    pub async fn load_execution_report(
        &self,
        execution_id: &str,
    ) -> Result<Option<EnhancedReport>> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT report_json FROM executions WHERE id = ?1",
            params![execution_id],
            |row| {
                let json: String = row.get(0)?;
                serde_json::from_str(&json).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })
            },
        )
        .optional()
        .map_err(Into::into)
    }

    pub async fn list_runs(&self, limit: usize) -> Result<Vec<RunSummary>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, timestamp, query, status, duration_ms, total_tokens, confidence_score
             FROM executions ORDER BY timestamp DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit as i64], |row| {
            let timestamp: String = row.get(1)?;
            Ok(RunSummary {
                execution_id: row.get(0)?,
                timestamp: DateTime::parse_from_rfc3339(&timestamp)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                query: row.get(2)?,
                status: row.get(3)?,
                duration_ms: row.get::<_, i64>(4)? as u64,
                total_tokens: row.get::<_, i64>(5)? as u64,
                confidence_score: row.get::<_, i64>(6)? as u8,
            })
        })?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub async fn analyze_trends(&self) -> Result<TrendAnalysis> {
        let runs = self.list_runs(2).await?;
        let mut trends = BTreeMap::new();
        let mut delta_summary = Vec::new();
        if let Some(current) = runs.first() {
            let previous = runs.get(1);
            let previous_tokens = previous.map(|run| run.total_tokens as f64);
            let change_percent = previous_tokens
                .map(|prev| {
                    if prev == 0.0 {
                        0.0
                    } else {
                        ((current.total_tokens as f64 - prev) / prev) * 100.0
                    }
                })
                .unwrap_or(0.0);
            trends.insert(
                "total_tokens".to_string(),
                Trend {
                    current_value: current.total_tokens as f64,
                    previous_value: previous_tokens,
                    change_percent,
                },
            );
            delta_summary.push(format!(
                "Token usage changed by {change_percent:.1}% compared with the previous run."
            ));
        }
        Ok(TrendAnalysis {
            trends,
            delta_summary,
        })
    }
}
