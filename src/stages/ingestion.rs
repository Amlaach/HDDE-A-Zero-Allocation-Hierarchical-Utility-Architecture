use crate::registry::soa::SoARegistry;
use crate::core::time::Tick;
use crate::belief::record::{BeliefRecord, BeliefKind};
use crate::core::id::EntityId;
use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct RawPerceptionEvent {
    pub receiver: EntityId,
    pub target: EntityId,
    pub source_entity: EntityId,
    pub kind: BeliefKind,
    pub confidence: f32,
}

pub fn run(registry: &mut SoARegistry, events: &[RawPerceptionEvent], current_tick: Tick) {
    for event in events {
        if event.receiver.index() < crate::registry::soa::MAX_ENTITIES {
            let mut hasher = DefaultHasher::new();
            event.source_entity.hash(&mut hasher);
            let source_chain_hash = hasher.finish();

            let record = BeliefRecord {
                subject_id: event.target,
                kind: event.kind.clone(),
                observed_at: current_tick,
                received_at: current_tick,
                confidence: event.confidence,
                source_chain_hash,
            };
            registry.beliefs[event.receiver.index()].insert_or_update(record);
            registry.dirty_flag.set(event.receiver.index(), true);
        }
    }
}
