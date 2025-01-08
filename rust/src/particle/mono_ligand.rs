use super::{Event, Particle};

#[pyo3_stub_gen::derive::gen_stub_pyclass]
#[pyo3::pyclass]
#[derive(Clone, Copy, Debug, Default)]
pub struct MonoLigandState {
    #[pyo3(get, set)]
    is_attached: bool,
}

impl super::Attach for MonoLigandState {
    fn is_attached(&self) -> bool {
        self.is_attached
    }
}

#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pyo3::pymethods]
impl MonoLigandState {
    pub fn toggle(&self) -> Self {
        Self {
            is_attached: !self.is_attached,
        }
    }
}

/// A particle that can attach to a receptor.
///
/// This particle is a simple model of a ligand that can attach to a receptor. It has a binding
/// strength that determines how likely it is to attach to a receptor, and a receptor density that
/// determines how many receptors are available to attach to.
#[pyo3_stub_gen::derive::gen_stub_pyclass]
#[pyo3::pyclass]
#[derive(Clone, Copy, Debug, Default)]
pub struct MonoLigand {
    /// Density of receptors relative to the number of particles.
    pub receptor_density: f64,

    /// The strength the particle binds with. 
    pub binding_strength: f64,

    /// Rate at which the particle binds.
    pub on_rate: f64,

    /// Rate at which the particle unbinds. 
    pub off_rate: f64,
}

impl super::Particle for MonoLigand {
    type State = MonoLigandState;

    fn events(&self, state: &Self::State) -> Vec<Event<Self::State>> {
        if state.is_attached {
            vec![Event {
                rate: self.off_rate,
                transition: |state| state.toggle(),
            }]
        } else {
            vec![Event {
                rate: self.on_rate * self.receptor_density * self.binding_strength,
                transition: |state| state.toggle(),
            }]
        }
    }

    fn new_state(&self) -> Self::State {
        MonoLigandState { is_attached: false }
    }
}

#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pyo3::pymethods]
impl MonoLigand {
    #[new]
    fn new(receptor_density: f64, binding_strength: f64, on_rate: f64, off_rate: f64) -> Self {
        Self {
            receptor_density,
            binding_strength,
            on_rate,
            off_rate,
        }
    }

    fn simulate(&self) -> MonoLigandSimulationSingle {
        MonoLigandSimulationSingle::new(*self)
    }

    fn simulate_many(&self, n: usize) -> MonoLigandSimulation {
        MonoLigandSimulation::new(*self, n)
    }
}

crate::monomorphize!(
    MonoLigand,
    MonoLigandSimulation,
    MonoLigandSimulationSingle,
    MonoLiagndTransition
);
