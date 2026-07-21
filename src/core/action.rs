use crate::core::id::EntityId;
use crate::core::math::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActionKind {
    Idle,
    Advance(Vec3),
    Hold(Vec3),
    Retreat(Vec3),
    Engage(EntityId),
    RequestSupport,
    Flank(Vec3),
    TakeCover(Vec3),
    Patrol(Vec3),
    Investigate(Vec3),
    Heal(EntityId),
    Reload,
    Flee(Vec3),
    Custom(u16),
    CustomPos(u16, Vec3),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Intent {
    pub origin_rank: u8,
    pub goal: ActionKind,
    pub confidence: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionCandidate {
    pub kind: ActionKind,
    pub score: f32,
}

impl Default for ActionCandidate {
    fn default() -> Self {
        Self {
            kind: ActionKind::Idle,
            score: 0.0,
        }
    }
}
