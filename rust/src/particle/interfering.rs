use crate::simulation::markov::MarkovChain;

use super::Event;

/// # Invariants
///
/// - `has_entered && has_exited == false`
#[cfg_attr(
    feature = "python-build-stubs",
    pyo3_stub_gen::derive::gen_stub_pyclass
)]
#[cfg_attr(feature = "python", pyo3::pyclass(get_all))]
#[derive(Clone, Copy, Debug, Default)]
pub struct InterferingState {
    has_entered: bool,
    has_exited: bool,
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
        if self.attached_ligands == 1 {
            return Self {
                attached_ligands: 0,
                has_exited: true,
                ..*self
            };
        }

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
#[cfg_attr(
    feature = "python-build-stubs",
    pyo3_stub_gen::derive::gen_stub_pyclass
)]
#[cfg_attr(feature = "python", pyo3::pyclass(get_all, set_all))]
#[derive(Clone, Debug, Default)]
pub struct Interfering {
    /// Total number of ligands for the particle.
    pub total_ligands: u16,

    /// Rate at which an individual ligand attaches to a host.
    ///
    /// When you have `n` unattached ligands, the probability of going from `n` to `n + 1` attached
    /// ligands is `n * attachment_rate`.
    pub attachment_rate: f64,

    /// Rate at which an individual ligand de-attaches from a host.
    ///
    /// Multiplies the same way as [`Fatiguing::attachment_rate`]
    pub deattachment_rate: f64,

    /// The rate at which an unobstructed particle enters the host.
    pub enter_rate: f64,

    /// Factor related to the increased difficulty of the initial ligand attaching as opposed to
    /// the rest of them.
    pub inital_collision_factor: f64,

    /// Factor by which the entering rate decrases for a ligand when a new ligand is attached.
    pub obstruction_factor: f64,

    /// The density of receptors available to bind to.
    ///
    /// `1.0` corresponds to one receptor per ligand.
    pub receptor_density: f64,
}

impl super::Particle for Interfering {
    type State = InterferingState;

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
            has_exited: false,
        }
    }
}

impl MarkovChain for Interfering {
    fn states(&self) -> Vec<Self::State> {
        let mut output = Vec::with_capacity(2 * self.total_ligands() as usize);
        for has_entered in [true, false] {
            for has_exited in [true, false] {
                for attached_ligands in 0..=self.total_ligands() {
                    output.push(Self::State {
                        has_entered,
                        attached_ligands,
                        has_exited,
                    });
                }
            }
        }

        output
    }
}

crate::monomorphize!(
    Interfering {
        #[cfg(feature = "python")]
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
