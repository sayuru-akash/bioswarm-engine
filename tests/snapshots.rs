use bioswarm_engine::models::{
    AgentOutput, EnhancedReport, RunMetadata, SearchResult, TrendAnalysis,
};
use insta::assert_snapshot;
use std::collections::BTreeMap;

fn sample_report() -> EnhancedReport {
    let output = AgentOutput {
        agent_name: "DeepResearcher".into(),
        query_type: "ai market intelligence".into(),
        content: "A sample output".into(),
        confidence: 88,
        duration_ms: 1200,
        recursive_depth: 2,
        sources: vec![SearchResult {
            title: "Example".into(),
            url: "https://example.com".into(),
            snippet: "Snippet".into(),
            source: "mock-exa".into(),
            published_date: Some("2026-04-08".into()),
        }],
        insights: vec![],
    };
    let mut outputs = BTreeMap::new();
    outputs.insert("DeepResearcher".into(), output);
    EnhancedReport {
        title: "Test Report".into(),
        execution_id: "exec-123".into(),
        timestamp: chrono::Utc::now(),
        summary: "summary".into(),
        trends: TrendAnalysis::default(),
        action_items: vec!["Action".into()],
        charts: BTreeMap::new(),
        agent_outputs: outputs,
        export_formats: vec![
            "Markdown".into(),
            "Json".into(),
            "Html".into(),
            "Csv".into(),
        ],
        confidence_score: 88,
        metadata: RunMetadata {
            query: "ai market intelligence".into(),
            agent_count: 14,
            formats: vec!["Markdown".into()],
            output_dir: "outputs".into(),
            resumed: false,
        },
    }
}

#[test]
fn markdown_snapshot() {
    assert_snapshot!(sample_report().to_markdown());
}

#[test]
fn csv_snapshot() {
    assert_snapshot!(sample_report().to_csv());
}

#[test]
fn html_snapshot() {
    assert_snapshot!(sample_report().to_html());
}
