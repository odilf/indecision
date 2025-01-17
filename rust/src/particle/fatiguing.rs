use pyo3::PyResult;

use crate::simulation::markov::MarkovChain;

use super::{Event, Particle};

/// A fatigue-interference model.
///
/// From the paper:
///
/// > The idea is that ligands, which were bound but disconnected again from the cell,
/// > don’t go back to their original state but are now considered "fatigued". They
/// > then receive a different, much lower, rate for attaching to the cell again. This
/// > way, the particles will have a chance to slowly detach again from the cell and
/// > eventually, when fully detached, get the opportunity to explore different cells,
/// > until they find the correct density.
#[pyo3_stub_gen::derive::gen_stub_pyclass]
#[pyo3::pyclass]
#[derive(Clone, Copy, Debug, Default)]
pub struct FatiguingState {
    #[pyo3(get)]
    has_entered: bool,

    #[pyo3(get)]
    attached_ligands: u16,

    #[pyo3(get)]
    fatigued_ligands: u16,
}

impl super::Attach for FatiguingState {
    fn is_attached(&self) -> bool {
        self.has_entered
    }
}

impl FatiguingState {
    pub fn toggle_entered(&self) -> Self {
        Self {
            has_entered: !self.has_entered,
            ..*self
        }
    }

    pub fn bind_regular(&self) -> Self {
        Self {
            attached_ligands: self.attached_ligands + 1,
            ..*self
        }
    }

    pub fn bind_fatigued(&self) -> Self {
        Self {
            attached_ligands: self.attached_ligands + 1,
            fatigued_ligands: self.fatigued_ligands - 1,
            ..*self
        }
    }

    pub fn unbind(&self) -> Self {
        Self {
            attached_ligands: self.attached_ligands - 1,
            fatigued_ligands: self.fatigued_ligands + 1,
            ..*self
        }
    }
}

/// TODO
#[pyo3_stub_gen::derive::gen_stub_pyclass]
#[pyo3::pyclass]
#[derive(Clone, Debug, Default)]
pub struct Fatiguing {
    /// Total number of ligands for the particle.
    #[pyo3(get)]
    total_ligands: u16,

    /// Rate at which an individual non-fatigued ligand attaches to a host.
    ///
    /// When you have `n` unattached ligands, the probability of going from `n` to `n + 1` attached
    /// ligands is `n * attachment_rate`.
    #[pyo3(get)]
    attachment_rate: f64,

    /// Rate at which an individual fatigued ligand attaches to a host.
    ///
    /// Multiplies the same way as [`Fatiguing::attachment_rate`]
    #[pyo3(get)]
    fatigued_attachment_rate: f64,

    /// Rate at which an individual ligand de-attaches from a host.
    ///
    /// Multiplies the same way as [`Fatiguing::attachment_rate`]
    #[pyo3(get)]
    deattachment_rate: f64,

    /// The rate at which an unobstructed particle enters the host.
    #[pyo3(get)]
    enter_rate: f64,

    /// Factor related to the increased difficulty of the initial ligand attaching as opposed to
    /// the rest of them.
    #[pyo3(get)]
    inital_collision_factor: f64,

    /// Factor by which the entering rate decrases for a non-fatigued ligand when a new ligand is attached.
    #[pyo3(get)]
    obstruction_factor: f64,

    /// Factor by which the entering rate decrases for a fatigued ligand when a new ligand is attached.
    #[pyo3(get)]
    fatigued_obstruction_factor: f64,

    /// The density of receptors available to bind to.
    ///
    /// `1.0` corresponds to one receptor per ligand.
    #[pyo3(get)]
    receptor_density: f64,

    /// The binding strength of the particle. It makes all transitions occur more often.
    #[pyo3(get)]
    binding_strength: f64,
}

impl super::Particle for Fatiguing {
    type State = FatiguingState;

    fn events(&self, state: &Self::State) -> Vec<Event<Self::State>> {
        if state.has_entered {
            return vec![Event { target: *state, rate: 0.0 }];
        }

        let entering = Event {
            target: state.toggle_entered(),
            rate: state.attached_ligands as f64
                * self.enter_rate
                * self
                    .obstruction_factor
                    .powi(state.attached_ligands as i32 - 1)
                * self
                    .fatigued_obstruction_factor
                    .powi(state.fatigued_ligands as i32),
        };

        let deattach = Event {
            target: state.unbind(),
            rate: state.attached_ligands as f64 * self.deattachment_rate,
        };

        let bind_regular = Event {
            target: state.bind_regular(),
            rate: self.free_ligands(*state) as f64 * self.attachment_rate,
        };

        let bind_fatigued = Event {
            target: state.bind_fatigued(),
            rate: state.fatigued_ligands as f64 * self.fatigued_attachment_rate,
        };

        vec![entering, deattach, bind_regular, bind_fatigued]
    }

    fn new_state(&self) -> Self::State {
        FatiguingState {
            attached_ligands: 0,
            fatigued_ligands: 0,
            has_entered: false,
        }
    }
}

impl MarkovChain for Fatiguing {
    fn states(&self) -> Vec<Self::State> {
        // Possibles states fro attached ligands and fatigued ligands form a triangle in state
        // space, so a reasonable estimate would be total ligands squared over 2, but we have to
        // multiply by 2 since we have a boolean `has_entered` so it just works out to
        // `total_ligands.pow(2)`.
        let mut output = Vec::with_capacity(self.total_ligands.pow(2) as usize);
        for attached_ligands in 0..=self.total_ligands {
            for fatigued_ligands in 0..=(self.total_ligands - attached_ligands) {
                for has_entered in [true, false] {
                    output.push(Self::State {
                        attached_ligands,
                        fatigued_ligands,
                        has_entered,
                    })
                }
            }
        }

        output
    }
}

crate::monomorphize!(
    Fatiguing {
        #[new]
        fn new(
            total_ligands: u16,
            attachment_rate: f64,
            fatigued_attachment_rate: f64,
            deattachment_rate: f64,
            enter_rate: f64,
            inital_collision_factor: f64,
            obstruction_factor: f64,
            fatigued_obstruction_factor: f64,
            receptor_density: f64,
            binding_strength: f64,
        ) -> PyResult<Self> {
            if obstruction_factor >= 1.0 {
                println!("WARNING: `obstruction_factor` should probably be less than 1.0 (is {obstruction_factor})");
            }

            Ok(Self {
            total_ligands,
            attachment_rate,
            fatigued_attachment_rate,
            deattachment_rate,
            enter_rate,
            inital_collision_factor,
            obstruction_factor,
            fatigued_obstruction_factor,
            receptor_density,
            binding_strength,
            })
        }

        /// The total amount of ligands the particle has.
        fn total_ligands(&self) -> u16 {
            self.total_ligands
        }

        /// The total amount of free ligands in a state's particle.
        fn free_ligands(&self, state: FatiguingState) -> u16 {
            self.total_ligands - state.fatigued_ligands - state.attached_ligands
        }
    },
    FatiguingSimulation,
    FatiguingSimulationSingle,
    FatiguingTransition
);
