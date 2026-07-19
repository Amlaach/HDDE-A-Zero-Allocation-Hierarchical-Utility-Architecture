use super::event::CommEvent;
use crate::core::id::EntityId;
use crate::core::time::Tick;

pub struct CommChannel {
    events: Vec<CommEvent>,
}

impl CommChannel {
    pub fn new() -> Self {
        Self {
            events: Vec::with_capacity(1024),
        }
    }

    pub fn send(&mut self, event: CommEvent) {
        self.events.push(event);
    }

    pub fn receive_for(&mut self, receiver: EntityId, current_tick: Tick) -> Vec<CommEvent> {
        let mut delivered = Vec::new();
        let mut i = 0;
        
        while i < self.events.len() {
            if self.events[i].receiver == receiver && self.events[i].delivery_tick <= current_tick {
                delivered.push(self.events.swap_remove(i));
            } else {
                i += 1;
            }
        }
        
        delivered
    }
}
