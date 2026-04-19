use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Markdown,
    Json,
    Html,
    Csv,
}

impl ExportFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Markdown => "md",
            Self::Json => "json",
            Self::Html => "html",
            Self::Csv => "csv",
        }
    }
}

impl std::fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Markdown => "markdown",
            Self::Json => "json",
            Self::Html => "html",
            Self::Csv => "csv",
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ModelBackend {
    Fireworks,
    Ollama,
    #[value(name = "openai-compatible", alias = "open-ai-compatible")]
    OpenAiCompatible,
    #[value(name = "codex")]
    Codex,
}

impl std::fmt::Display for ModelBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Fireworks => "fireworks",
            Self::Ollama => "ollama",
            Self::OpenAiCompatible => "openai-compatible",
            Self::Codex => "codex",
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    pub fireworks_api_key: Option<String>,
    pub exa_api_key: Option<String>,
    pub rate_limit_rpm: Option<u32>,
    pub database_path: Option<PathBuf>,
    pub output_dir: Option<PathBuf>,
    pub depth: Option<u8>,
    pub agents: Option<Vec<String>>,
    pub formats: Option<Vec<ExportFormat>>,
    pub backend: Option<ModelBackend>,
    pub model: Option<String>,
    pub api_base_url: Option<String>,
    pub api_key_env: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub fireworks_api_key: String,
    pub exa_api_key: Option<String>,
    pub rate_limit_rpm: u32,
    pub database_path: PathBuf,
    pub output_dir: PathBuf,
    pub depth: u8,
    pub agents: Vec<String>,
    pub formats: Vec<ExportFormat>,
    pub query: String,
    pub backend: ModelBackend,
    pub model: String,
    pub api_base_url: String,
    pub api_key_env: String,
}

#[derive(Debug, Parser)]
#[command(
    name = "bioswarm",
    version,
    about = "Production-ready multi-source intelligence engine"
)]
pub struct Cli {
    #[arg(long, env = "BIOSWARM_CONFIG")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run {
        #[arg(long, default_value = "ai market intelligence")]
        query: String,
        #[arg(long)]
        output_dir: Option<PathBuf>,
        #[arg(long)]
        database_path: Option<PathBuf>,
        #[arg(long, default_value_t = 2)]
        depth: u8,
        #[arg(long, value_delimiter = ',', default_values_t = default_formats())]
        formats: Vec<ExportFormat>,
        #[arg(long, value_delimiter = ',')]
        agents: Option<Vec<String>>,
        #[arg(long, env = "BIOSWARM_BACKEND")]
        backend: Option<ModelBackend>,
        #[arg(long, env = "BIOSWARM_MODEL")]
        model: Option<String>,
        #[arg(long, env = "BIOSWARM_API_BASE_URL")]
        api_base_url: Option<String>,
        #[arg(long, env = "BIOSWARM_API_KEY_ENV")]
        api_key_env: Option<String>,
    },
    Resume {
        #[arg(long)]
        execution_id: Option<String>,
    },
    Export {
        #[arg(long)]
        execution_id: String,
        #[arg(long, value_delimiter = ',', default_values_t = default_formats())]
        formats: Vec<ExportFormat>,
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
    History {
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },
    Status,
}

fn default_formats() -> Vec<ExportFormat> {
    vec![
        ExportFormat::Markdown,
        ExportFormat::Json,
        ExportFormat::Html,
        ExportFormat::Csv,
    ]
}

impl RuntimeConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn load(
        cli: &Cli,
        query: String,
        output_override: Option<PathBuf>,
        db_override: Option<PathBuf>,
        depth_override: Option<u8>,
        formats_override: Option<Vec<ExportFormat>>,
        agents_override: Option<Vec<String>>,
        backend_override: Option<ModelBackend>,
        model_override: Option<String>,
        api_base_url_override: Option<String>,
        api_key_env_override: Option<String>,
    ) -> Result<Self> {
        dotenvy::dotenv().ok();

        let file_config = if let Some(path) = &cli.config {
            Some(load_file_config(path)?)
        } else if Path::new("bioswarm.toml").exists() {
            Some(load_file_config(Path::new("bioswarm.toml"))?)
        } else {
            None
        };

        let backend = backend_override
            .or_else(|| {
                std::env::var("BIOSWARM_BACKEND")
                    .ok()
                    .and_then(|v| parse_backend(&v))
            })
            .or_else(|| file_config.as_ref().and_then(|cfg| cfg.backend.clone()))
            .unwrap_or(ModelBackend::Fireworks);

        let api_key_env = api_key_env_override
            .or_else(|| std::env::var("BIOSWARM_API_KEY_ENV").ok())
            .or_else(|| file_config.as_ref().and_then(|cfg| cfg.api_key_env.clone()))
            .unwrap_or_else(|| match backend {
                ModelBackend::Fireworks => "FIREWORKS_API_KEY".to_string(),
                ModelBackend::Ollama => "OLLAMA_API_KEY".to_string(),
                ModelBackend::OpenAiCompatible => "OPENAI_API_KEY".to_string(),
                ModelBackend::Codex => "OPENAI_API_KEY".to_string(),
            });

        let fireworks_api_key = std::env::var(&api_key_env)
            .ok()
            .or_else(|| {
                if api_key_env == "FIREWORKS_API_KEY" {
                    file_config
                        .as_ref()
                        .and_then(|cfg| cfg.fireworks_api_key.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                if matches!(backend, ModelBackend::Ollama) {
                    "ollama".to_string()
                } else {
                    String::new()
                }
            });

        if !matches!(backend, ModelBackend::Ollama) && fireworks_api_key.trim().is_empty() {
            bail!("{} is required. Add it to .env or bioswarm.toml", api_key_env);
        }

        let exa_api_key = std::env::var("EXA_API_KEY")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .or_else(|| {
                file_config
                    .as_ref()
                    .and_then(|cfg| cfg.exa_api_key.clone())
                    .filter(|v| !v.trim().is_empty())
            });

        let rate_limit_rpm = std::env::var("RATE_LIMIT_RPM")
            .ok()
            .and_then(|v| v.parse().ok())
            .or_else(|| file_config.as_ref().and_then(|cfg| cfg.rate_limit_rpm))
            .unwrap_or(60);

        if rate_limit_rpm == 0 {
            bail!("RATE_LIMIT_RPM must be greater than 0");
        }

        let database_path = db_override
            .or_else(|| std::env::var("DATABASE_PATH").ok().map(PathBuf::from))
            .or_else(|| file_config.as_ref().and_then(|cfg| cfg.database_path.clone()))
            .unwrap_or_else(|| PathBuf::from("bioswarm.db"));

        let output_dir = output_override
            .or_else(|| std::env::var("OUTPUT_DIR").ok().map(PathBuf::from))
            .or_else(|| file_config.as_ref().and_then(|cfg| cfg.output_dir.clone()))
            .unwrap_or_else(|| PathBuf::from("outputs"));

        let depth = depth_override
            .or_else(|| std::env::var("BIOSWARM_DEPTH").ok().and_then(|v| v.parse().ok()))
            .or_else(|| file_config.as_ref().and_then(|cfg| cfg.depth))
            .unwrap_or(2);

        if !(1..=5).contains(&depth) {
            bail!("depth must be between 1 and 5");
        }

        let agents = agents_override
            .or_else(|| file_config.as_ref().and_then(|cfg| cfg.agents.clone()))
            .unwrap_or_else(default_agents);

        let formats = formats_override
            .or_else(|| file_config.as_ref().and_then(|cfg| cfg.formats.clone()))
            .unwrap_or_else(default_formats);

        let model = model_override
            .or_else(|| std::env::var("BIOSWARM_MODEL").ok())
            .or_else(|| file_config.as_ref().and_then(|cfg| cfg.model.clone()))
            .unwrap_or_else(|| match backend {
                ModelBackend::Fireworks => "accounts/fireworks/models/kimi-k2-instruct".to_string(),
                ModelBackend::Ollama => "kimi-k2.5:cloud".to_string(),
                ModelBackend::OpenAiCompatible => "gpt-4.1-mini".to_string(),
                ModelBackend::Codex => "gpt-5-codex".to_string(),
            });

        let api_base_url = api_base_url_override
            .or_else(|| std::env::var("BIOSWARM_API_BASE_URL").ok())
            .or_else(|| file_config.as_ref().and_then(|cfg| cfg.api_base_url.clone()))
            .unwrap_or_else(|| match backend {
                ModelBackend::Fireworks => "https://api.fireworks.ai/inference/v1".to_string(),
                ModelBackend::Ollama => "http://127.0.0.1:11434/v1".to_string(),
                ModelBackend::OpenAiCompatible => "https://api.openai.com/v1".to_string(),
                ModelBackend::Codex => "https://api.openai.com/v1".to_string(),
            });

        Ok(Self {
            fireworks_api_key,
            exa_api_key,
            rate_limit_rpm,
            database_path,
            output_dir,
            depth,
            agents,
            formats,
            query,
            backend,
            model,
            api_base_url,
            api_key_env,
        })
    }
}

pub fn default_agents() -> Vec<String> {
    vec![
        "DeepResearcher",
        "GapAnalyzer",
        "OpportunityScorer",
        "CompetitorTracker",
        "InnovationScout",
        "StrategyFormulator",
        "QualityValidator",
        "DeploymentTester",
        "SentimentAnalyzer",
        "PricingIntelligence",
        "TalentScout",
        "FundingTracker",
        "RegulatoryWatcher",
        "ClientIntelligence",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

fn load_file_config(path: &Path) -> Result<FileConfig> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config file {}", path.display()))?;
    toml::from_str(&text).with_context(|| format!("failed to parse {}", path.display()))
}

fn parse_backend(value: &str) -> Option<ModelBackend> {
    match value.trim().to_ascii_lowercase().as_str() {
        "fireworks" => Some(ModelBackend::Fireworks),
        "ollama" => Some(ModelBackend::Ollama),
        "openai" | "openai-compatible" | "openai_compatible" => {
            Some(ModelBackend::OpenAiCompatible)
        }
        "codex" => Some(ModelBackend::Codex),
        _ => None,
    }
}
