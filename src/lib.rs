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
