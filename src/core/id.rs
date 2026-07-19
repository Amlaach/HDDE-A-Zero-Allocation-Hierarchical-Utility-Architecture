use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[repr(transparent)]
pub struct EntityId(pub u32);

impl EntityId {
    pub const NONE: EntityId = EntityId(u32::MAX);
    
    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[repr(transparent)]
pub struct TeamId(pub u16);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum FactionRelation {
    Allied = 0,
    Neutral = 1,
    Hostile = 2,
}

impl Default for FactionRelation {
    fn default() -> Self {
        Self::Neutral
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum HierarchyLevel {
    Soldier = 0,
    SquadLeader = 1,
    PlatoonCommander = 2,
    CompanyCommander = 3,
    StrategicAI = 4,
}

impl Default for HierarchyLevel {
    fn default() -> Self {
        Self::Soldier
    }
}
