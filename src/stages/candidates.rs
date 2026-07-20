use crate::belief::record::BeliefKind;
use crate::core::action::{ActionCandidate, ActionKind};
use crate::core::math::Vec3;
use crate::registry::soa::SoARegistry;

#[allow(unused_assignments)]
pub fn run(registry: &mut SoARegistry) {
    let active_indices: Vec<usize> = registry.active.ones().collect();

    for idx in active_indices {
        if !registry.dirty_flag.contains(idx) {
            continue;
        }

        let self_id = crate::core::id::EntityId(idx as u32);
        let self_pos = registry.positions[idx];
        let mut c_idx = 0;
        let mut candidates = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ];

        macro_rules! add {
            ($kind:expr) => {
                if c_idx < 16 {
                    candidates[c_idx] = Some(ActionCandidate {
                        kind: $kind,
                        score: 0.0,
                    });
                    c_idx += 1;
                }
            };
        }

        add!(ActionKind::Idle);

        let mut closest_enemy_id = None;
        let mut closest_enemy_dist_sq = f32::MAX;
        let mut closest_enemy_pos = None;
        let mut has_threats = false;

        for record in registry.beliefs[idx].iter() {
            if record.confidence > 0.3 {
                if let BeliefKind::Position(pos) = record.kind {
                    let dist_sq = self_pos.distance_sq(pos);
                    if dist_sq < closest_enemy_dist_sq {
                        closest_enemy_dist_sq = dist_sq;
                        closest_enemy_id = Some(record.subject_id);
                        closest_enemy_pos = Some(pos);
                    }
                    has_threats = true;
                }
            }
        }

        if let (Some(e_id), Some(e_pos)) = (closest_enemy_id, closest_enemy_pos) {
            add!(ActionKind::Engage(e_id));
            add!(ActionKind::Advance(e_pos));

            let dir = (e_pos - self_pos).normalize();
            let perp = Vec3::new(-dir.y, dir.x, 0.0);
            add!(ActionKind::Flank(self_pos + perp * 10.0));

            if registry.health[idx] < 0.3 {
                add!(ActionKind::Retreat(self_pos - dir * 10.0));
                add!(ActionKind::Heal(self_id));
            }
            if registry.morale[idx] < 0.3 {
                add!(ActionKind::Flee(self_pos - dir * 20.0));
            }
        } else {
            if registry.health[idx] < 0.3 {
                add!(ActionKind::Heal(self_id));
            }
        }

        if registry.ammo[idx] < 0.2 {
            add!(ActionKind::Reload);
        }

        for record in registry.beliefs[idx].iter() {
            match record.kind {
                BeliefKind::CoverPosition(pos) => add!(ActionKind::TakeCover(pos)),
                BeliefKind::Sound(pos) => add!(ActionKind::Investigate(pos)),
                _ => {}
            }
        }

        if let Some(intent) = &registry.current_intent[idx] {
            add!(intent.goal.clone());
        }

        if !has_threats {
            let rx = registry.rng[idx].gen_f32() * 20.0 - 10.0;
            let ry = registry.rng[idx].gen_f32() * 20.0 - 10.0;
            add!(ActionKind::Patrol(self_pos + Vec3::new(rx, ry, 0.0)));
        }

        registry.candidates[idx] = candidates;
    }
}
