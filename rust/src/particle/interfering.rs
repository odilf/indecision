use pyo3::PyResult;

use super::{Event, Particle};

#[pyo3_stub_gen::derive::gen_stub_pyclass]
#[pyo3::pyclass]
#[derive(Clone, Copy, Debug, Default)]
pub struct InterferingState {
    #[pyo3(get, set)]
    has_entered: bool,

    #[pyo3(get, set)]
    attached_ligands: u16,
}

impl super::Attach for InterferingState {
    fn is_attached(&self) -> bool {
        self.has_entered
    }
}

impl InterferingState {
    pub fn toggle_entered(&self) -> Self {
        Self {
            has_entered: !self.has_entered,
            ..*self
        }
    }

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

/// A multi-valent particle that can attach and enter a host. 
///
/// Ligands obtruct the particle from entering, where for each additional attached ligand,
/// the entering rate is decreased by a constant factor. 
#[pyo3_stub_gen::derive::gen_stub_pyclass]
#[pyo3::pyclass]
#[derive(Clone, Debug, Default)]
pub struct Interfering {
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

    /// The rate at which an unobstructed particle enters the host.
    pub enter_rate: f64,

    /// Factor by which the entering rate decrases when a new ligand is attached.
    pub obstruction_factor: f64,
}

impl super::Particle for Interfering {
    type State = InterferingState;

    fn events(&self, state: &Self::State) -> Vec<Event<Self::State>> {
        let mut events = Vec::with_capacity(2);

        if state.attached_ligands < self.max_ligands() {
            let rate = self.on_rates[state.attached_ligands as usize]
                * if state.attached_ligands == 0 {
                    self.receptor_density
                } else {
                    1.0
                }
                * self.binding_strength;

            events.push(Event {
                rate,
                transition: Self::State::bind,
            });
        }

        if state.attached_ligands > 0 {
            events.push(Event {
                rate: self.off_rates[state.attached_ligands as usize - 1],
                transition: Self::State::unbind,
            });
        }

        if !state.has_entered {
            let obstruction = self.obstruction_factor.powi(state.attached_ligands as i32);
            events.push(Event {
                rate: self.enter_rate * obstruction * self.receptor_density,
                transition: Self::State::toggle_entered,
            });
        }

        events
    }

    fn new_state(&self) -> Self::State {
        InterferingState {
            attached_ligands: 0,
            has_entered: false,
        }
    }
}

#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pyo3::pymethods]
impl Interfering {
    #[new]
    fn new(
        receptor_density: f64,
        binding_strength: f64,
        on_rates: Vec<f64>,
        off_rates: Vec<f64>,
        enter_rate: f64,
        obstruction_factor: f64,
    ) -> PyResult<Self> {
        if on_rates.len() != off_rates.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "on_rates and off_rates must have the same length",
            ));
        }

        if obstruction_factor >= 1.0 {
            println!("WARNING: `obstruction_factor` should probably be less than 1.0 (is {obstruction_factor})");
        }

        Ok(Self {
            receptor_density,
            binding_strength,
            on_rates,
            off_rates,
            enter_rate,
            obstruction_factor,
        })
    }

    fn max_ligands(&self) -> u16 {
        assert_eq!(self.on_rates.len(), self.off_rates.len());
        self.on_rates.len() as u16
    }

    fn simulate(&self) -> InterferingSimulationSingle {
        InterferingSimulationSingle::new(self.clone())
    }

    fn simulate_many(&self, n: usize) -> InterferingSimulation {
        InterferingSimulation::new(self.clone(), n)
    }
}

crate::monomorphize!(
    Interfering,
    InterferingSimulation,
    InterferingSimulationSingle,
    InterferingTransition
);
