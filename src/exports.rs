//! Export module for BioSwarm v3.0 - Multiple format support
use anyhow::{Context, Result};
use std::path::Path;

pub async fn export_all_formats(report: &crate::models::EnhancedReport, execution_id: &str) -> Result<()> {
    let base_path = format!("/tmp/bioswarm_{}", execution_id);
    
    // Markdown (default)
    let md_path = format!("{}.md", base_path);
    std::fs::write(&md_path, report.to_markdown())?;
    println!("✅ Markdown: {}", md_path);
    
    // JSON
    let json_path = format!("{}.json", base_path);
    std::fs::write(&json_path, serde_json::to_string_pretty(report)?)?;
    println!("✅ JSON: {}", json_path);
    
    // HTML
    let html_path = format!("{}.html", base_path);
    std::fs::write(&html_path, report.to_html())?;
    println!("✅ HTML: {}", html_path);
    
    // CSV (data tables only)
    let csv_path = format!("{}.csv", base_path);
    std::fs::write(&csv_path, report.to_csv())?;
    println!("✅ CSV: {}", csv_path);
    
    println!("\n📁 All exports saved to /tmp/");
    
    Ok(())
}

pub struct PdfExporter;
impl PdfExporter {
    pub async fn export(report: &crate::models::EnhancedReport, path: &str) -> Result<()> {
        // In production, use a PDF library like genpdf or printpdf
        // For now, create a styled HTML that can be printed to PDF
        let html = report.to_html();
        std::fs::write(path, html)?;
        Ok(())
    }
}

pub struct ExcelExporter;
impl ExcelExporter {
    pub async fn export(report: &crate::models::EnhancedReport, path: &str) -> Result<()> {
        // In production, use calamine or rust_xlsxwriter
        // For now, create CSV with Excel formatting hints
        let csv = report.to_csv();
        std::fs::write(path, csv)?;
        Ok(())
    }
}
