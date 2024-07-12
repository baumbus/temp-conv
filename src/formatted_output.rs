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
    pub fn new(
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

    pub fn to_json(self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }

    pub fn to_yaml(self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(&self)
    }

    pub fn to_toml(self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(&self)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, ValueEnum)]
pub enum Format {
    Json,
    Yaml,
    Toml,
}
