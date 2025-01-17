//! Particle implementations

mod fatiguing;
mod interfering;
mod mono_ligand;
mod multi_ligand;
// mod walker;

pub use fatiguing::Fatiguing;
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
    fn event_probabilities(&self, state: &Self::State) -> impl Iterator<Item = (Self::State, f64)> {
        let events = self.events(state);
        let total_rate = events.iter().map(|e| e.rate).sum::<f64>();
        if total_rate == 0.0 {
            log::debug!("Total rate of events is 0, no transitions are possible");
        };

        events
            .into_iter()
            .map(move |event| (event.target, event.rate / total_rate))
    }

    /// Advances in-place the state of a particle, and returns the time elapsed to make that transition.
    // TODO: This doesn't need to be a result anymore.
    fn advance_state(&self, state: &Self::State) -> eyre::Result<(Self::State, f64)> {
        let events = self.events(state);

        // Note: we always need one event because we need to have a return value for the function.
        // If no more transitions should occur, you should still have at least one event with rate
        // 0.0. Then it would place the theoretical transition to said event at `t == infinity`,
        // but we need some valid state to put there. Using an `Option` might technically be more
        // idiomatic, but it's a hassle for little benefit. And an event with rate 0.0 is still
        // decently idiomatic.
        if events.len() == 0 {
            eyre::bail!("No events to process.");
        }

        let total_rate = events
            .iter()
            .inspect(|e| assert!(!e.rate.is_nan()))
            .map(|e| e.rate)
            .sum::<f64>();

        if total_rate == 0.0 {
            log::debug!("Total rate of events is 0.0, no more transitions will ocurr");
        };

        let delta_t = -rand::random::<f64>().log2() / total_rate;

        let r = rand::random::<f64>() * total_rate;

        let mut cumulative_rate = 0.0;
        for event in events {
            cumulative_rate += event.rate;
            if cumulative_rate >= r {
                let next_state = event.target;
                return Ok((next_state, delta_t));
            }
        }

        // SAFETY: This unsafe should never be hit. This unsafe is hit if the loop is exited, so
        // the loop should never be exited.
        //
        // We assert at the top that there are more than 0 events, so the loop is entered.
        //
        // Inside the loop, `rand::random` generates the half-open range `[0, 1)`, so `r` is between `[0,
        // total_rate)`. The last value of `cumulative_rate` before exiting the loop will be
        // `total_rate`, since we add up all the `event.rate`s, which is preciesly how we obtained
        // `total_rate` in the first place. But we return from the function if `cumulative_rate` is
        // greater than `r`. Therefore, it is impossible to exit the loop because `cumulative_rate`
        // has to be equal to `total_rate`, but that means that it will have been at some point
        // greater than `r`, which returns from the function.
        //
        // Ergo, this block is never reached.
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
            ) -> Vec<(<$type as Particle>::State, f64)> {
                self.event_probabilities(state).collect()
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

            pub fn sample(&self, samples: usize) -> Vec<Vec<<$type as Particle>::State>> {
                self.inner
                    .sample(samples)
                    .map(|s| s.into_iter().map(|v| v.clone()).collect())
                    .collect()
            }

            pub fn thetas(&self, samples: usize) -> Vec<f64> {
                self.inner.thetas(samples).collect()
            }

            /// Returns the states at the last point in the simulation.
            pub fn last_states(&self) -> Vec<<$type as Particle>::State> {
                self.inner.last_states().into_iter().cloned().collect()
            }

            pub fn last_theta(&self) -> f64 {
                self.inner.last_theta()
            }

            pub fn advance_until(&mut self, t: f64, py: ::pyo3::Python<'_>) -> pyo3::PyResult<()> {
                py.allow_threads(|| {
                    self.inner.advance_until(t).map_err(|err| ::pyo3::exceptions::PyException::new_err(err.to_string()))
                })
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

            pub fn advance_until(&mut self, t: f64, py: ::pyo3::Python<'_>) -> pyo3::PyResult<()> {
                py.allow_threads(|| {
                    self.inner.advance_until(t).map_err(|err| pyo3::exceptions::PyException::new_err(err.to_string()))
                })
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
