use crate::core::action::ActionKind;
use crate::core::time::Tick;

#[derive(Clone, Debug, PartialEq)]
pub struct DecisionTrace {
    pub tick: Tick,
    pub chosen_action: ActionKind,
    pub top_scores: [(ActionKind, f32); 3],
}

impl Default for DecisionTrace {
    fn default() -> Self {
        Self {
            tick: Tick(0),
            chosen_action: ActionKind::Idle,
            top_scores: [
                (ActionKind::Idle, 0.0),
                (ActionKind::Idle, 0.0),
                (ActionKind::Idle, 0.0),
            ],
        }
    }
}
