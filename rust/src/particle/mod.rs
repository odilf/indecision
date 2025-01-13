//! Particle implementations

// mod fatiguing;
mod interfering;
mod mono_ligand;
mod multi_ligand;
// mod walker;

// pub use fatiguing::Fatiguing;
pub use interfering::Interfering;
pub use mono_ligand::MonoLigand;
pub use multi_ligand::MultiLigand;
// pub use walker::Walker;

use color_eyre::eyre;

/// Particles that can output a binary "I'm attached" or "I'm not attached".
pub trait Attach {
    /// Whether to count the particle as attached to the receptor or not.
    fn is_attached(&self) -> bool;
}

/// Trait representing a nano-particle that can be simulated using [`crate::simulation`].
///
/// It has the state of the particle as an associated type, and two required methods for returing a
/// list of events for each state and for generating new states.
pub trait Particle {
    /// A type that represents the state that the particle can be in.
    type State;

    /// Returns the possible events that can happen to the particle in the current state
    fn events(&self, state: &Self::State) -> Vec<Event<Self::State>>;

    /// Create a new state initial for the particle.
    fn new_state(&self) -> Self::State;

    /// A list of probabilities for each possible next state.
    ///
    /// If a state is not contained in the list it can be assumed is 0.
    fn event_probabilities(&self, state: &Self::State) -> eyre::Result<Vec<(Self::State, f64)>> {
        let events = self.events(state);
        if events.is_empty() {
            eyre::bail!("No events to process");
        }

        let total_rate = events.iter().map(|e| e.rate).sum::<f64>();
        if total_rate == 0.0 {
            eyre::bail!("Total rate of events is 0, no transitions are possible");
        };

        // TODO: We could leave this as an iterator. I didn't because the generics where being a
        // pain.
        let output = events
            .into_iter()
            .map(move |event| (event.target, event.rate / total_rate))
            .collect();

        Ok(output)
    }

    /// Advances in-place the state of a particle, and returns the time elapsed to make that transition.
    fn advance_state(&self, state: &Self::State) -> eyre::Result<(Self::State, f64)> {
        let events = self.events(state);
        if events.is_empty() {
            eyre::bail!("No events to process");
        }

        let total_rate = events.iter().map(|e| e.rate).sum::<f64>();
        if total_rate == 0.0 {
            eyre::bail!("Total rate of events is 0, no transitions are possible");
        };

        let delta_t = -rand::random::<f64>().log2() / total_rate;
        let r = rand::random::<f64>() * total_rate;

        let mut cumulative_rate = 0.0;
        for event in events {
            cumulative_rate += event.rate;
            if cumulative_rate > r {
                let next_state = event.target;
                return Ok((next_state, delta_t));
            }
        }

        // Maybe we want to remove this unsafe

        // SAFETY: `rand::random` generates the half-open range `[0, 1)`, so `r` is between `[0,
        // total_rate]`. `total_rate` is the sum of all `event.rate`s, and in the loop we
        // eventually add all of the `event.rate`s. Therefore, the loop can only exit if
        // `cumulative_rate` is equal to `total_rate`, but since `r` is always less than
        // `total_rate`, the loop will never exit. Ergo, this function is never called.
        unsafe { std::hint::unreachable_unchecked() }
    }

    /// Creates a simulation object for this particle.
    fn simulation(self) -> crate::simulation::SimulationSingle<Self>
    where
        Self: Sized,
    {
        crate::simulation::SimulationSingle::new(self)
    }
}

/// A transition that happens at some rate.
#[derive(Clone, Copy, Debug)]
pub struct Event<State> {
    /// The amount of times that this event occurs per unit time.
    pub rate: f64,

    /// The state after the transition.
    pub target: State,
}

/// Generates concrete types from the generic types, for use in Python.
///
/// Note: All doc-comments have to be manually copied from the implementation and kept up-to-date.
#[macro_export]
macro_rules! monomorphize {
    ($type:path $({ $($impls:tt)* })?, $simulation:ident, $simulation_single:ident, $transition:ident $(,)?) => {
        #[pyo3_stub_gen::derive::gen_stub_pymethods]
        #[pyo3::pymethods]
        impl $type {
            /// Create a new single-particle simulation from this particle.
            fn simulate(&self) -> $simulation_single {
                $simulation_single::new(self.clone())
            }

            /// Create a new `n`-particle simulation from this particle.
            fn simulate_many(&self, n: usize) -> $simulation {
                $simulation::new(self.clone(), n)
            }

            #[pyo3(name = "states")]
            fn states_python(&self) -> Vec<<$type as Particle>::State> {
                $crate::simulation::markov::MarkovChain::states(self)
            }

            /// A list of probabilities for each possible next state.
            ///
            /// If a state is not contained in the list it can be assumed is 0.
            #[pyo3(name = "event_probabilities")]
            fn event_probabilities_python(
                &self, 
                state: &<$type as Particle>::State
            ) -> pyo3::PyResult<Vec<(<$type as Particle>::State, f64)>> {
                self.event_probabilities(state)
                    .map_err(|err| ::pyo3::exceptions::PyException::new_err(err.to_string()))
            }

            $($($impls)*)?
        }


        #[pyo3_stub_gen::derive::gen_stub_pyclass]
        #[pyo3::pyclass]
        pub struct $simulation {
            pub inner: $crate::simulation::Simulation<$type>,
        }

        #[pyo3_stub_gen::derive::gen_stub_pymethods]
        #[pyo3::pymethods]
        impl $simulation {
            /// Constructs a new simulation of this particle.
            #[new]
            pub fn new(particle: $type, n: usize) -> Self {
                Self {
                    inner: $crate::simulation::Simulation::new(particle, n),
                }
            }

            ///
            pub fn sample(&self, samples: usize) -> Vec<Vec<<$type as Particle>::State>> {
                self.inner
                    .sample(samples)
                    .map(|s| s.into_iter().map(|v| v.clone()).collect())
                    .collect()
            }

            pub fn thetas(&self, samples: usize) -> Vec<f64> {
                self.inner.thetas(samples).collect()
            }

            pub fn last_theta(&self) -> f64 {
                self.inner.last_theta()
            }

            pub fn advance_until(&mut self, t: f64) -> pyo3::PyResult<()> {
                self.inner.advance_until(t).map_err(|err| ::pyo3::exceptions::PyException::new_err(err.to_string()))
            }
        }

        #[pyo3_stub_gen::derive::gen_stub_pyclass]
        #[pyo3::pyclass]
        pub struct $simulation_single {
            pub inner: $crate::simulation::SimulationSingle<$type>,
        }

        #[pyo3_stub_gen::derive::gen_stub_pymethods]
        #[pyo3::pymethods]
        impl $simulation_single {
            #[new]
            pub fn new(particle: $type) -> Self {
                Self {
                    inner: $crate::simulation::SimulationSingle::new(particle),
                }
            }

            #[getter]
            pub fn transition_history(&self) -> Vec<$transition> {
                self.inner
                    .transition_history
                    .iter()
                    .map(|t| $transition { inner: *t })
                    .collect()
            }

            pub fn advance_until(&mut self, t: f64) -> pyo3::PyResult<()> {
                self.inner.advance_until(t).map_err(|err| pyo3::exceptions::PyException::new_err(err.to_string()))
            }
        }

        #[pyo3_stub_gen::derive::gen_stub_pyclass]
        #[pyo3::pyclass]
        pub struct $transition {
            pub inner: $crate::simulation::Transition<<$type as Particle>::State>,
        }

        #[pyo3_stub_gen::derive::gen_stub_pymethods]
        #[pyo3::pymethods]
        impl $transition {
            #[getter]
            /// The time at which the event happened.
            pub fn time(&self) -> f64 {
                self.inner.time
            }

            #[getter]
            /// The state that it was transitioned _to_.
            pub fn target(&self) -> <$type as Particle>::State {
                self.inner.target
            }
        }
    };
}
