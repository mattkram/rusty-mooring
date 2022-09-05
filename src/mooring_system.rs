use crate::config::Config;
use pyo3::prelude::*;
use std::collections::HashMap;

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
        let system = MooringSystem::new(config);
        Ok(system)
    }

    /// Construct a new MooringSystem from a Config.
    #[new]
    fn new(config: Config) -> Self {
        MooringSystem { config: config }
    }

    /// Solve the static equilibrium of the system.
    fn solve_static(&self) {}

    /// Return a vector of all coordinates along each line.
    /// The return type is a mapping of line name to a vector of 3d coordinates.
    fn get_line_coordinates(&self) -> HashMap<String, Vec<[f64; 3]>> {
        HashMap::new()
    }
}
