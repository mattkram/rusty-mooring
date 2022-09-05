use pyo3::exceptions::PyFileNotFoundError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use toml;

/// Top level struct to hold the config data.
#[pyclass]
#[derive(Deserialize)]
pub struct Config {
    #[pyo3(get)]
    pub general: GeneralConfig,
    #[pyo3(get)]
    pub line_types: HashMap<String, LineType>,
    #[pyo3(get)]
    pub lines: HashMap<String, Line>,
}

/// Line type properties.
#[pyclass]
#[derive(Deserialize, Clone)]
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
#[derive(Deserialize, Clone)]
pub struct LineSegment {
    #[pyo3(get)]
    line_type: String,
    #[pyo3(get)]
    length: f64,
    #[pyo3(get)]
    num_elements: i32,
}

/// A mooring line, consisting of multiple segments.
#[pyclass]
#[derive(Deserialize, Clone)]
pub struct Line {
    #[pyo3(get)]
    top_position: [f64; 3],
    #[pyo3(get)]
    bottom_position: [f64; 3],
    #[pyo3(get)]
    segments: Vec<LineSegment>,
}

// TODO: Is there any way to define as Metric in Rust, but make uppercase in Python?
#[pyclass]
#[derive(Deserialize, Clone)]
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
    pub units: Units,
    #[pyo3(get)]
    pub gravity: f64,
    #[pyo3(get)]
    pub water_density: f64,
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
