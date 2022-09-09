use crate::config::{Config, Line};
use pyo3::prelude::*;
use std::collections::HashMap;
use std::f64::consts::PI;

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
    fn solve_static(&self) {
        for (line_name, line) in self.config.lines.iter() {
            solve_catenary_equation(line_name, line);
        }
    }

    /// Return a vector of all coordinates along each line.
    /// The return type is a mapping of line name to a vector of 3d coordinates.
    fn get_line_coordinates(&self) -> HashMap<String, Vec<[f64; 3]>> {
        HashMap::new()
    }
}

fn solve_catenary_equation(line_name: &String, line: &Line) {
    println!(
        "{:?}, top coords: {:?}, bottom coords: {:?}",
        &line_name, line.top_position, line.bottom_position
    );

    println!("{:?}", PI);

    let mut total_length = 0.0;
    for segment in line.segments.iter() {
        println!(
            "length: {:?}, num_elements: {:?}",
            segment.length, segment.num_elements
        );
        total_length += segment.length;
        let element_length = segment.length / (segment.num_elements as f64);
        println!("element_length: {:?}", element_length)
    }
    println!("total_length: {:?}", total_length);
}
