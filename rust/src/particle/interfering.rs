use crate::simulation::markov::MarkovChain;

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
    /// Total number of ligands for the particle.
    #[pyo3(get)]
    pub total_ligands: u16,

    /// Rate at which an individual ligand attaches to a host.
    ///
    /// When you have `n` unattached ligands, the probability of going from `n` to `n + 1` attached
    /// ligands is `n * attachment_rate`.
    #[pyo3(get)]
    pub attachment_rate: f64,

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

    /// Factor by which the entering rate decrases for a ligand when a new ligand is attached.
    #[pyo3(get)]
    pub obstruction_factor: f64,

    /// The density of receptors available to bind to.
    ///
    /// `1.0` corresponds to one receptor per ligand.
    #[pyo3(get)]
    pub receptor_density: f64,
}

impl super::Particle for Interfering {
    type State = InterferingState;

    fn events(&self, state: &Self::State) -> Vec<Event<Self::State>> {
        // if state.has_entered {
        //     return vec![Event {
        //         target: *state,
        //         rate: 0.0,
        //     }];
        // }
        //
        // let mut events = Vec::with_capacity(3);
        //
        // if state.attached_ligands < self.total_ligands() {
        //     let rate = self.on_rate * state.attached_ligands
        //         * if state.attached_ligands == 0 {
        //             self.receptor_density
        //         } else {
        //             1.0
        //         }
        //         * self.binding_strength;
        //
        //     events.push(Event {
        //         rate,
        //         target: state.bind(),
        //     });
        // }
        //
        // if state.attached_ligands > 0 {
        //     events.push(Event {
        //         rate: self.off_rates[state.attached_ligands as usize - 1],
        //         target: state.unbind(),
        //     });
        // }
        //
        // if !state.has_entered {
        //     let obstruction = self.obstruction_factor.powi(state.attached_ligands as i32);
        //     events.push(Event {
        //         rate: self.enter_rate * obstruction * self.receptor_density,
        //         target: state.toggle_entered(),
        //     });
        // }
        //
        // events
        if state.has_entered {
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
                        .powi(state.attached_ligands as i32 - 1),
            });

            output.push(Event {
                target: state.unbind(),
                rate: state.attached_ligands as f64 * self.deattachment_rate,
            });
        }

        output.push(Event {
            target: state.bind(),
            rate: self.free_ligands(*state) as f64 * self.attachment_rate * self.receptor_density,
        });

        output
    }

    fn new_state(&self) -> Self::State {
        InterferingState {
            attached_ligands: 0,
            has_entered: false,
        }
    }
}

impl MarkovChain for Interfering {
    fn states(&self) -> Vec<Self::State> {
        let mut output = Vec::with_capacity(2 * self.total_ligands() as usize);
        for has_entered in [true, false] {
            for attached_ligands in 0..=self.total_ligands() {
                output.push(Self::State {
                    has_entered,
                    attached_ligands,
                });
            }
        }

        output
    }
}

crate::monomorphize!(
    Interfering {
        #[new]
        fn new(
            total_ligands: u16,
            attachment_rate: f64,
            deattachment_rate: f64,
            enter_rate: f64,
            inital_collision_factor: f64,
            obstruction_factor: f64,
            receptor_density: f64,
        ) -> Self {
            if obstruction_factor >= 1.0 {
                println!("WARNING: `obstruction_factor` should probably be less than 1.0 (is {obstruction_factor})");
            }

            Self {
                total_ligands,
                attachment_rate,
                deattachment_rate,
                enter_rate,
                inital_collision_factor,
                obstruction_factor,
                receptor_density,
            }
        }

        fn total_ligands(&self) -> u16 {
            self.total_ligands
        }

        /// The total amount of free ligands in a state's particle.
        fn free_ligands(&self, state: InterferingState) -> u16 {
            self.total_ligands - state.attached_ligands
        }
    },
    InterferingSimulation,
    InterferingSimulationSingle,
    InterferingTransition
);
