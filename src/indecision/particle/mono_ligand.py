from dataclasses import dataclass
from typing import override
from .core import Particle, Event


@dataclass
class MonoLigandState:
    is_attached: bool = False

    def toggle(self):
        self.is_attached = not self.is_attached


# This implementation serves mostly as a simple example, but is redundant given the MultiLigandParticle.
@dataclass
class MonoLigandParticle(Particle[MonoLigandState]):
    """
    A particle with one ligand.

    Equivalent to a [`MultiLigandParticle`] with one ligand.
    """

    state_type = MonoLigandState

    receptor_density: float
    binding_strength: float

    on_rate: float
    off_rate: float

    @override
    def events(self, state):
        if state.is_attached:
            yield Event(
                self.off_rate * self.binding_strength,
                MonoLigandState.toggle,
                repr="Dettach",
            )
        else:
            yield Event(
                self.on_rate * self.receptor_density * self.binding_strength,
                MonoLigandState.toggle,
                repr="Attach",
            )
