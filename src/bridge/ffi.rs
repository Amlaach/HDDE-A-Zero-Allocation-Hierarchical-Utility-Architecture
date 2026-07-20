#![allow(clippy::not_unsafe_ptr_arg_deref)]

use crate::belief::record::BeliefKind;
use crate::core::id::EntityId;
use crate::core::id::HierarchyLevel;
use crate::core::math::Vec3;
use crate::engine::HDDEngine;
use crate::stages::ingestion::RawPerceptionEvent;

#[repr(C)]
pub struct FFIEvent {
    pub receiver: u32,
    pub target: u32,
    pub source_entity: u32,
    pub kind_type: u8,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub confidence: f32,
}

#[repr(C)]
pub struct FFIAction {
    pub entity_id: u32,
    pub action_type: u8,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub target_id: u32,
}

#[no_mangle]
pub extern "C" fn hdde_engine_create() -> *mut HDDEngine {
    let engine = Box::new(HDDEngine::new());
    Box::into_raw(engine)
}

#[no_mangle]
pub extern "C" fn hdde_engine_destroy(engine_ptr: *mut HDDEngine) {
    if !engine_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(engine_ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn hdde_engine_spawn(
    engine_ptr: *mut HDDEngine,
    x: f32,
    y: f32,
    z: f32,
    hierarchy: u8,
) -> u32 {
    if engine_ptr.is_null() {
        return 0;
    }
    let engine = unsafe { &mut *engine_ptr };

    let level = match hierarchy {
        0 => HierarchyLevel::Soldier,
        1 => HierarchyLevel::SquadLeader,
        2 => HierarchyLevel::PlatoonCommander,
        _ => HierarchyLevel::Soldier,
    };

    let id = engine.spawn_entity(Vec3::new(x, y, z));
    engine.registry.hierarchy_level[id.index()] = level;
    id.0
}

#[no_mangle]
pub extern "C" fn hdde_engine_tick(
    engine_ptr: *mut HDDEngine,
    events_ptr: *const FFIEvent,
    events_count: usize,
) {
    if engine_ptr.is_null() {
        return;
    }
    let engine = unsafe { &mut *engine_ptr };

    let mut native_events = Vec::with_capacity(events_count);

    if !events_ptr.is_null() && events_count > 0 {
        let ffi_events = unsafe { std::slice::from_raw_parts(events_ptr, events_count) };
        for e in ffi_events {
            let kind = match e.kind_type {
                0 => BeliefKind::Position(Vec3::new(e.pos_x, e.pos_y, e.pos_z)),
                1 => BeliefKind::ThreatLevel(e.confidence),
                _ => BeliefKind::Position(Vec3::new(e.pos_x, e.pos_y, e.pos_z)),
            };

            native_events.push(RawPerceptionEvent {
                receiver: EntityId(e.receiver),
                target: EntityId(e.target),
                source_entity: EntityId(e.source_entity),
                kind,
                confidence: e.confidence,
            });
        }
    }

    engine.tick(&native_events);
}

#[no_mangle]
pub extern "C" fn hdde_engine_get_actions(
    engine_ptr: *const HDDEngine,
    out_actions_ptr: *mut FFIAction,
    max_actions: usize,
) -> usize {
    if engine_ptr.is_null() || out_actions_ptr.is_null() {
        return 0;
    }
    let engine = unsafe { &*engine_ptr };

    let active_indices: Vec<usize> = engine.registry.active.ones().collect();
    let count = std::cmp::min(active_indices.len(), max_actions);

    let out_actions = unsafe { std::slice::from_raw_parts_mut(out_actions_ptr, count) };

    for (i, &idx) in active_indices.iter().take(count).enumerate() {
        let action = &engine.registry.chosen_action[idx];
        let mut ffi = FFIAction {
            entity_id: idx as u32,
            action_type: 0,
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            target_id: 0,
        };

        match action {
            crate::core::action::ActionKind::Idle => ffi.action_type = 0,
            crate::core::action::ActionKind::Advance(p) => {
                ffi.action_type = 1;
                ffi.pos_x = p.x;
                ffi.pos_y = p.y;
                ffi.pos_z = p.z;
            }
            crate::core::action::ActionKind::Engage(id) => {
                ffi.action_type = 2;
                ffi.target_id = id.0;
            }
            crate::core::action::ActionKind::Flee(p) => {
                ffi.action_type = 3;
                ffi.pos_x = p.x;
                ffi.pos_y = p.y;
                ffi.pos_z = p.z;
            }
            _ => ffi.action_type = 0,
        }

        out_actions[i] = ffi;
    }

    count
}
