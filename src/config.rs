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
    pub line_type: HashMap<String, LineType>,
    #[pyo3(get)]
    pub lines: HashMap<String, Line>,
}

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

/// Data from the `[general]` section.
#[pyclass]
#[derive(Clone, Deserialize)]
pub struct GeneralConfig {
    #[pyo3(get)]
    pub units: String,
    #[pyo3(get)]
    pub gravity: f64,
    #[pyo3(get)]
    pub water_density: f64,
}

#[pymethods]
impl GeneralConfig {
    #[new]
    fn new(units: String, gravity: f64, water_density: f64) -> Self {
        GeneralConfig {
            units: units,
            gravity: gravity,
            water_density: water_density,
        }
    }
}

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
            Err(_) => {
                return Err(PyValueError::new_err(format!(
                    "Unable to load data from `{}`",
                    filename
                )));
            }
        };

        Ok(data)
    }
}
