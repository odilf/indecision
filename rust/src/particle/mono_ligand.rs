use super::Event;

#[derive(Clone, Copy, Debug, Default)]
pub struct MonoLigandState {
    is_attached: bool,
}

impl MonoLigandState {
    pub fn toggle(&self) -> Self {
        Self {
            is_attached: !self.is_attached,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[pyo3::pyclass]
pub struct MonoLigand {
    pub receptor_density: f64,
    pub binding_strength: f64,

    pub on_rate: f64,
    pub off_rate: f64,
}

impl super::Particle for MonoLigand {
    type State = MonoLigandState;

    fn events(&self, state: &Self::State) -> Vec<Event<Self::State>> {
        if state.is_attached {
            return vec![Event {
                rate: self.off_rate * self.binding_strength,
                transition: |state| state.toggle(),
            }];
        } else {
            return vec![Event {
                rate: self.on_rate * self.receptor_density * self.binding_strength,
                transition: |state| state.toggle(),
            }];
        }
    }
}
