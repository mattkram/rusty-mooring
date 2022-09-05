use crate::config::Config;
use pyo3::prelude::*;

#[pyclass]
pub struct MooringSystem {
    #[pyo3(get)]
    config: Config,
}

#[pymethods]
impl MooringSystem {
    /// Load the configuration from a TOML file and create a new instance.
    #[staticmethod]
    pub fn from_file(filename: String) -> PyResult<MooringSystem> {
        let config = match Config::from_file(filename) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        let system = MooringSystem { config: config };
        Ok(system)
    }
}
