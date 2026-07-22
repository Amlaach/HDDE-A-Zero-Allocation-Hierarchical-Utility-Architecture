use crate::core::action::{ActionCandidate, ActionKind};
use crate::registry::soa::SoARegistry;

pub trait Consideration {
    fn evaluate(
        &self,
        entity_idx: usize,
        candidate: &ActionCandidate,
        registry: &SoARegistry,
    ) -> f32;
}

struct ThreatProximityConsideration {
    weight: f32,
}
impl Consideration for ThreatProximityConsideration {
    fn evaluate(&self, _idx: usize, candidate: &ActionCandidate, _registry: &SoARegistry) -> f32 {
        match candidate.kind {
            ActionKind::Engage(_) | ActionKind::Advance(_) => self.weight * 1.0,
            _ => self.weight * 0.1,
        }
    }
}

struct SelfPreservationConsideration {
    weight: f32,
}
impl Consideration for SelfPreservationConsideration {
    fn evaluate(&self, idx: usize, candidate: &ActionCandidate, registry: &SoARegistry) -> f32 {
        let health = registry.health[idx];
        let pres = f32::max(0.1, registry.self_preservation[idx]); // Prevent 0.0 death trap
        match candidate.kind {
            ActionKind::Retreat(_)
            | ActionKind::TakeCover(_)
            | ActionKind::Flee(_)
            | ActionKind::Heal(_) => self.weight * f32::max(0.0, 1.0 - health) * pres,
            _ => self.weight * health,
        }
    }
}

struct AmmoConsideration {
    weight: f32,
}
impl Consideration for AmmoConsideration {
    fn evaluate(&self, idx: usize, candidate: &ActionCandidate, registry: &SoARegistry) -> f32 {
        let ammo = registry.ammo[idx];
        match candidate.kind {
            ActionKind::Reload => self.weight * f32::max(0.0, 1.0 - ammo),
            ActionKind::Engage(_) => {
                if ammo < 0.1 {
                    self.weight * 0.1
                } else {
                    self.weight * 1.0
                }
            }
            _ => self.weight * 1.0,
        }
    }
}

struct MoraleConsideration {
    weight: f32,
}
impl Consideration for MoraleConsideration {
    fn evaluate(&self, idx: usize, candidate: &ActionCandidate, registry: &SoARegistry) -> f32 {
        let morale = registry.morale[idx];
        match candidate.kind {
            ActionKind::Flee(_) => self.weight * f32::max(0.0, 1.0 - morale),
            ActionKind::Advance(_) => self.weight * morale,
            _ => self.weight * 0.5,
        }
    }
}

struct IntentComplianceConsideration {
    weight: f32,
}
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

struct InertiaConsideration {
    weight: f32,
}
impl Consideration for InertiaConsideration {
    fn evaluate(&self, idx: usize, candidate: &ActionCandidate, registry: &SoARegistry) -> f32 {
        if candidate.kind == registry.chosen_action[idx] {
            self.weight * 1.0
        } else {
            self.weight * 0.8 // 20% penalty, allows Needs to overcome Inertia
        }
    }
}

struct NeedsConsideration {
    weight: f32,
}
impl Consideration for NeedsConsideration {
    fn evaluate(&self, idx: usize, candidate: &ActionCandidate, registry: &SoARegistry) -> f32 {
        let cur = registry.curiosity[idx];
        match candidate.kind {
            ActionKind::Patrol(_) | ActionKind::Investigate(_) => self.weight * cur,
            _ => self.weight * 0.5,
        }
    }
}

use crate::engine::EngineHooks;

pub fn run(registry: &mut SoARegistry, hooks: &impl EngineHooks) {
    let tp = ThreatProximityConsideration { weight: 1.0 };
    let sp = SelfPreservationConsideration { weight: 1.0 };
    let ac = AmmoConsideration { weight: 1.0 };
    let mc = MoraleConsideration { weight: 1.0 };
    let ic = IntentComplianceConsideration { weight: 1.0 };
    let in_c = InertiaConsideration { weight: 1.0 };
    let nc = NeedsConsideration { weight: 1.0 };
    let considerations: [&dyn Consideration; 7] = [&tp, &sp, &ac, &mc, &ic, &in_c, &nc];

    for idx in registry.active.ones() {
        for i in 0..16 {
            let score = if let Some(candidate) = &registry.candidates[idx][i] {
                if let Some(custom_score) = hooks.evaluate_utility(idx, candidate, registry) {
                    Some(custom_score)
                } else {
                    let mut total_score = 1.0;
                    for cons in considerations.iter() {
                        total_score *= cons.evaluate(idx, candidate, registry);
                    }
                    Some(total_score)
                }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::math::Vec3;

    #[test]
    fn test_threat_proximity_consideration() {
        let reg = SoARegistry::new();
        let cons = ThreatProximityConsideration { weight: 1.0 };

        let c_engage = ActionCandidate {
            kind: ActionKind::Engage(crate::core::id::EntityId(1)),
            score: 0.0,
        };
        let c_idle = ActionCandidate {
            kind: ActionKind::Idle,
            score: 0.0,
        };

        assert!(cons.evaluate(0, &c_engage, &reg) > cons.evaluate(0, &c_idle, &reg));
    }

    #[test]
    fn test_self_preservation_consideration() {
        let mut reg = SoARegistry::new();
        reg.health[0] = 0.1;
        reg.self_preservation[0] = 1.0;
        let cons = SelfPreservationConsideration { weight: 1.0 };

        let c_flee = ActionCandidate {
            kind: ActionKind::Flee(Vec3::zero()),
            score: 0.0,
        };
        let c_idle = ActionCandidate {
            kind: ActionKind::Idle,
            score: 0.0,
        };

        assert!(cons.evaluate(0, &c_flee, &reg) > cons.evaluate(0, &c_idle, &reg));
    }
}
