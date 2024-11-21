from typing import Any, Iterator, Callable
from copy import copy
import itertools

from indecision.particle import Particle
from .convergence import ConvergenceCriterion


def simulate[State, Output](
    particle: Particle[State],
    extractor: Callable[[list[State]], Output],
    number_of_particles: int,
) -> Iterator[Output]:
    """
    Simulates a system of a particle.

    Returns a generator that goes on forever. Either use [`simulate_until_convergence`], [`simulate_until`] for a specific number of steps, or cap the number of steps from the outside (e.g., `[next(generator) for _ in range(n)]`),
    """

    # Array of current states
    states = [particle.state_type() for _ in range(number_of_particles)]

    # Array of next states and when the transition is going to happen
    next_states = [copy(state) for state in states]
    transition_times = [particle.advance_state(state) for state in next_states]

    yield extractor(states)

    # From t=0 to infinty
    for t in itertools.count(start=0):
        # Transition states if it is the time
        for (i, transition_time) in enumerate(transition_times):
            if t >= transition_time:
                states[i], next_states[i] = next_states[i], states[i] # Swap the two states to create less garbage
                transition_times[i] += particle.advance_state(next_states[i])

        yield extractor(states)


def simulate_until[State, Output](
    particle: Particle[State],
    extractor: Callable[[list[State]], Output],
    steps: int,
    *args,
    **kwargs: dict[str, Any],
) -> Iterator[Output]:
    t = 0
    generator = simulate(particle, extractor, *args, **kwargs)

    while t < steps:
        yield next(generator)


def simulate_until_converge[State, Output](
    particle: Particle[State],
    extractor: Callable[[list[State]], Output],
    convergence: ConvergenceCriterion[Output],
    *args,
    **kwargs,
) -> Iterator[Output]:
    generator = simulate(particle, extractor, *args, **kwargs)

    recent_states = []

    for states in generator:
        recent_states.append(states)

        if convergence.has_converged(recent_states):
            break

        yield states
