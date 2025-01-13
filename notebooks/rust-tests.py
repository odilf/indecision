from indecision_rs import particle, simulate

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
    print(f"Got {len(thetas)} values for {p}")


    states = p.states()
    print(f"States for {p} are {states}")

    print(f"Probability of nexts states from {states[0]} is {p.event_probabilities(states[0])}")

print("\nAll seems correct!")

