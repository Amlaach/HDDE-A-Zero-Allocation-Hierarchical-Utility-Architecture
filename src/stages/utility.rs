use crate::registry::soa::SoARegistry;
use crate::core::action::{ActionKind, ActionCandidate};

pub trait Consideration {
    fn evaluate(&self, entity_idx: usize, candidate: &ActionCandidate, registry: &SoARegistry) -> f32;
}

struct ThreatProximityConsideration { weight: f32 }
impl Consideration for ThreatProximityConsideration {
    fn evaluate(&self, _idx: usize, candidate: &ActionCandidate, _registry: &SoARegistry) -> f32 {
        match candidate.kind {
            ActionKind::Engage(_) | ActionKind::Advance(_) => self.weight * 1.0,
            _ => self.weight * 0.1,
        }
    }
}

struct SelfPreservationConsideration { weight: f32 }
impl Consideration for SelfPreservationConsideration {
    fn evaluate(&self, idx: usize, candidate: &ActionCandidate, registry: &SoARegistry) -> f32 {
        let health = registry.health[idx];
        let pres = registry.self_preservation[idx];
        match candidate.kind {
            ActionKind::Retreat(_) | ActionKind::TakeCover(_) | ActionKind::Flee(_) | ActionKind::Heal(_) => self.weight * (1.0 - health) * pres,
            _ => self.weight * health,
        }
    }
}

struct AmmoConsideration { weight: f32 }
impl Consideration for AmmoConsideration {
    fn evaluate(&self, idx: usize, candidate: &ActionCandidate, registry: &SoARegistry) -> f32 {
        let ammo = registry.ammo[idx];
        match candidate.kind {
            ActionKind::Reload => self.weight * (1.0 - ammo),
            ActionKind::Engage(_) => if ammo < 0.1 { self.weight * 0.1 } else { self.weight * 1.0 },
            _ => self.weight * 1.0,
        }
    }
}

struct MoraleConsideration { weight: f32 }
impl Consideration for MoraleConsideration {
    fn evaluate(&self, idx: usize, candidate: &ActionCandidate, registry: &SoARegistry) -> f32 {
        let morale = registry.morale[idx];
        match candidate.kind {
            ActionKind::Flee(_) => self.weight * (1.0 - morale),
            ActionKind::Advance(_) => self.weight * morale,
            _ => self.weight * 0.5,
        }
    }
}

struct IntentComplianceConsideration { weight: f32 }
impl Consideration for IntentComplianceConsideration {
    fn evaluate(&self, idx: usize, candidate: &ActionCandidate, registry: &SoARegistry) -> f32 {
        if let Some(intent) = &registry.current_intent[idx] {
            if candidate.kind == intent.goal {
                return self.weight * 1.0;
            }
        }
        self.weight * 0.5
    }
}

struct InertiaConsideration { weight: f32 }
impl Consideration for InertiaConsideration {
    fn evaluate(&self, idx: usize, candidate: &ActionCandidate, registry: &SoARegistry) -> f32 {
        if candidate.kind == registry.chosen_action[idx] {
            self.weight * 1.0
        } else {
            self.weight * 0.5
        }
    }
}

struct NeedsConsideration { weight: f32 }
impl Consideration for NeedsConsideration {
    fn evaluate(&self, idx: usize, candidate: &ActionCandidate, registry: &SoARegistry) -> f32 {
        let cur = registry.curiosity[idx];
        match candidate.kind {
            ActionKind::Patrol(_) | ActionKind::Investigate(_) => self.weight * cur,
            _ => self.weight * 0.5,
        }
    }
}

pub fn run(registry: &mut SoARegistry) {
    let tp = ThreatProximityConsideration { weight: 1.0 };
    let sp = SelfPreservationConsideration { weight: 1.0 };
    let ac = AmmoConsideration { weight: 1.0 };
    let mc = MoraleConsideration { weight: 1.0 };
    let ic = IntentComplianceConsideration { weight: 1.0 };
    let in_c = InertiaConsideration { weight: 1.0 };
    let nc = NeedsConsideration { weight: 1.0 };
    
    let considerations: [&dyn Consideration; 7] = [&tp, &sp, &ac, &mc, &ic, &in_c, &nc];
    let active_indices: Vec<usize> = registry.active.ones().collect();

    for idx in active_indices {
        for i in 0..16 {
            let score = if let Some(candidate) = &registry.candidates[idx][i] {
                let mut total_score = 1.0;
                for cons in considerations.iter() {
                    total_score *= cons.evaluate(idx, candidate, registry);
                }
                Some(total_score)
            } else {
                None
            };
            if let Some(s) = score {
                if let Some(candidate) = &mut registry.candidates[idx][i] {
                    candidate.score = s;
                }
            }
        }
    }
}
