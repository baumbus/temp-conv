use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TemperatureParseError {
    #[error("not temperature available with the name `{0}`")]
    InvalidTemperature(String),
}
