# TODO: Make it actually a notebook

from simulation import simulate_until_converge
from particle import MultiLigandParticle


import numpy as np

n_rs = np.expspace(1e0, 1e400, 400)

data_points = simulate_until_converge(
    MultiLigandParticle(),
    number_of_particles=1000,
    convergence=None,
)
