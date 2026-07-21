use crate::core::action::ActionKind;
use crate::registry::soa::SoARegistry;

pub fn run(registry: &mut SoARegistry) {
    let active_indices: Vec<usize> = registry.active.ones().collect();

    for idx in active_indices {
        let mut sorted_candidates = Vec::new();
        for i in 0..16 {
            if let Some(candidate) = &registry.candidates[idx][i] {
                sorted_candidates.push((candidate.kind.clone(), candidate.score));
            }
        }

        sorted_candidates
            .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(core::cmp::Ordering::Equal));

        if let Some(best) = sorted_candidates.first() {
            registry.chosen_action[idx] = best.0.clone();
        } else {
            registry.chosen_action[idx] = ActionKind::Idle;
        }

        #[cfg(feature = "debug-trace")]
        {
            registry.decision_traces[idx] = sorted_candidates.into_iter().take(3).collect();
        }
        registry.dirty_flag.set(idx, false);
    }
}
