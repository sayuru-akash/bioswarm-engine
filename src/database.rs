//! Database layer for BioSwarm v3.0 - SQLite persistence with rusqlite
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use std::collections::HashMap;
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub async fn new<P: AsRef<Path>>(database_path: P) -> Result<Self> {
        let conn = Connection::open(database_path)
            .context("Failed to open SQLite database")?;
        
        // Initialize schema
        conn.execute(
            "CREATE TABLE IF NOT EXISTS executions (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                duration_ms INTEGER NOT NULL,
                total_tokens INTEGER NOT NULL,
                status TEXT NOT NULL,
                report_markdown TEXT,
                report_json TEXT
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS agent_outputs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                execution_id TEXT NOT NULL,
                agent_name TEXT NOT NULL,
                content TEXT NOT NULL,
                confidence INTEGER NOT NULL,
                duration_ms INTEGER NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS checkpoints (
                execution_id TEXT PRIMARY KEY,
                completed_agents TEXT NOT NULL,
                remaining_agents TEXT NOT NULL,
                partial_results TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS trends (
                metric_name TEXT PRIMARY KEY,
                current_value REAL NOT NULL,
                previous_value REAL,
                change_percent REAL,
                last_updated TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(Self { conn })
    }
    
    pub async fn store_execution(
        &self,
        execution_id: &str,
        results: &crate::orchestrator::SwarmResults,
        report: &crate::models::EnhancedReport,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT INTO executions (id, timestamp, duration_ms, total_tokens, status, report_markdown, report_json)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET
             timestamp=excluded.timestamp, duration_ms=excluded.duration_ms,
             total_tokens=excluded.total_tokens, status=excluded.status,
             report_markdown=excluded.report_markdown, report_json=excluded.report_json",
            params![
                execution_id,
                Utc::now().to_rfc3339(),
                results.duration_ms as i64,
                results.total_tokens as i64,
                "completed",
                report.to_markdown(),
                serde_json::to_string(report)?
            ],
        )?;
        
        for (agent_name, output) in &results.agent_outputs {
            self.conn.execute(
                "INSERT INTO agent_outputs (execution_id, agent_name, content, confidence, duration_ms)
                 VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT DO NOTHING",
                params![execution_id, agent_name, output, 75i64, 35000i64],
            )?;
        }
        
        Ok(())
    }
    
    pub async fn get_latest_checkpoint(&self) -> Result<Option<crate::models::Checkpoint>> {
        let result = self.conn.query_row(
            "SELECT execution_id, completed_agents, remaining_agents, partial_results
             FROM checkpoints
             ORDER BY timestamp DESC
             LIMIT 1",
            [],
            |row| {
                Ok(crate::models::Checkpoint {
                    execution_id: row.get(0)?,
                    completed_agents: serde_json::from_str(row.get::<_, String>(1)?.as_str()).unwrap_or_default(),
                    remaining_agents: serde_json::from_str(row.get::<_, String>(2)?.as_str()).unwrap_or_default(),
                    partial_results: serde_json::from_str(row.get::<_, String>(3)?.as_str()).unwrap_or_default(),
                })
            },
        ).optional()?;
        
        Ok(result)
    }
    
    pub async fn save_checkpoint(&self, checkpoint: &crate::models::Checkpoint) -> Result<()> {
        self.conn.execute(
            "INSERT INTO checkpoints (execution_id, completed_agents, remaining_agents, partial_results, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(execution_id) DO UPDATE SET
             completed_agents=excluded.completed_agents,
             remaining_agents=excluded.remaining_agents,
             partial_results=excluded.partial_results,
             timestamp=excluded.timestamp",
            params![
                &checkpoint.execution_id,
                serde_json::to_string(&checkpoint.completed_agents)?,
                serde_json::to_string(&checkpoint.remaining_agents)?,
                serde_json::to_string(&checkpoint.partial_results)?,
                Utc::now().to_rfc3339()
            ],
        )?;
        
        Ok(())
    }
    
    pub async fn analyze_trends(&self) -> Result<crate::models::TrendAnalysis> {
        let mut stmt = self.conn.prepare(
            "SELECT metric_name, current_value, previous_value
             FROM trends
             ORDER BY last_updated DESC"
        )?;
        
        let rows = stmt.query_map([], |row| {
            let name: String = row.get(0)?;
            let current: f64 = row.get(1)?;
            let previous: Option<f64> = row.get(2)?;
            
            let change = if let Some(prev) = previous {
                if prev != 0.0 {
                    ((current - prev) / prev) * 100.0
                } else {
                    0.0
                }
            } else {
                0.0
            };
            
            Ok((name, crate::models::Trend {
                current_value: current,
                change_percent: change,
            }))
        })?;
        
        let mut trends = HashMap::new();
        for row in rows {
            let (name, trend) = row?;
            trends.insert(name, trend);
        }
        
        Ok(crate::models::TrendAnalysis { trends })
    }
}
