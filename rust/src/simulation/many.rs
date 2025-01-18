use color_eyre::eyre;
use rayon::prelude::*;

use crate::particle::{self, Attach as _, Particle};

use super::{SimulationSingle, Transition};

/// Simulation of many particles at once.
///
/// # Invariants:
/// - All internal simulations must be on the same time.
#[derive(Clone, Debug, Default)]
pub struct Simulation<P: Particle> {
    /// All individual simulations.
    ///
    /// # Invariants:
    /// - All internal simulations must be on the same time.
    pub simulations: Vec<SimulationSingle<P>>,

    /// To be able to derive Debug, Clone, etc...
    _phantom: std::marker::PhantomData<P::State>,
}

/// Invariant: All internal simulations must be on the same time.
impl<P: Particle> Simulation<P> {
    /// Contructs a new simulation with `n` particles of the given kind.
    pub fn new(particle: P, n: usize) -> Self
    where
        P: Clone,
    {
        Self {
            simulations: (0..n)
                .map(|_| SimulationSingle::new(particle.clone()))
                .collect(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// The current time of the simulation.
    pub fn time(&self) -> f64 {
        // Uses invariant that all simulations are on the same time.
        self.simulations[0].time()
    }

    /// Advances the simulation until a particular time.
    ///
    /// See also [`SimulationSingle::advance_until`].
    pub fn advance_until(&mut self, t: f64) -> eyre::Result<()>
    where
        P::State: Clone,
        P: Send + Sync,
        P::State: Send + Sync,
    {
        self.simulations
            .par_iter_mut()
            .map(|sim| sim.advance_until(t))
            .collect()
    }

    /// The transition histories of all simulations.
    ///
    /// Just in case, it is returned as a list of transition histories, not the other way around.
    pub fn transition_histories(&self) -> impl Iterator<Item = &Vec<Transition<P::State>>> {
        self.simulations.iter().map(|sim| &sim.transition_history)
    }

    /// Collects a vector of the states of all simulations at a particular time.
    ///
    /// Returns [`None`] if it's outside of the simulated time-range.
    ///
    /// Calling this function at time = [`Self::time`] is guarantedd to return [`Some`].
    pub fn states_at_time(&self, time: f64) -> Option<Vec<&P::State>> {
        self.simulations
            .iter()
            .map(|sim| sim.state_at_time(time))
            .collect()
    }

    /// Takes `n` evenly spaced samples between `0` and [`Self::time`], using
    /// [`Self::states_at_time`].
    pub fn sample(&self, n: usize) -> impl Iterator<Item = Vec<&P::State>> {
        let step = self.time() / n as f64;

        (0..n).map(move |i| {
            self.states_at_time(i as f64 * step)
                .expect("We are within the bounds of 0 and simulation time")
        })
    }

    /// Returns the states at the last point in the simulation.
    pub fn last_states(&self) -> Vec<&P::State> {
        self.states_at_time(self.time())
            .expect("We are on a valid time")
    }

    /// Returns the attachment percentage at the last point in time (i.e., [`Self::time`]),
    /// commonly denoted with the greek theta (θ).
    pub fn last_theta(&self) -> f64
    where
        P::State: particle::Attach,
    {
        self.states_at_time(self.time())
            .expect("We are on a valid time")
            .iter()
            .filter(|&&state| state.is_attached())
            .count() as f64
            / self.simulations.len() as f64
    }

    /// Returns the attachment percentage (theta), at evenly spaced samples (using [`Self::sample`]).
    pub fn thetas(&self, samples: usize) -> impl Iterator<Item = f64> + use<'_, P>
    where
        P::State: particle::Attach,
    {
        self.sample(samples).map(|states| {
            let attached = states.iter().filter(|&&state| state.is_attached()).count();

            attached as f64 / states.len() as f64
        })
    }
}
