from typing import Any, Generator, Iterator, Type, get_args

from particle import Particle
from .convergence import ConvergenceCriterion

def simulate[State](
    particle: Particle[State],
    number_of_particles: int,
) -> Iterator[list[State]]:
    """
    Simulates a system of a particle.

    Returns a generator that goes on forever. Either use [`simulate_until_convergence`], [`simulate_until`] for a specific number of steps, or cap the number of steps from the outside (e.g., `[next(generator) for _ in range(n)]`),
    """
    states = [particle.state_type() for _ in range(number_of_particles)]

    yield states

    raise Exception("TODO: implement simulation")


def simulate_until[State](
    particle: Particle[State],
    steps: int,
    *args,
    **kwargs: dict[str, Any],
) -> Iterator[list[State]]:
    t = 0
    generator = simulate(particle, *args, **kwargs)

    while t < steps:
        yield next(generator)


def simulate_until_converge[State](
    particle: Particle[State],
    convergence: ConvergenceCriterion,
    *args,
    **kwargs,
) -> Iterator[list[State]]:
    generator = simulate(particle, *args, **kwargs)

    # [next(generator) for _ in range(steps)]
    raise Exception("TODO: implement convergence criteria")
