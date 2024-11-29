# from indecision import particle, simulation
# from indecision.particle.multi_ligand import MultiLigandState
# import numpy as np

# p = particle.MultiLigandParticle(
#     receptor_density=0.1, rates=[(0.1, 0.1), (0.5, 0.5), (0.9, 0.9)]
# )
#
# # p = particle.MonoLigandParticle(
# #     receptor_density=0.1, on_rate=0.3, off_rate=0.8
# # )
#
# convergence = simulation.convergence.ConvergenceCriterion[MultiLigandState](
#     metric=lambda a, b: np.abs(a.attached_ligands - b.attached_ligands),
#     tolerance=0.01,
#     window_size=10,
#     sample_size=100,
# )
#
# # binding = [b for b in simulation.simulate_until_converge(
# #     particle=p,
# #     number_of_particles=1000,
# #     extractor=lambda states: len(list(filter(lambda state: state.is_attached > 0, states))) / len(states),
# # )]
#
# # print(binding)
