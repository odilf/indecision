mod extract;
pub mod particle;
pub mod simulation;

use pyo3::prelude::*;

pyo3_stub_gen::define_stub_info_gatherer!(stub_info);

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
mod core {
    #[pymodule_export]
    use crate::particle::{MonoLigand, MultiLigand, Interfering};

    // #[pymodule_export]
    // use crate::simulation::{MonoLigand, MultiLigand};
}
