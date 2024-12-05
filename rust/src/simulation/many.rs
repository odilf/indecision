use crate::particle::{self, Attach as _, Particle};

use super::SimulationSingle;

/// Invariant: All internal simulations must be on the same time.
#[derive(Clone, Debug, Default)]
pub struct Simulation<P: Particle> {
    pub simulations: Vec<SimulationSingle<P>>,
    /// To be able to derive Debug, Clone, etc...
    _phantom: std::marker::PhantomData<P::State>,
}

/// Invariant: All internal simulations must be on the same time.
impl<P: Particle> Simulation<P> {
    pub fn new(particle: P, n: usize) -> Self
    where
        P::State: Default,
        P: Clone,
    {
        Self {
            simulations: (0..n)
                .map(|_| SimulationSingle::new(particle.clone()))
                .collect(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn time(&self) -> f64 {
        // Uses invariant that all simulations are on the same time.
        self.simulations[0].time
    }

    pub fn advance_until(&mut self, t: f64)
    where
        P::State: Clone,
    {
        for simulation in &mut self.simulations {
            simulation.advance_until(t);
        }
    }

    pub fn states_at_time(&self, time: f64) -> Option<Vec<&P::State>> {
        self.simulations
            .iter()
            .map(|sim| sim.state_at_time(time))
            .collect()
    }

    pub fn sample(&self, samples: usize) -> impl Iterator<Item = Vec<&P::State>> {
        let step = self.time() / samples as f64;

        (0..samples).map(move |i| {
            self.states_at_time(i as f64 * step)
                .expect("We are within the bounds of 0 and simulation time")
        })
    }

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
