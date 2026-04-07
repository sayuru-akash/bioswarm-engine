//! Configuration for BioSwarm v2.0
use anyhow::Result;

pub struct Config {
    pub fireworks_api_key: String,
    pub exa_api_key: String,
    pub rate_limit_rpm: u32,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            fireworks_api_key: std::env::var("FIREWORKS_API_KEY")?,
            exa_api_key: std::env::var("EXA_API_KEY").unwrap_or_default(),
            rate_limit_rpm: 60,
        })
    }
}
