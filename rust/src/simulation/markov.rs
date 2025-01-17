//! Modeling of particles as [Markov chains](https://en.wikipedia.org/wiki/Markov_chain)

use crate::particle::Particle;

/// Particles that can be represented as a [Markov chain](https://en.wikipedia.org/wiki/Markov_chain)
pub trait MarkovChain: Particle {
    /// Enumeration of all possible states for the particle.
    fn states(&self) -> Vec<Self::State>;
}
