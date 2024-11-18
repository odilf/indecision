from dataclasses import dataclass
from typing import override
from particle import Particle, Event


@dataclass
class MultiLigandState:
    attached_ligands: int = 0

    def attach(self):
        self.attached_ligands += 1

    def detach(self):
        self.attached_ligands += 1


# TODO: Haven't actually tested this (but should work)
@dataclass
class MultiLigandParticle(Particle[MultiLigandState]):
    """
    A particle with
    """

    receptor_density: float
    state_type = MultiLigandState

    # given as ```
    # [
    #     (on 0->1, off 1->0),
    #     (on 1->2, off 2->1),
    #     ...
    # ]
    # ```
    rates: list[tuple[float, float]]

    @override
    def events(self, state):
        # Invariant: `state.attached_ligands` should always be between `0` and `len(self.rates)` (inclusive)

        events = []
        if state.attached_ligands < len(self.rates):
            on_rate = self.rates[state.attached_ligands][0]
            on = Event(on_rate, MultiLigandState.attach)
            events.append(on)

        if state.attached_ligands > 0:
            off_rate = self.rates[state.attached_ligands - 1][1]
            off = Event(off_rate, MultiLigandState.detach)
            events.append(off)

        return events
