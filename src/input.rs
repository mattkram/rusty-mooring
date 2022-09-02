use pyo3::exceptions::PyFileNotFoundError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde_derive::Deserialize;
use std::fs;
use toml;

// Top level struct to hold the TOML data.
#[derive(Deserialize)]
pub struct Data {
    pub config: Config,
}

/// Config holds data from the `[config]` section.
#[pyclass]
#[derive(Deserialize)]
pub struct Config {
    #[pyo3(get, set)]
    pub ip: String,
    #[pyo3(get, set)]
    pub port: u16,
}

#[pymethods]
impl Config {
    #[new]
    fn new(ip: String, port: u16) -> Self {
        Config { ip: ip, port: port }
    }

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

        let data: Data = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                return Err(PyValueError::new_err(format!(
                    "Unable to load data from `{}`",
                    filename
                )));
            }
        };

        Ok(data.config)
    }
}
