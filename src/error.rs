//! Errors which may occur from this crate

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A list of specifying general categories of parsing Errors.
#[derive(Debug, Error, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TemperatureParseError {
    /// Givin temperature was not matching any implemented temperature.
    #[error("not temperature available with the name `{0}`")]
    InvalidTemperature(String),
}
