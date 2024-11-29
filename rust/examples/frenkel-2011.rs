use color_eyre::eyre;
use indecision::particle::{self, Particle as _};

fn main() -> eyre::Result<()> {
    let particle = particle::MonoLigand {
        receptor_density: 0.5,
        binding_strength: 0.4,

        on_rate: 0.1,
        off_rate: 0.1,
    };

    let number_of_particles = 1000;

    let mut simulations = (0..number_of_particles)
        .map(|_| particle.simulation())
        .collect::<Vec<_>>();

    for sim in &mut simulations {
        sim.advance_until(1_000_000.0);
    }

    for sim in &simulations {
        dbg!(sim.transition_history.len());
        dbg!(sim.time);
    }

    Ok(())
}
