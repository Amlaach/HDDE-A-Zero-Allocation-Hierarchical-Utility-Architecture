use crate::registry::soa::SoARegistry;
use crate::core::id::{EntityId, HierarchyLevel};
use crate::comm::channel::CommChannel;
use crate::core::action::Intent;
use crate::core::time::Tick;
use crate::comm::event::{CommEvent, Payload};
use crate::belief::record::BeliefKind;

pub fn run(registry: &SoARegistry, channel: &mut CommChannel, current_tick: Tick) {
    let active_indices: Vec<usize> = registry.active.ones().collect();

    for &idx in &active_indices {
        let level = &registry.hierarchy_level[idx];
        
        if *level == HierarchyLevel::Soldier {
            let mut high_threat = false;
            for record in registry.beliefs[idx].iter() {
                if let BeliefKind::ThreatLevel(val) = record.kind {
                    if val > 0.5 && record.confidence > 0.5 {
                        high_threat = true;
                        break;
                    }
                }
            }
            if high_threat {
                if let Some(parent) = registry.parent_ids[idx] {
                    let reports: Vec<_> = registry.beliefs[idx].iter().cloned().collect();
                    let event = CommEvent {
                        sender: EntityId(idx as u32),
                        receiver: parent,
                        payload: Payload::StatusReport(reports),
                        send_tick: current_tick,
                        delivery_tick: current_tick + 2,
                    };
                    channel.send(event);
                }
            }
        }
        
        if *level == HierarchyLevel::SquadLeader || *level == HierarchyLevel::PlatoonCommander {
            let intent = Intent {
                origin_rank: if *level == HierarchyLevel::PlatoonCommander { 2 } else { 1 },
                goal: registry.chosen_action[idx].clone(),
                confidence: 1.0,
            };
            
            for &child in &registry.children_ids[idx] {
                let delay = if *level == HierarchyLevel::PlatoonCommander { 5 } else { 2 };
                let event = CommEvent {
                    sender: EntityId(idx as u32),
                    receiver: child,
                    payload: Payload::IntentDirective(intent.clone()),
                    send_tick: current_tick,
                    delivery_tick: current_tick + delay,
                };
                channel.send(event);
            }
        }
    }
}
