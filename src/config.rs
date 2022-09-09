use pyo3::exceptions::{PyFileNotFoundError, PyValueError};
use pyo3::prelude::*;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use toml;

/// Top level struct to hold the config data.
#[pyclass]
#[derive(Clone, Deserialize)]
pub struct Config {
    #[pyo3(get)]
    general: GeneralConfig,
    #[pyo3(get)]
    line_types: HashMap<String, LineType>,
    #[pyo3(get)]
    pub lines: HashMap<String, Line>,
}

// TODO: Is there any way to define as Metric in Rust, but make uppercase in Python?
#[pyclass]
#[derive(Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Units {
    METRIC,
    ENGLISH,
}

/// General configuration.
#[pyclass]
#[derive(Clone, Deserialize)]
pub struct GeneralConfig {
    #[pyo3(get)]
    units: Units,
    #[pyo3(get)]
    gravity: f64,
    #[pyo3(get)]
    water_density: f64,
}

/// Line type properties.
#[pyclass]
#[derive(Clone, Deserialize)]
pub struct LineType {
    #[pyo3(get)]
    diameter: f64,
    #[pyo3(get)]
    mass_per_length: f64,
    #[pyo3(get)]
    axial_stiffness: f64,
}

/// A line segment, used to build up a full line.
#[pyclass]
#[derive(Clone, Deserialize)]
pub struct LineSegment {
    #[pyo3(get)]
    pub line_type: String,
    #[pyo3(get)]
    pub length: f64,
    #[pyo3(get)]
    pub num_elements: i32,
}

/// A mooring line, consisting of multiple segments.
#[pyclass]
#[derive(Clone, Deserialize)]
pub struct Line {
    #[pyo3(get)]
    pub top_position: [f64; 3],
    #[pyo3(get)]
    pub bottom_position: [f64; 3],
    #[pyo3(get)]
    pub segments: Vec<LineSegment>,
}

/// A structural representation of the input configuration.
#[pymethods]
impl Config {
    /// Load the configuration from a TOML file.
    #[staticmethod]
    pub fn from_file(filename: String) -> PyResult<Config> {
        let contents = match fs::read_to_string(&filename) {
            Ok(c) => c,
            Err(_) => {
                return Err(PyFileNotFoundError::new_err(format!(
                    "File '{filename}' not found"
                )));
            }
        };

        let data: Config = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(e) => {
                return Err(PyValueError::new_err(format!(
                    "Unable to load data from `{filename}`. {e}.",
                )));
            }
        };

        Ok(data)
    }
}
