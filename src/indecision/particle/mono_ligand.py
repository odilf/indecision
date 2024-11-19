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

    receptor_density: float
    state_type = MonoLigandState

    on_rate: float
    off_rate: float

    @override
    def events(self, state):
        if state.is_attached:
            yield Event(self.on_rate, transition=MonoLigandState.toggle)
        else:
            yield Event(self.off_rate, transition=MonoLigandState.toggle)
