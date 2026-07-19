use crate::core::id::EntityId;
use crate::core::time::Tick;
use crate::core::action::Intent;
use crate::belief::record::BeliefRecord;

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
