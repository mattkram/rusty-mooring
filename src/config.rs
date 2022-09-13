use pyo3::exceptions::{PyFileNotFoundError, PyValueError};
use pyo3::prelude::*;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::fs;
use toml;

/// Top level struct to hold the config data.
#[pyclass]
#[derive(Clone, Deserialize)]
pub struct Config {
    #[pyo3(get)]
    pub general: GeneralConfig,
    #[pyo3(get)]
    pub line_types: HashMap<String, LineType>,
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
    pub gravity: f64,
    #[pyo3(get)]
    pub water_density: f64,
    #[pyo3(get)]
    pub water_depth: f64,
}

/// Line type properties.
#[pyclass]
#[derive(Clone, Deserialize)]
pub struct LineType {
    #[pyo3(get)]
    pub diameter: f64,
    #[pyo3(get)]
    pub mass_per_length: f64,
    #[pyo3(get)]
    pub youngs_modulus: f64,
    #[pyo3(get)]
    pub internal_diameter: f64,
    #[pyo3(get)]
    pub internal_contents_density: f64,
}

impl LineType {
    /// The circular area of the inside of a pipe section.
    pub fn internal_area(&self) -> f64 {
        0.25 * PI * self.internal_diameter.powi(2)
    }

    /// The circular area of exterior profile of the line.
    pub fn external_area(&self) -> f64 {
        0.25 * PI * self.diameter.powi(2)
    }

    /// The axial stiffness, i.e. EA
    pub fn axial_stiffness(&self) -> f64 {
        self.youngs_modulus * (self.external_area() - self.internal_area())
    }

    /// The total mass per length, including both structural and internal fluid contents.
    pub fn total_mass_per_length(&self) -> f64 {
        self.mass_per_length + self.internal_contents_density * self.internal_area()
    }
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

impl LineSegment {
    /// The length of the discrete elements in the line segment.
    pub fn element_length(&self) -> f64 {
        self.length / (self.num_elements as f64)
    }
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
