use crate::config::ExportFormat;
use crate::models::EnhancedReport;
use anyhow::Result;
use std::path::{Path, PathBuf};

pub async fn export_formats(
    report: &EnhancedReport,
    execution_id: &str,
    output_dir: &Path,
    formats: &[ExportFormat],
) -> Result<Vec<PathBuf>> {
    std::fs::create_dir_all(output_dir)?;
    let mut outputs = Vec::new();
    for format in formats {
        let path = output_dir.join(format!("bioswarm_{}.{}", execution_id, format.extension()));
        match format {
            ExportFormat::Markdown => std::fs::write(&path, report.to_markdown())?,
            ExportFormat::Json => std::fs::write(&path, serde_json::to_string_pretty(report)?)?,
            ExportFormat::Html => std::fs::write(&path, report.to_html())?,
            ExportFormat::Csv => std::fs::write(&path, report.to_csv())?,
        }
        outputs.push(path);
    }
    Ok(outputs)
}
