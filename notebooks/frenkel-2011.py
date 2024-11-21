from indecision import particle, simulation

p = particle.MultiLigandParticle(
    receptor_density=0.1, rates=[(0.1, 0.1), (0.5, 0.5), (0.9, 0.9)]
)

p = particle.MonoLigandParticle(
    receptor_density=0.1, on_rate=0.3, off_rate=0.8
)

# binding = [b for b in simulation.simulate_until_converge(
#     particle=p,
#     number_of_particles=1000,
#     extractor=lambda states: len(list(filter(lambda state: state.is_attached > 0, states))) / len(states),
# )]

# print(binding)
