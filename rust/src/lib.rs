//! Framework for experimenting on selectivity of nano-particles

#![warn(missing_docs)]

mod extract;
pub mod particle;
pub mod simulation;

use pyo3::prelude::*;

pyo3_stub_gen::define_stub_info_gatherer!(stub_info);

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn core(m: Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    use crate::particle::*;
    m.add_class::<MonoLigand>()?;
    m.add_class::<MultiLigand>()?;
    m.add_class::<Interfering>()?;
    
    Ok(())
}
