use crate::core::time::Tick;
use crate::registry::soa::SoARegistry;

const DECAY_RATE: f32 = 0.05;
const THRESHOLD: f32 = 0.01;

pub fn run(registry: &mut SoARegistry, current_tick: Tick) {
    for idx in registry.active.ones() {
        let store = &mut registry.beliefs[idx];
        let mut modified = false;
        for record in store.iter_mut() {
            let age_ticks = current_tick - record.received_at;
            if age_ticks > 0 {
                let new_conf = if age_ticks == 1 {
                    record.confidence * (1.0 - DECAY_RATE)
                } else {
                    record.confidence * (1.0 - DECAY_RATE).powi(age_ticks as i32)
                };
                if (record.confidence - new_conf).abs() > 0.001 {
                    record.confidence = new_conf;
                    record.received_at = current_tick;
                    modified = true;
                }
            }
        }
        store.remove_stale(THRESHOLD);
        if modified {
            registry.dirty_flag.set(idx, true);
        }
    }
}
