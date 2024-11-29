use std::any::Any;

use crate::particle::Particle;

#[derive(Clone, Copy, Debug, Default)]
pub struct Transition<State> {
    /// The time the event happened.
    at: f64,

    /// The state that it was transitioned to.
    to: State,
}

/// A simulation of a single particle
#[derive(Clone, Debug, Default)]
pub struct Simulation<P: Particle> {
    pub particle: P,
    pub time: f64,
    pub next_transition: Transition<P::State>,
    pub transition_history: Vec<Transition<P::State>>,
}

impl<P: Particle> Simulation<P> {
    pub fn new(particle: P) -> Self
    where
        P::State: Default,
    {
        Self {
            particle,
            time: 0.0,
            next_transition: Transition {
                at: 0.0,
                to: P::State::default(),
            },
            transition_history: Vec::new(),
        }
    }

    pub fn advance_until(&mut self, t: f64) where P::State: Clone {
        while t >= self.next_transition.at {
            // TODO: This might be doable more efficiently with `mem::swap` shenanigans
            self.transition_history.push(self.next_transition.clone());
            let (next_state, delta_t) = self.particle.advance_state(&self.next_transition.to).unwrap();

            self.next_transition = Transition {
                to: next_state,
                at: self.time + delta_t,
            };

            self.time += delta_t;
        }

        self.time = t; 
    }
}
