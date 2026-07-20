use crate::comm::channel::CommChannel;
use crate::comm::event::Payload;
use crate::core::time::Tick;
use crate::registry::soa::SoARegistry;

pub fn run(registry: &mut SoARegistry, channel: &mut CommChannel, current_tick: Tick) {
    let active_indices: Vec<usize> = registry.active.ones().collect();

    for idx in active_indices {
        let receiver_id = crate::core::id::EntityId(idx as u32);
        let delivered = channel.receive_for(receiver_id, current_tick);

        let mut modified = false;

        for event in delivered {
            match event.payload {
                Payload::StatusReport(beliefs) => {
                    for mut b in beliefs {
                        b.confidence *= 0.8;
                        b.received_at = current_tick;
                        registry.beliefs[idx].insert_or_update(b);
                        modified = true;
                    }
                }
                Payload::IntentDirective(intent) => {
                    registry.current_intent[idx] = Some(intent);
                    modified = true;
                }
            }
        }

        if modified {
            registry.dirty_flag.insert(idx);
        }
    }
}
