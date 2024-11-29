mod mono_ligand;

pub use mono_ligand::MonoLigand;

use color_eyre::eyre;

pub trait Particle {
    type State;

    fn events(&self, state: &Self::State) -> Vec<Event<Self::State>>;

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
                let next_state = (event.transition)(state);
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

    fn simulation(self) -> crate::simulation::Simulation<Self>
    where
        Self: Sized,
        Self::State: Default,
    {
        crate::simulation::Simulation::new(self)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Event<State> {
    pub rate: f64,
    pub transition: fn(&State) -> State,
}
