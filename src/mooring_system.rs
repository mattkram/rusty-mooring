use crate::config::{Config, Line};
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
    fn solve_static(&self) {
        for (line_name, line) in self.config.lines.iter() {
            self.solve_catenary_equation(line_name, line);
        }
    }

    /// Return a vector of all coordinates along each line.
    /// The return type is a mapping of line name to a vector of 3d coordinates.
    fn get_line_coordinates(&self) -> HashMap<String, Vec<[f64; 3]>> {
        HashMap::new()
    }
}

impl MooringSystem {
    /// Solve the catenary equation for a specific line.
    fn solve_catenary_equation(&self, line_name: &String, line: &Line) {
        println!(
            "{:?}, top coords: {:?}, bottom coords: {:?}",
            &line_name, line.top_position, line.bottom_position
        );

        let mut total_length = 0.0;
        let mut submerged_length = 0.0;
        let mut submerged_weight = 0.0;
        for (i, segment) in line.segments.iter().enumerate() {
            println!(
                "segment {:?}, length: {:?}, num_elements: {:?}, element_length: {:?}",
                i,
                segment.length,
                segment.num_elements,
                &segment.element_length()
            );

            total_length += segment.length;

            // FIXME: this is wrong because the submerged length is at an angle,
            //        and depth is vertical
            if total_length + segment.length <= self.config.general.water_depth {
                submerged_length += segment.length;
            }

            let line_type = match self.config.line_types.get(&segment.line_type) {
                Some(l) => l,
                _ => panic!("No line type with name '{}' specified", segment.line_type),
            };
            println!("line_type.mass_per_length: {:?}", line_type.mass_per_length);

            submerged_weight += segment.element_length()
                * self.config.general.gravity
                * (line_type.total_mass_per_length()
                    - self.config.general.water_density * line_type.external_area());
        }
        println!("total_length: {:?}", total_length);
        println!("submerged_length: {:?}", submerged_length);
        println!("submerged_weight: {:?}", submerged_weight);
    }
}
