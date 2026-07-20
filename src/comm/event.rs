use crate::belief::record::BeliefRecord;
use crate::core::action::Intent;
use crate::core::id::EntityId;
use crate::core::time::Tick;

#[derive(Clone, Debug)]
pub enum Payload {
    StatusReport(Vec<BeliefRecord>),
    IntentDirective(Intent),
}

#[derive(Clone, Debug)]
pub struct CommEvent {
    pub sender: EntityId,
    pub receiver: EntityId,
    pub payload: Payload,
    pub send_tick: Tick,
    pub delivery_tick: Tick,
}
