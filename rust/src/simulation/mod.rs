mod single;
pub mod many;

pub use single::SimulationSingle;
pub use many::Simulation;

#[derive(Clone, Copy, Debug, Default)]
pub struct Transition<State> {
    /// The time at which the event happened.
    pub time: f64,

    /// The state that it was transitioned to.
    pub state: State,
}
