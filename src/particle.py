from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import TypeVar, override

State = TypeVar("State")


@dataclass
# Inherits from ABC (Abtract Base Class) to make sure that `on_rate` and `off_rate` are overriden.
class Particle(ABC):
    """A nano-particle (e.g.: a virus), with its parameters.

    This is a very generic class that should provide the interface to do our simulations. We should generally avoid using specific particle implementations in our simulations to make them very generic (and to avoid writing 20 versions of the same algorithm just slightly tweaked for each model). 
    """

    @abstractmethod
    def on_rate(self) -> float:
        """
        TODO: Write docstring
        """
        ...

    @abstractmethod
    def off_rate(self) -> float:
        """
        TODO: Write docstring
        """
        ...

    def advance_state(self) -> int:
        """
        Advances in-place the state of a particle, and returns the time elapsed to make that transition.
        """

        # TODO:
        raise Exception("TODO: do gillespie here")


class OneLigandParticle(Particle):
    is_attached: bool
    on: float
    off: float

    @override
    def on_rate(self):
        return 0 if self.is_attached else self.on

    @override
    def off_rate(self):
        return self.off if self.is_attached else 0


# TODO: Haven't actually tested this (but should work)
class MultiLigandParticle(Particle):
    attached_ligands: bool
    # given as ```
    # [
    #     (on 0->1, off 1->0),
    #     (on 1->2, off 2->1),
    #     ...
    # ]
    # ```
    rates: list[tuple[float, float]]
    off: float

    @override
    def on_rate(self):
        if self.attached_ligands >= len(self.rates):
            # Sanity check, could remove
            if self.attached_ligands > len(self.rates):
                raise RuntimeError(
                    "We should never have more attached ligands than on rates"
                )

            return 0

        return self.rates[self.attached_ligands][0]

    @override
    def off_rate(self):
        if self.attached_ligands <= 0:
            return 0

        return self.rates[self.attached_ligands - 1][1]
