use crate::models::ReportTemplateConfig;

pub struct ReportTemplate;

impl ReportTemplate {
    pub fn full() -> ReportTemplateConfig {
        ReportTemplateConfig {
            name: "full".to_string(),
            include_trends: true,
            include_action_items: true,
            include_charts: true,
            include_sources: true,
        }
    }

    pub fn executive() -> ReportTemplateConfig {
        ReportTemplateConfig {
            name: "executive".to_string(),
            include_trends: true,
            include_action_items: true,
            include_charts: false,
            include_sources: false,
        }
    }
}
