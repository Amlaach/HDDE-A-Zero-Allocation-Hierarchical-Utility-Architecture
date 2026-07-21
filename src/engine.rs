use crate::comm::channel::CommChannel;
use crate::core::time::Tick;
use crate::registry::soa::SoARegistry;
use crate::stages::ingestion::RawPerceptionEvent;
use crate::core::action::ActionCandidate;
use std::time::{Duration, Instant};

pub trait EngineHooks {
    fn generate_candidates(&self, _entity_idx: usize, _registry: &mut SoARegistry) {}
    fn evaluate_utility(&self, _entity_idx: usize, _candidate: &ActionCandidate, _registry: &SoARegistry) -> Option<f32> { None }
}

pub struct DefaultHooks;
impl EngineHooks for DefaultHooks {}

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
        self.tick_with_hooks(incoming_events, &DefaultHooks);
    }

    pub fn tick_with_hooks(&mut self, incoming_events: &[RawPerceptionEvent], hooks: &impl EngineHooks) {
        self.current_tick = self.current_tick + 1;

        crate::stages::ingestion::run(&mut self.registry, incoming_events, self.current_tick);
        crate::stages::decay::run(&mut self.registry, self.current_tick);
        crate::stages::needs::run(&mut self.registry);
        crate::stages::candidates::run(&mut self.registry, hooks);
        crate::stages::utility::run(&mut self.registry, hooks);
        crate::stages::commit::run(&mut self.registry);
        crate::stages::propagation::run(&self.registry, &mut self.comm_channel, self.current_tick);
        crate::stages::comms::run(
            &mut self.registry,
            &mut self.comm_channel,
            self.current_tick,
        );
    }

    pub fn tick_profiled(&mut self, incoming_events: &[RawPerceptionEvent], hooks: &impl EngineHooks) -> [Duration; 7] {
        self.current_tick = self.current_tick + 1;
        let mut timings = [Duration::ZERO; 7];

        let t0 = Instant::now();
        crate::stages::ingestion::run(&mut self.registry, incoming_events, self.current_tick);
        timings[0] = t0.elapsed();

        let t1 = Instant::now();
        crate::stages::decay::run(&mut self.registry, self.current_tick);
        timings[1] = t1.elapsed();

        let t2 = Instant::now();
        crate::stages::needs::run(&mut self.registry);
        // We group Needs and Candidates into stage 2 logically, or we can just profile them together or separately.
        crate::stages::candidates::run(&mut self.registry, hooks);
        timings[2] = t2.elapsed();

        let t3 = Instant::now();
        crate::stages::utility::run(&mut self.registry, hooks);
        timings[3] = t3.elapsed();

        let t4 = Instant::now();
        crate::stages::commit::run(&mut self.registry);
        timings[4] = t4.elapsed();

        let t5 = Instant::now();
        crate::stages::propagation::run(&self.registry, &mut self.comm_channel, self.current_tick);
        timings[5] = t5.elapsed();

        let t6 = Instant::now();
        crate::stages::comms::run(
            &mut self.registry,
            &mut self.comm_channel,
            self.current_tick,
        );
        timings[6] = t6.elapsed();

        timings
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
    use crate::belief::record::BeliefKind;
    use crate::core::id::EntityId;
    use crate::core::math::Vec3;

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
