use crate::core::action::ActionKind;
use crate::registry::soa::SoARegistry;

pub fn run(registry: &mut SoARegistry) {
    let active_indices: Vec<usize> = registry.active.ones().collect();

    for idx in active_indices {
        registry.hunger[idx] = (registry.hunger[idx] + 0.001).clamp(0.0, 1.0);
        registry.fatigue[idx] = (registry.fatigue[idx] + 0.001).clamp(0.0, 1.0);

        match registry.chosen_action[idx] {
            ActionKind::Investigate(_) => {
                registry.curiosity[idx] = (registry.curiosity[idx] - 0.005).clamp(0.0, 1.0);
            }
            ActionKind::Idle => {
                registry.curiosity[idx] = (registry.curiosity[idx] + 0.002).clamp(0.0, 1.0);
            }
            _ => {}
        }

        let health = registry.health[idx];
        if health < 0.5 {
            registry.self_preservation[idx] =
                (registry.self_preservation[idx] + 0.01).clamp(0.0, 1.0);
        } else if health > 0.9 {
            registry.self_preservation[idx] =
                (registry.self_preservation[idx] - 0.005).clamp(0.0, 1.0);
        }
    }
}
