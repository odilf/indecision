from typing import Callable, Sequence
import numpy as np
from scipy.stats import wasserstein_distance

class ConvergenceCriterion[State]:
    def __init__(
        self,
        metric: Callable[[State, State], float],
        tolerance: float,
        window_size: int,
        sample_size: int,
    ):
        """ Initialise a convergence criterion based on empirical distribution convergence.

            Args:
            metric (Callable): A distance metric function `d(x, y)`.
            tolerance (float): Maximum allowable Wasserstein distance for convergence.
            window_size (int): Number of consecutive time steps to consider.
            sample_size (int): Number of samples to approximate the distribution.
        """
        self.metric = metric
        self.tolerance = tolerance
        self.window_size = window_size
        self.sample_size = sample_size
        self.history = []

    def _compute_empirical_distribution(self, states: list[list[State]]) -> np.ndarray:
        """ Computes an empirical distribtion for the given window of states.

            Args:
            states (list[list[State]]): A list of states over time.

            Returns:
            np.ndarray: Flattened represention of the empirical distribution.
        """
        flattened_states = [
            np.ravel(state) for snapshot in states for state in snapshot
        ]
        return np.array(flattened_states[: self.sample_size])

    def has_converged(self, states: Sequence[list[State]]) -> bool:
        """ Determines whether the system has converged by evaluating Wasserstein distances.

            Args:
            states (Sequence[list[tate]]): A sequence of recent states.

            Returns:
            bool: True if the system has converged, False otherwise.
        """
        self.history.append(states)
        if len(self.history) < self.window_size:
            return False

        recent = self.history[-self.window_size :]
        older = self.history[-2 * self.window_size : -self.window_size]

        if len(older) < self.window_size:
            return False  

        recent_distribution = self._compute_empirical_distribution(recent)
        older_distribution = self._compute_empirical_distribution(older)

        distance = wasserstein_distance(recent_distribution, older_distribution)
        return distance < self.tolerance
