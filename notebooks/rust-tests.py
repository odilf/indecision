from indecision_rs import particle, simulate  # , extract
from matplotlib import pyplot as plt

p = particle.MonoLigand(
    receptor_density=1.0,
    binding_strength=0.1,
    on_rate=0.1,
    off_rate=0.1,
)

# simulation = simulate.multiple(
#     p, 
#     number_of_particles=1000, 
#     until=100_000.0
# )

simulation = p.simulate_many(1000)
simulation.advance_until(1000.0)

thetas = simulation.thetas(samples=1000)

plt.plot(thetas)

# data = simulate.multiple(p, number_of_particles=1000, until=100_000.0)
# thetas = extract.thetas(data)
# selectivity = extract.thetas(selectivity)
