use pyo3::PyResult;

use crate::simulation::markov::MarkovChain;

use super::{Event, Particle};

/// # Invariants
/// - `attached_ligands <= total_ligands`
#[pyo3_stub_gen::derive::gen_stub_pyclass]
#[pyo3::pyclass]
#[derive(Clone, Copy, Debug)]
pub struct MultiLigandState {
    #[pyo3(get, set)]
    total_ligands: u16,

    #[pyo3(get, set)]
    attached_ligands: u16,
}

impl super::Attach for MultiLigandState {
    fn is_attached(&self) -> bool {
        self.attached_ligands > 0
    }
}

#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pyo3::pymethods]
impl MultiLigandState {
    pub fn bind(&self) -> Self {
        Self {
            attached_ligands: self.attached_ligands + 1,
            ..*self
        }
    }

    pub fn unbind(&self) -> Self {
        Self {
            attached_ligands: self.attached_ligands - 1,
            ..*self
        }
    }
}

/// A particle with many ligands, where each one can attach and dettach from the host.
///
/// # Invariants
/// - `on_rates.len() == off_rates.len()`
#[pyo3_stub_gen::derive::gen_stub_pyclass]
#[pyo3::pyclass]
#[derive(Clone, Debug, Default)]
pub struct MultiLigand {
    /// The density of receptors available to bind to.
    ///
    /// `1.0` corresponds to one receptor per ligand. But perhaps should be by unit area.
    pub receptor_density: f64,

    /// The binding strength of the particle. It makes all transitions occur more often.
    pub binding_strength: f64,

    /// The rates for binding. Encoded as `[0->1, 1->2, 2->3, ...]`.
    pub on_rates: Vec<f64>,

    /// The rates for binding. Encoded as `[1->0, 1->2, 2->3, ...]`.
    pub off_rates: Vec<f64>,
}

impl super::Particle for MultiLigand {
    type State = MultiLigandState;

    fn events(&self, state: &Self::State) -> Vec<Event<Self::State>> {
        let mut events = Vec::with_capacity(2);

        if state.attached_ligands < self.total_ligands() {
            let rate = self.on_rates[state.attached_ligands as usize]
                * if state.attached_ligands == 0 {
                    self.receptor_density
                } else {
                    1.0
                }
                * self.binding_strength;

            events.push(Event {
                rate,
                target: state.bind(),
            });
        }

        if state.attached_ligands > 0 {
            events.push(Event {
                rate: self.off_rates[state.attached_ligands as usize - 1],
                target: state.unbind(),
            });
        }

        events
    }

    fn new_state(&self) -> Self::State {
        MultiLigandState {
            attached_ligands: 0,
            total_ligands: self.total_ligands(),
        }
    }
}

impl MarkovChain for MultiLigand {
    fn states(&self) -> Vec<Self::State> {
        let mut output = Vec::with_capacity(2 * self.total_ligands() as usize);
        for attached_ligands in 0..=self.total_ligands() {
            output.push(Self::State { attached_ligands, total_ligands: self.total_ligands() });
        }

        output
    }
}

crate::monomorphize!(
    MultiLigand {
        #[new]
        fn new(
            receptor_density: f64,
            binding_strength: f64,
            on_rates: Vec<f64>,
            off_rates: Vec<f64>,
        ) -> PyResult<Self> {
            if on_rates.len() != off_rates.len() {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "on_rates and off_rates must have the same length",
                ));
            }

            Ok(Self {
                receptor_density,
                binding_strength,
                on_rates,
                off_rates,
            })
        }

        fn total_ligands(&self) -> u16 {
            assert_eq!(self.on_rates.len(), self.off_rates.len());
            self.on_rates.len() as u16
        }
    },
    MultiLigandSimulation,
    MultiLigandSimulationSingle,
    MultiLiagndTransition
);
