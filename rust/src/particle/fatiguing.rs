use pyo3::PyResult;

use crate::simulation::markov::MarkovChain;

use super::{Event, Particle};

/// # Invariants
///
/// - `has_entered && has_exited == false`
#[pyo3_stub_gen::derive::gen_stub_pyclass]
#[pyo3::pyclass]
#[derive(Clone, Copy, Debug, Default, derive_more::Display)]
#[display("FatiguingState that {} entered, with {attached_ligands} attached ligands, of which {fatigued_ligands} are fatigued.", if *has_entered { "has" } else { "hasn't" })]
pub struct FatiguingState {
    #[pyo3(get)]
    has_entered: bool,

    #[pyo3(get)]
    has_exited: bool,

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
        if self.attached_ligands == 1 {
            return Self {
                has_exited: true,
                ..*self
            };
        }

        Self {
            attached_ligands: self.attached_ligands - 1,
            fatigued_ligands: self.fatigued_ligands + 1,
            ..*self
        }
    }
}

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
#[derive(Clone, Debug, Default)]
pub struct Fatiguing {
    /// Total number of ligands for the particle.
    #[pyo3(get)]
    pub total_ligands: u16,

    /// Rate at which an individual non-fatigued ligand attaches to a host.
    ///
    /// When you have `n` unattached ligands, the probability of going from `n` to `n + 1` attached
    /// ligands is `n * attachment_rate`.
    #[pyo3(get)]
    pub attachment_rate: f64,

    /// Rate at which an individual fatigued ligand attaches to a host.
    ///
    /// Multiplies the same way as [`Fatiguing::attachment_rate`]
    #[pyo3(get)]
    pub fatigued_attachment_rate: f64,

    /// Rate at which an individual ligand de-attaches from a host.
    ///
    /// Multiplies the same way as [`Fatiguing::attachment_rate`]
    #[pyo3(get)]
    pub deattachment_rate: f64,

    /// The rate at which an unobstructed particle enters the host.
    #[pyo3(get)]
    pub enter_rate: f64,

    /// Factor related to the increased difficulty of the initial ligand attaching as opposed to
    /// the rest of them.
    #[pyo3(get)]
    pub inital_collision_factor: f64,

    /// Factor by which the entering rate decrases for a non-fatigued ligand when a new ligand is attached.
    #[pyo3(get)]
    pub obstruction_factor: f64,

    /// Factor by which the entering rate decrases for a fatigued ligand when a new ligand is attached.
    #[pyo3(get)]
    pub fatigued_obstruction_factor: f64,

    /// The density of receptors available to bind to.
    ///
    /// `1.0` corresponds to one receptor per ligand.
    #[pyo3(get)]
    pub receptor_density: f64,
}

impl super::Particle for Fatiguing {
    type State = FatiguingState;

    fn events(&self, state: &Self::State) -> Vec<Event<Self::State>> {
        if state.has_entered || state.has_exited {
            return vec![Event {
                target: *state,
                rate: 0.0,
            }];
        }

        let mut output = Vec::with_capacity(4);

        if state.attached_ligands > 0 {
            output.push(Event {
                target: state.toggle_entered(),
                rate: state.attached_ligands as f64
                    * self.enter_rate
                    * self
                        .obstruction_factor
                        .powi(state.attached_ligands as i32 - 1)
                    * self
                        .fatigued_obstruction_factor
                        .powi(state.fatigued_ligands as i32),
            });

            output.push(Event {
                target: state.unbind(),
                rate: state.attached_ligands as f64 * self.deattachment_rate,
            });
        }

        output.push(Event {
            target: state.bind_regular(),
            rate: self.free_ligands(*state) as f64 * self.attachment_rate * self.receptor_density,
        });

        if state.fatigued_ligands > 0 {
            output.push(Event {
                target: state.bind_fatigued(),
                rate: state.fatigued_ligands as f64
                    * self.fatigued_attachment_rate
                    * self.receptor_density,
            });
        }

        output
    }

    fn new_state(&self) -> Self::State {
        FatiguingState {
            attached_ligands: 0,
            fatigued_ligands: 0,
            has_entered: false,
            has_exited: false,
        }
    }
}

impl MarkovChain for Fatiguing {
    fn states(&self) -> Vec<Self::State> {
        // Possibles states fro attached ligands and fatigued ligands form a triangle in state
        // space, so a reasonable estimate would be total ligands squared over 2, but we have to
        // multiply by 4 since we have two booleans, so it just works out to
        // `total_ligands.pow(2)`.
        let mut output = Vec::with_capacity(self.total_ligands.pow(2) as usize * 2);
        for attached_ligands in 0..=self.total_ligands {
            for fatigued_ligands in 0..=(self.total_ligands - attached_ligands) {
                for has_entered in [true, false] {
                    for has_exited in [true, false] {
                        output.push(Self::State {
                            attached_ligands,
                            fatigued_ligands,
                            has_entered,
                            has_exited,
                        })
                    }
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

#[test]
fn obstruction_factor_0_doesn_crash() {
    use crate::simulation::Simulation;

    let particle = Fatiguing {
        total_ligands: 5,
        attachment_rate: 1.0,
        fatigued_attachment_rate: 1.0,
        deattachment_rate: 1.0,
        enter_rate: 1.0,
        inital_collision_factor: 1.0,
        obstruction_factor: 0.0,
        fatigued_obstruction_factor: 0.0,
        receptor_density: 1.0,
    };

    let mut sim = Simulation::new(particle, 1000);
    sim.advance_until(1000.0).unwrap();
}
