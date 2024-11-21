from typing import Any, Generator, Iterator, Type, get_args
from copy import copy
import itertools

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

    # Array of current states
    states = [particle.state_type() for _ in range(number_of_particles)]

    # Array of next states and when the transition is going to happen
    next_states = [copy(state) for state in states]
    transition_times = [particle.advance_state(state) for state in next_states]

    yield states

    # From t=0 to infinty
    for t in itertools.count(start=0):
        # Transition states if it is the time
        for (i, (state, next_state, transition_time)) in enumerate(zip(states, next_states, transition_times)):
            if t == transition_time:
                states[i], next_states[i] = next_state, state # Swap the two states to create less garbage
                transition_times[i] += particle.advance_state(next_state)

        yield states


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

    recent_states = []  

    for states in generator:
        recent_states.append(states)

        if convergence.has_converged(recent_states):
            break

        yield states