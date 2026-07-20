use crate::registry::soa::SoARegistry;
use crate::comm::channel::CommChannel;
use crate::core::time::Tick;
use crate::stages::ingestion::RawPerceptionEvent;

pub struct HDDEngine {
    pub registry: SoARegistry,
    pub comm_channel: CommChannel,
    pub current_tick: Tick,
}

impl HDDEngine {
    pub fn new() -> Self {
        Self {
            registry: SoARegistry::new(),
            comm_channel: CommChannel::new(),
            current_tick: Tick(0),
        }
    }

    pub fn spawn_entity(&mut self, pos: crate::core::math::Vec3) -> crate::core::id::EntityId {
        self.registry.spawn(pos)
    }

    pub fn tick(&mut self, incoming_events: &[RawPerceptionEvent]) {
        self.current_tick = self.current_tick + 1;

        crate::stages::ingestion::run(&mut self.registry, incoming_events, self.current_tick);
        crate::stages::decay::run(&mut self.registry, self.current_tick);
        crate::stages::needs::run(&mut self.registry);
        crate::stages::candidates::run(&mut self.registry);
        crate::stages::utility::run(&mut self.registry);
        crate::stages::commit::run(&mut self.registry);
        crate::stages::propagation::run(&self.registry, &mut self.comm_channel, self.current_tick);
        crate::stages::comms::run(&mut self.registry, &mut self.comm_channel, self.current_tick);
    }
}

impl Default for HDDEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::math::Vec3;
    use crate::belief::record::BeliefKind;
    use crate::core::id::EntityId;

    #[test]
    fn test_engine_tick() {
        let mut engine = HDDEngine::new();
        let e1 = engine.spawn_entity(Vec3::zero());
        
        assert_eq!(engine.current_tick.0, 0);
        engine.tick(&[]);
        assert_eq!(engine.current_tick.0, 1);
        
        let event = RawPerceptionEvent {
            receiver: e1,
            target: EntityId(99),
            source_entity: e1,
            kind: BeliefKind::Position(Vec3::new(10.0, 0.0, 0.0)),
            confidence: 1.0,
        };
        
        engine.tick(&[event]);
        assert_eq!(engine.current_tick.0, 2);
    }
}
