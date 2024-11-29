from indecision_rs import particle, simulate, extract
p = particle.MonoLigand(
    binding_rate=0.3,
)

data = simulate.multiple(p, number_of_particles=1000, until=100_000.0)
thetas = extract.thetas(data)
selectivity = extract.thetas(selectivity)
