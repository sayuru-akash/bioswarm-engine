//! Report templates for BioSwarm v3.0

pub struct ReportTemplate {
    pub name: String,
    pub include_trends: bool,
    pub include_action_items: bool,
    pub include_charts: bool,
    pub include_sources: bool,
}

impl Default for ReportTemplate {
    fn default() -> Self {
        Self {
            name: "Full Intelligence Report".to_string(),
            include_trends: true,
            include_action_items: true,
            include_charts: true,
            include_sources: true,
        }
    }
}

impl ReportTemplate {
    pub fn executive_summary_template() -> Self {
        Self {
            name: "Executive Summary Only".to_string(),
            include_trends: true,
            include_action_items: true,
            include_charts: false,
            include_sources: false,
        }
    }
    
    pub fn detailed_analysis_template() -> Self {
        Self {
            name: "Detailed Technical Analysis".to_string(),
            include_trends: true,
            include_action_items: true,
            include_charts: true,
            include_sources: true,
        }
    }
}
