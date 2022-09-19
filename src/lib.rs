use config::{Config, Units};
use mooring_system::{Coordinate, MooringSystem, Node};
use pyo3::prelude::*;

mod config;
mod mooring_system;

/// A Python module implemented in Rust.
#[pymodule]
fn rusty_mooring(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Config>()?;
    m.add_class::<Units>()?;
    m.add_class::<Node>()?;
    m.add_class::<Coordinate>()?;
    m.add_class::<MooringSystem>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::config::LineType;
    use float_cmp::approx_eq;
    use std::f64::consts::PI;

    fn make_line_type() -> LineType {
        LineType {
            diameter: 0.233,
            mass_per_length: 53.7,
            youngs_modulus: 9.15e9,
            internal_diameter: 0.1,
            internal_contents_density: 0.0,
        }
    }

    #[test]
    fn test_line_type_external_area() {
        let line_type = make_line_type();
        assert!(approx_eq!(
            f64,
            line_type.external_area(),
            PI * 0.25_f64 * 0.233_f64.powi(2)
        ));
    }

    #[test]
    fn test_line_type_internal_area() {
        let line_type = make_line_type();
        assert!(approx_eq!(
            f64,
            line_type.internal_area(),
            PI * 0.25_f64 * 0.1_f64.powi(2)
        ));
    }
}
