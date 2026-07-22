use crate::core::action::ActionKind;
use crate::registry::soa::SoARegistry;

pub fn run(registry: &mut SoARegistry) {
    for idx in registry.active.ones() {
        let mut best_candidate: Option<(&ActionKind, f32)> = None;

        for candidate in registry.candidates[idx].iter().flatten() {
            match best_candidate {
                Some((_, best_score)) if candidate.score <= best_score => {}
                _ => {
                    best_candidate = Some((&candidate.kind, candidate.score));
                }
            }
        }

        if let Some((kind, _)) = best_candidate {
            registry.chosen_action[idx] = kind.clone();
        } else {
            registry.chosen_action[idx] = ActionKind::Idle;
        }

        #[cfg(feature = "debug-trace")]
        {
            let mut sorted_candidates = Vec::new();
            for candidate in registry.candidates[idx].iter().flatten() {
                sorted_candidates.push((candidate.kind.clone(), candidate.score));
            }
            sorted_candidates
                .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(core::cmp::Ordering::Equal));
            registry.decision_traces[idx] = sorted_candidates.into_iter().take(3).collect();
        }

        registry.dirty_flag.set(idx, false);
    }
}
