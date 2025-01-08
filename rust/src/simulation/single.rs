use crate::particle::Particle;

use super::Transition;

/// A simulation of a single particle
#[derive(Clone, Debug, Default)]
pub struct SimulationSingle<P: Particle> {
    /// The particle to be simulated.
    pub particle: P,

    pub(crate) next_transition: Transition<P::State>,
    pub(crate) transition_history: Vec<Transition<P::State>>,
    time: f64,
}

// Public API for Python
impl<P: Particle> SimulationSingle<P> {
    /// Constructs a new simulation with the given particle. 
    pub fn new(particle: P) -> Self {
        let initial_state = particle.new_state();

        Self {
            particle,
            time: 0.0,
            next_transition: Transition {
                time: 0.0,
                state: initial_state,
            },
            transition_history: Vec::new(),
        }
    }

    /// The current time of the simulation. 
    pub const fn time(&self) -> f64 {
        self.time
    }

    /// Advances the simulation until at least time `t`. 
    ///
    /// If the time is already more than `t`, the simulation doesn't advance. 
    pub fn advance_until(&mut self, t: f64)
    where
        P::State: Clone,
    {
        while t >= self.next_transition.time {
            // TODO: This might be doable more efficiently with `mem::swap` shenanigans
            self.transition_history.push(self.next_transition.clone());
            let (next_state, delta_t) = self
                .particle
                .advance_state(&self.next_transition.state)
                .unwrap();

            self.next_transition = Transition {
                state: next_state,
                time: self.time + delta_t,
            };

            self.time += delta_t;
        }

        self.time = t;
    }

    /// Makes a new [`Simulation`](super::Simulation) with `n` particles of the current kind.
    pub fn multiple(self, n: usize) -> super::Simulation<P>
    where
        P: Clone,
        P::State: Default,
    {
        super::Simulation::new(self.particle, n)
    }
}

// Private API for Python
impl<P: Particle> SimulationSingle<P> {
    /// The last transition that had occurrured at the given time.
    ///
    /// If the time is the same as a transition, that transition is returned.
    ///
    /// It will return `None` if the time is before the first transition or after the next
    /// scheduled transition.
    pub fn last_transition_at_time(&self, time: f64) -> Option<&Transition<P::State>> {
        let mut last_transition = None;
        for transition in &self.transition_history {
            if transition.time > time {
                return last_transition;
            } else if transition.time == time {
                return Some(transition);
            }

            last_transition = Some(transition)
        }

        if self.next_transition.time > time {
            last_transition
        } else {
            None
        }
    }

    /// The state of the particle at the given time.
    pub fn state_at_time(&self, time: f64) -> Option<&P::State> {
        Some(&self.last_transition_at_time(time)?.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::particle::MonoLigand;

    #[test]
    fn state_at_time() {
        let particle = MonoLigand {
            receptor_density: 1.0,
            binding_strength: 1.0,
            on_rate: 1.0,
            off_rate: 1.0,
        };

        let mut sim = SimulationSingle::new(particle);
        sim.advance_until(1.0);
        sim.state_at_time(0.0).unwrap();
    }
}
