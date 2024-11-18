from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Callable, Iterable, Type

@dataclass
class Event[State]:
    rate: float

    # TODO: Maybe transition should not be in-place? I think it's worth it to make it in-place to create less garbage, since this is Python after all.
    transition: Callable[[State], None]


# Inherits from ABC (Abtract Base Class) to make sure that `on_rate` and `off_rate` are overriden.
class Particle[State](ABC):
    """A nano-particle (e.g.: a virus), with its parameters and its state.

    This is an abstract class that needs to be inherited from.

    This is a very generic class that should provide the interface to do our simulations. We should generally avoid using specific particle implementations in our simulations to make them very generic (and to avoid writing 20 versions of the same algorithm just slightly tweaked for each model).
    """

    state_type: Type[State]

    @abstractmethod
    def events(self, state: State) -> Iterable[Event[State]]: ...

    def advance_state(self, state: State) -> int:
        """
        Advances in-place the state of a particle, and returns the time elapsed to make that transition.
        """

        events = self.events(state)

        # TODO:
        raise Exception("TODO: do gillespie here")


class ParticleInstance[State]:
    """
    The actual instance of the particle. This is a concrete class.
    """

    state: State
    specification: Particle[State]

    def __init__(self, state_type: Type[State], specification: Particle[State]):
        self.state = state_type()
        self.specification = specification

    def advance_state(self) -> int:
        return self.specification.advance_state(self.state)
