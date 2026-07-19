use crate::core::id::EntityId;
use crate::core::math::Vec3;
use crate::belief::store::BeliefStore;
use crate::core::action::{ActionCandidate, ActionKind, Intent};
use crate::core::rng::DeterministicRng;
use fixedbitset::FixedBitSet;

pub const MAX_ENTITIES: usize = 16384;
pub const MAX_CANDIDATES: usize = 16;

#[derive(Clone, Debug, PartialEq)]
pub enum HierarchyLevel {
    Soldier,
    SquadLeader,
    Commander,
}

pub struct SoARegistry {
    pub positions: Vec<Vec3>,
    pub beliefs: Vec<BeliefStore>,
    pub candidates: Vec<[Option<ActionCandidate>; MAX_CANDIDATES]>,
    pub chosen_action: Vec<ActionKind>,
    pub health: Vec<f32>,
    pub ammo: Vec<f32>,
    pub morale: Vec<f32>,
    pub hunger: Vec<f32>,
    pub fatigue: Vec<f32>,
    pub curiosity: Vec<f32>,
    pub self_preservation: Vec<f32>,
    pub current_intent: Vec<Option<Intent>>,
    pub hierarchy_level: Vec<HierarchyLevel>,
    pub parent_ids: Vec<Option<EntityId>>,
    pub children_ids: Vec<Vec<EntityId>>,
    pub rng: Vec<DeterministicRng>,
    pub active: FixedBitSet,
    pub dirty_flag: FixedBitSet,
    pub decision_traces: Vec<Vec<(ActionKind, f32)>>,
}

impl SoARegistry {
    pub fn new() -> Self {
        const INIT_CANDIDATE: Option<ActionCandidate> = None;
        Self {
            positions: vec![Vec3::zero(); MAX_ENTITIES],
            beliefs: vec![BeliefStore::new(); MAX_ENTITIES],
            candidates: vec![[INIT_CANDIDATE; MAX_CANDIDATES]; MAX_ENTITIES],
            chosen_action: vec![ActionKind::Idle; MAX_ENTITIES],
            health: vec![1.0; MAX_ENTITIES],
            ammo: vec![1.0; MAX_ENTITIES],
            morale: vec![1.0; MAX_ENTITIES],
            hunger: vec![0.0; MAX_ENTITIES],
            fatigue: vec![0.0; MAX_ENTITIES],
            curiosity: vec![0.5; MAX_ENTITIES],
            self_preservation: vec![0.5; MAX_ENTITIES],
            current_intent: vec![None; MAX_ENTITIES],
            hierarchy_level: vec![HierarchyLevel::Soldier; MAX_ENTITIES],
            parent_ids: vec![None; MAX_ENTITIES],
            children_ids: vec![Vec::new(); MAX_ENTITIES],
            rng: (0..MAX_ENTITIES).map(|i| DeterministicRng::new(i as u64)).collect(),
            active: FixedBitSet::with_capacity(MAX_ENTITIES),
            dirty_flag: FixedBitSet::with_capacity(MAX_ENTITIES),
            decision_traces: vec![Vec::new(); MAX_ENTITIES],
        }
    }
    
    pub fn spawn(&mut self, pos: Vec3) -> EntityId {
        for i in 0..MAX_ENTITIES {
            if !self.active.contains(i) {
                self.active.insert(i);
                self.dirty_flag.insert(i);
                self.positions[i] = pos;
                self.beliefs[i] = BeliefStore::new();
                self.chosen_action[i] = ActionKind::Idle;
                self.candidates[i] = core::array::from_fn(|_| None);
                self.health[i] = 1.0;
                self.ammo[i] = 1.0;
                self.morale[i] = 1.0;
                self.hunger[i] = 0.0;
                self.fatigue[i] = 0.0;
                self.curiosity[i] = 0.5;
                self.self_preservation[i] = 0.5;
                self.current_intent[i] = None;
                self.hierarchy_level[i] = HierarchyLevel::Soldier;
                self.parent_ids[i] = None;
                self.children_ids[i].clear();
                self.decision_traces[i].clear();
                return EntityId(i as u32);
            }
        }
        EntityId::NONE
    }

    pub fn spawn_with_role(&mut self, pos: Vec3, level: HierarchyLevel) -> EntityId {
        let id = self.spawn(pos);
        if id != EntityId::NONE {
            self.hierarchy_level[id.index()] = level;
        }
        id
    }

    pub fn despawn(&mut self, id: EntityId) {
        let idx = id.index();
        if idx < MAX_ENTITIES && self.active.contains(idx) {
            self.active.set(idx, false);
        }
    }

    pub fn set_parent(&mut self, child: EntityId, parent: EntityId) {
        let child_idx = child.index();
        let parent_idx = parent.index();
        
        if child_idx < MAX_ENTITIES {
            self.parent_ids[child_idx] = Some(parent);
        }
        
        if parent_idx < MAX_ENTITIES {
            if !self.children_ids[parent_idx].contains(&child) {
                self.children_ids[parent_idx].push(child);
            }
        }
    }

    #[inline]
    pub fn entity_count(&self) -> usize {
        self.active.count_ones(..)
    }
}
