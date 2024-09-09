//! Data Structure for helping output the results of a conversion

use anyhow::Result;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Output {
    temperature_in: crate::temperature::Temperature,
    temperature_out: crate::temperature::Temperature,
    value_in: f64,
    value_out: f64,
}

impl Output {
    /// Creates a new [`Output`].
    #[inline]
    pub const fn new(
        temperature_in: crate::temperature::Temperature,
        value_in: f64,
        temperature_out: crate::temperature::Temperature,
        value_out: f64,
    ) -> Self {
        Self {
            temperature_in,
            value_in,
            temperature_out,
            value_out,
        }
    }

    #[cfg(feature = "json")]
    #[inline]
    pub fn to_json(self) -> Result<String> {
        use anyhow::Context;

        serde_json::to_string_pretty(&self).context("Failed to parse Output to Json-Format")
    }

    #[cfg(feature = "yaml")]
    #[inline]
    pub fn to_yaml(self) -> Result<String> {
        use anyhow::Context;

        serde_yaml::to_string(&self).context("Failed to parse Output to Yaml-Format")
    }

    #[cfg(feature = "toml")]
    #[inline]
    pub fn to_toml(self) -> Result<String> {
        use anyhow::Context;

        toml::to_string_pretty(&self).context("Failed to parse Output to Toml-Format")
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, ValueEnum)]
pub enum Format {
    #[cfg(feature = "json")]
    Json,
    #[cfg(feature = "yaml")]
    Yaml,
    #[cfg(feature = "toml")]
    Toml,
}
