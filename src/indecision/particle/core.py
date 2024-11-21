from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Callable, Iterable, Type
import numpy as np


@dataclass
class Event[State]:
    rate: float
    transition: Callable[[State], None]

    repr: str | None = None

    def __repr__(self):
        return self.repr or repr(self.__class__)
        # return self.__repr__() if self.string_repr is None else self.string_repr


# Inherits from ABC (Abtract Base Class) to make sure that `on_rate` and `off_rate` are overriden.
class Particle[State](ABC):
    """A nano-particle (e.g.: a virus), with its parameters and its state.

    This is an abstract class that needs to be inherited from.

    This is a very generic class that should provide the interface to do our simulations. We should generally avoid using specific particle implementations in our simulations to make them very generic (and to avoid writing 20 versions of the same algorithm just slightly tweaked for each model).
    """

    state_type: Type[State]

    @abstractmethod
    def events(self, state: State) -> Iterable[Event[State]]: ...

    def advance_state(self, state: State) -> float:
        """
        Advances in-place the state of a particle, and returns the time elapsed to make that transition.
        """
        events = list(self.events(state))
        if not events:
            raise RuntimeError("No events to process!")
        
        total_rate = sum(event.rate for event in events)
        if total_rate == 0:
            raise RuntimeError("Total rate is zero; no transitions are possible!")

        delta_t = -np.log(np.random.random()) / total_rate

        r = np.random.uniform(0, total_rate)
        cumulative_rate = 0.0
        for event in events:
            cumulative_rate += event.rate
            if cumulative_rate > r:
                event.transition(state)  
                break

        return delta_t
