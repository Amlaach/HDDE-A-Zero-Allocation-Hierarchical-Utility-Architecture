use crate::core::id::EntityId;
use crate::core::math::Vec3;
use crate::core::time::Tick;

#[derive(Clone, Debug, PartialEq)]
pub enum BeliefKind {
    Position(Vec3),
    Strength(f32),
    ThreatLevel(f32),
    EnemyCount(u16),
    AllyCount(u16),
    CoverPosition(Vec3),
    DangerZone(Vec3),
    Sound(Vec3),
    LastKnownDirection(Vec3),
    Health(f32),
    AmmoLevel(f32),
}

#[derive(Clone, Debug)]
pub struct BeliefRecord {
    pub subject_id: EntityId,
    pub kind: BeliefKind,
    pub observed_at: Tick,
    pub received_at: Tick,
    pub confidence: f32,
    pub source_chain_hash: u64,
}
