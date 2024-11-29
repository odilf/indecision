pub mod particle;
pub mod simulation;

use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
mod indecision_rs {
    use super::*;

    #[pymodule]
    mod particle {
        #[pymodule_export]
        use crate::particle::MonoLigand;
    }

    #[pymodule]
    mod simulate {}

    #[pymodule]
    mod extract {
        // use pyo3::types::PyFloat;
        //
        // use super::*;
        //
        // #[pyfunction]
        // pub fn thetas(_data: numpy::array::PyArray2<PyFloat>) -> numpy::array::PyArray2<PyFloat> {
        //     todo!()
        // }
    }
}
