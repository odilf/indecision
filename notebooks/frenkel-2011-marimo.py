import marimo

__generated_with = "0.10.17"
app = marimo.App(width="medium")


@app.cell
def _():
    import marimo as mo
    return (mo,)


@app.cell
def _():
    from indecision_rs import particle, simulate
    from matplotlib import pyplot as plt
    import numpy as np
    return np, particle, plt, simulate


@app.cell
def _(particle, plt):
    _p = particle.MonoLigand(
        receptor_density=0.3,
        binding_strength=1.0,
        on_rate=1.0,
        off_rate=0.1,
    )

    _simulation = _p.simulate_many(1000)
    _simulation.advance_until(20.0)

    _thetas = _simulation.thetas(samples=1000)

    plt.figure(dpi=300)
    plt.title("Binding of many mono-ligand particles over time")
    plt.plot(_thetas)
    plt.show()
    return


@app.cell
def _(np, particle):
    # receptor_densities = np.linspace(0.0001, 1.0, num=100)
    receptor_densities = np.logspace(-4.0, 0.0, num=100)
    # print(receptor_densities)

    N = 100_000
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
def _(N, data, plt, receptor_densities):
    plt.figure(dpi=300)

    for _binding_strength, _thetas in data.items():
        plt.plot(receptor_densities * N, _thetas, label=f"binding strength: {_binding_strength}")

    plt.xscale('log')
    plt.yscale('log')
    plt.title("Binding percentage of mono-ligand particles with respect to number of receptors")
    plt.legend()
    plt.show()
    return


@app.cell
def _(N, data, np, plt, receptor_densities):
    def plot_selectivites(data, convolve=10):
        plt.xscale('log')

        for _binding_strength, _thetas in data.items():
            _selectivity = np.gradient(np.log(_thetas), np.log(receptor_densities))
            _selectivity_smooth = np.convolve(_selectivity, np.ones(convolve)/convolve, 'same')
            plt.plot(receptor_densities * N, _selectivity_smooth, label=f"binding strength: {_binding_strength:.2E}")

        plt.legend(loc=(1.04, 0))
        plt.show()

    plt.figure(dpi=300)
    plt.title("Selectivity of mono-ligand particles with respect to number of receptors")
    plot_selectivites(data)
    return (plot_selectivites,)


@app.cell
def _(particle, plt):
    _p = particle.MultiLigand(
        receptor_density=0.3,
        binding_strength=1.0,
        on_rates=[1.0, 0.5, 0.25],
        off_rates=[1.0, 0.5, 0.25],
    )

    _simulation = _p.simulate_many(1000)
    _simulation.advance_until(10.0)

    _thetas = _simulation.thetas(samples=1000)

    plt.figure(dpi=300)
    plt.title("Binding of many multi-ligand particles over time")
    plt.plot(_thetas)
    plt.show()
    return


@app.cell
def _(np, particle):
    # receptor_densities = np.linspace(0.0001, 1.0, num=100)
    receptor_densities_multi = np.logspace(-4.0, 10.0, num=100)
    # print(receptor_densities)

    N_multi = 10_000
    data_multi = {}
    # for _binding_strength in [0.01, 0.5, 1.0, 5.0, 10.0, 40.0]:
    for _binding_strength in [*np.logspace(-9.0, 1.0, num=10), 10, 20, 40]:
        _thetas = []
        for _rd in receptor_densities_multi:
            _p = particle.MultiLigand(
                receptor_density=_rd,
                binding_strength=_binding_strength,
                on_rates=np.array([1.0, 1.0, 1.0, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 1.0]) * 1.0,
                off_rates=np.array([1.0, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 1.0]) * 1.0,
            )

            _simulation = _p.simulate_many(N_multi)
            _simulation.advance_until(100.0)

            _theta = _simulation.last_theta()
            _thetas.append(_theta)

        data_multi[_binding_strength] = _thetas
    return N_multi, data_multi, receptor_densities_multi


@app.cell
def _(N_multi, data_multi, plt, receptor_densities):
    plt.figure(dpi=300)
    plt.xscale('log')
    plt.yscale('log')

    for _binding_strength, _thetas in data_multi.items():
        plt.plot(receptor_densities * N_multi, _thetas, label=f"binding strength: {_binding_strength:.2E}")

    # plt.xlim(1e-5, 1e3)
    # plt.ylim(1e-5, 1e3)
    ax = plt.gca()
    ax.set_aspect('equal', adjustable='box')
    plt.legend(loc=(1.04, 0))
    plt.show()
    return (ax,)


@app.cell
def _(data_multi, plot_selectivites, plt):
    plt.figure(dpi=300)
    plt.title("Selectivity of multi-ligand particles with respect to number of receptors")
    plot_selectivites(data_multi)
    return


if __name__ == "__main__":
    app.run()
