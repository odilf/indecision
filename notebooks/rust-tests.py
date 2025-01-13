import logging
import os

from indecision_rs import particle

FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(os.environ.get('LOGLEVEL', 'INFO') or logging.INFO)

mono_ligand = particle.MonoLigand(
    receptor_density=1.0,
    binding_strength=0.1,
    on_rate=1.0,
    off_rate=1.0,
)


multi_ligand = particle.MultiLigand(
    receptor_density=1.0,
    binding_strength=0.1,
    on_rates=[1.0, 0.5, 0.25],
    off_rates=[1.0, 0.5, 0.25],
)

interfering = particle.Interfering(
    receptor_density=1.0,
    binding_strength=0.1,
    on_rates=[1.0, 0.5, 0.25],
    off_rates=[1.0, 0.5, 0.25],
    enter_rate=1.0,
    obstruction_factor=0.8
)

particles = [ 
    mono_ligand,
    multi_ligand,
    interfering,
]

for p in particles:
    simulation = p.simulate_many(1000)
    simulation.advance_until(1000.0)

    thetas = simulation.thetas(samples=1000)
    logging.debug(f"Got {len(thetas)} values for {p}")


    states = p.states()
    logging.debug(f"There are {len(states)} for {p}")

    logging.debug(f"Probability of nexts states is {[prob for (_, prob) in p.event_probabilities(states[0])]}")

logging.info("\nAll seems correct!")

