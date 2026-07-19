#[derive(Clone, Debug, PartialEq)]
pub struct NeedState {
    pub hunger: f32,
    pub fatigue: f32,
    pub curiosity: f32,
    pub self_preservation: f32,
}

impl Default for NeedState {
    fn default() -> Self {
        Self {
            hunger: 0.5,
            fatigue: 0.5,
            curiosity: 0.5,
            self_preservation: 0.5,
        }
    }
}
