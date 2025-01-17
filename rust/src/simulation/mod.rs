//! Simulation of [`particles`](crate::particle) over time.

mod many;
pub mod markov;
mod single;

pub use many::Simulation;
pub use single::SimulationSingle;

/// A transition event into a particular `State` that happened at some point in time.
#[derive(Clone, Copy, Debug, Default)]
pub struct Transition<State> {
    /// The time at which the event happened.
    pub time: f64,

    /// The state that it was transitioned to.
    pub target: State,
}
