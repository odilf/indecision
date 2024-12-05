import marimo

__generated_with = "0.9.31"
app = marimo.App(width="medium")


@app.cell
def __():
    import marimo as mo
    return (mo,)


@app.cell
def __():
    from indecision_rs import particle, simulate
    from matplotlib import pyplot as plt
    import numpy as np
    return np, particle, plt, simulate


@app.cell
def __(particle, plt):
    _p = particle.MonoLigand(
        receptor_density=0.3,
        binding_strength=10.0,
        on_rate=1.0,
        off_rate=1.0,
    )

    _simulation = _p.simulate_many(1000)
    _simulation.advance_until(10.0)

    _thetas = _simulation.thetas(samples=1000)

    plt.title("Binding of many mono-ligand particles over time")
    plt.plot(_thetas)
    plt.show()
    return


@app.cell
def __(np, particle):
    # receptor_densities = np.linspace(0.0001, 1.0, num=100)
    receptor_densities = np.logspace(-4.0, 0.0, num=100)
    # print(receptor_densities)

    N = 10_000
    data = {}
    for binding_strength in [1.0, 5.0, 10.0, 40.0]:
        _thetas = []
        for rd in receptor_densities:
            p = particle.MonoLigand(
                receptor_density=rd,
                binding_strength=binding_strength,
                on_rate=0.1,
                off_rate=0.1,
            )

            simulation = p.simulate_many(N)
            simulation.advance_until(100.0)

            theta = simulation.last_theta()
            _thetas.append(theta)

        data[binding_strength] = _thetas
    return (
        N,
        binding_strength,
        data,
        p,
        rd,
        receptor_densities,
        simulation,
        theta,
    )


@app.cell
def __(N, data, plt, receptor_densities):
    plt.xscale('log')
    plt.yscale('log')

    for _binding_strength, _thetas in data.items():
        plt.plot(receptor_densities * N, _thetas, label=f"binding strength: {_binding_strength}")

    plt.title("Binding percentage of mono-ligand particles with respect to number of receptors")
    plt.legend()
    plt.show()
    return


@app.cell
def __(data, plt):
    plt.xscale('log')
    plt.yscale('log')

    for _binding_strength, _thetas in data.items():
        ...
        # TODO:
        # plt.plot(receptor_densities * N, _thetas, label=f"binding strength: {_binding_strength}")

    plt.title("Selectivity of mono-ligand particles with respect to number of receptors")
    plt.legend()
    plt.show()
    return


@app.cell
def __(particle, plt):
    _p = particle.MultiLigand(
        receptor_density=0.3,
        binding_strength=1.0,
        on_rates=[1.0, 0.5, 0.25],
        off_rates=[1.0, 0.5, 0.25],
    )

    _simulation = _p.simulate_many(1000)
    _simulation.advance_until(10.0)

    _thetas = _simulation.thetas(samples=1000)

    plt.title("Binding of many multi-ligand particles over time")
    plt.plot(_thetas)
    plt.show()
    return


@app.cell
def __(N, np, particle):
    # receptor_densities = np.linspace(0.0001, 1.0, num=100)
    receptor_densities_multi = np.logspace(-4.0, 0.0, num=100)
    # print(receptor_densities)

    N_multi = 10_000
    data_multi = {}
    for _binding_strength in [0.5, 1.0, 5.0, 10.0, 40.0]:
        _thetas = []
        for _rd in receptor_densities_multi:
            _p = particle.MultiLigand(
                receptor_density=_rd,
                binding_strength=_binding_strength,
                on_rates=np.array([1.0, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 1.0]) * 1.0,
                off_rates=np.array([1.0, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 1.0]) * 1.0,
            )

            _simulation = _p.simulate_many(N)
            _simulation.advance_until(100.0)

            _theta = _simulation.last_theta()
            _thetas.append(_theta)

        data_multi[_binding_strength] = _thetas
    return N_multi, data_multi, receptor_densities_multi


@app.cell
def __(N_multi, data_multi, plt, receptor_densities):
    plt.xscale('log')
    plt.yscale('log')

    for _binding_strength, _thetas in data_multi.items():
        plt.plot(receptor_densities * N_multi, _thetas, label=f"binding strength: {_binding_strength}")

    plt.legend()
    plt.show()
    return


if __name__ == "__main__":
    app.run()
