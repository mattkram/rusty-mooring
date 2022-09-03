use input::Config;
use pyo3::prelude::*;

mod input;

/// A Python module implemented in Rust.
#[pymodule]
fn rusty_mooring(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Config>()?;
    Ok(())
}
