use enemy_ai_engine::belief::record::BeliefKind;
use enemy_ai_engine::core::action::{ActionCandidate, ActionKind};
use enemy_ai_engine::core::id::EntityId;
use enemy_ai_engine::core::math::Vec3;
use enemy_ai_engine::engine::{EngineHooks, HDDEngine};
use enemy_ai_engine::registry::soa::SoARegistry;
use enemy_ai_engine::stages::ingestion::RawPerceptionEvent;
use std::time::Instant;

const TARGET_FOOD: u16 = 1;
const TARGET_REST: u16 = 2;

const ACTION_EAT: u16 = 1;
const ACTION_SLEEP: u16 = 2;

struct SimulationHooks;

impl EngineHooks for SimulationHooks {
    fn generate_candidates(&self, idx: usize, registry: &mut SoARegistry) {
        let mut candidates = registry.candidates[idx];
        let mut c_idx = 0;
        while c_idx < 16 && candidates[c_idx].is_some() {
            c_idx += 1;
        }

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

        // Custom Candidate Generation
        for record in registry.beliefs[idx].iter() {
            if let BeliefKind::CustomPos(id, pos) = record.kind {
                if id == TARGET_FOOD {
                    add!(ActionKind::CustomPos(ACTION_EAT, pos));
                } else if id == TARGET_REST {
                    add!(ActionKind::CustomPos(ACTION_SLEEP, pos));
                }
            }
        }

        // Add a generic explore candidate to encourage emergent behaviour when no needs are pressing
        let rx = registry.rng[idx].gen_f32() * 50.0 - 25.0;
        let ry = registry.rng[idx].gen_f32() * 50.0 - 25.0;
        add!(ActionKind::CustomPos(
            0,
            registry.positions[idx] + Vec3::new(rx, ry, 0.0)
        ));

        registry.candidates[idx] = candidates;
    }

    fn evaluate_utility(
        &self,
        idx: usize,
        candidate: &ActionCandidate,
        registry: &SoARegistry,
    ) -> Option<f32> {
        let hunger = registry.hunger[idx];
        let fatigue = registry.fatigue[idx];
        let cur = registry.curiosity[idx];

        match candidate.kind {
            ActionKind::CustomPos(id, _) => {
                if id == ACTION_EAT {
                    Some(hunger * 1.5)
                } else if id == ACTION_SLEEP {
                    Some(fatigue * 1.5)
                } else if id == 0 {
                    // Explore
                    Some(cur * 0.8)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

struct World {
    food_sources: Vec<Vec3>,
    rest_areas: Vec<Vec3>,
    agent_ids: Vec<EntityId>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let stress_mode = args.contains(&"--stress".to_string());
    let debug_mode = args.contains(&"--debug".to_string());

    let num_agents = if stress_mode {
        let mut count = 10000;
        if let Some(idx) = args.iter().position(|a| a == "--stress") {
            if idx + 1 < args.len() {
                if let Ok(c) = args[idx + 1].parse::<usize>() {
                    count = c;
                }
            }
        }
        count
    } else {
        if debug_mode {
            1
        } else {
            100
        }
    };

    println!("Initializing HDDE Simulation with {} agents...", num_agents);
    let mut engine = HDDEngine::new();
    let mut world = World {
        food_sources: vec![Vec3::new(10.0, 10.0, 0.0), Vec3::new(-50.0, 20.0, 0.0)],
        rest_areas: vec![Vec3::new(0.0, -10.0, 0.0)],
        agent_ids: Vec::with_capacity(num_agents),
    };

    for _ in 0..num_agents {
        world.agent_ids.push(engine.spawn_entity(Vec3::zero()));
    }

    let hooks = SimulationHooks;
    let mut events = Vec::with_capacity(num_agents * 2);
    let ticks = if stress_mode { 100 } else { 10 };
    let mut total_stage_times = [std::time::Duration::ZERO; 7];
    let start_time = Instant::now();

    for t in 0..ticks {
        events.clear();

        // 1. World Simulation & Perception (Zero Allocation in loop via clear)
        for &id in &world.agent_ids {
            let idx = id.index();
            let pos = engine.registry.positions[idx];

            // Simple logic: if hungry, perceive food
            if engine.registry.hunger[idx] > 0.5 {
                events.push(RawPerceptionEvent {
                    receiver: id,
                    target: EntityId::NONE,
                    source_entity: EntityId::NONE,
                    kind: BeliefKind::CustomPos(TARGET_FOOD, world.food_sources[0]),
                    confidence: 1.0,
                });
            }

            if engine.registry.fatigue[idx] > 0.5 {
                events.push(RawPerceptionEvent {
                    receiver: id,
                    target: EntityId::NONE,
                    source_entity: EntityId::NONE,
                    kind: BeliefKind::CustomPos(TARGET_REST, world.rest_areas[0]),
                    confidence: 1.0,
                });
            }
        }

        // 2. HDDE Tick Profiled
        let stage_times = engine.tick_profiled(&events, &hooks);
        for i in 0..7 {
            total_stage_times[i] += stage_times[i];
        }

        // 3. Apply Actions to World
        for &id in &world.agent_ids {
            let idx = id.index();
            let action = &engine.registry.chosen_action[idx];

            match action {
                ActionKind::CustomPos(action_id, target) => {
                    let mut pos = engine.registry.positions[idx];
                    let dir = (*target - pos).normalize();
                    pos = pos + dir * 1.0;
                    engine.registry.positions[idx] = pos;

                    // If reached food/rest, reset needs
                    if pos.distance_sq(*target) < 4.0 {
                        if *action_id == ACTION_EAT {
                            engine.registry.hunger[idx] = 0.0;
                        } else if *action_id == ACTION_SLEEP {
                            engine.registry.fatigue[idx] = 0.0;
                        }
                    }
                }
                _ => {}
            }
        }

        if debug_mode {
            println!("--- Tick {} ---", engine.current_tick.0);
            let idx = world.agent_ids[0].index();
            println!(
                "Needs: Hunger={:.2}, Fatigue={:.2}, Curiosity={:.2}",
                engine.registry.hunger[idx],
                engine.registry.fatigue[idx],
                engine.registry.curiosity[idx]
            );

            #[cfg(feature = "debug-trace")]
            {
                println!("Candidates:");
                for (kind, score) in &engine.registry.decision_traces[idx] {
                    println!("  - {:?} : Score {:.2}", kind, score);
                }
            }

            println!("Decision: {:?}", engine.registry.chosen_action[idx]);
        }
    }

    let elapsed = start_time.elapsed();

    println!("\nSimulation Summary:");
    println!("Total Agents: {}", num_agents);
    println!("Total Ticks: {}", ticks);
    println!("Total Time: {:.2?}", elapsed);
    if elapsed.as_secs_f64() > 0.0 {
        println!(
            "TPS (Ticks Per Second): {:.0}",
            (ticks as f64) / elapsed.as_secs_f64()
        );
    }

    println!("\nAverage Stage Times:");
    let stage_names = [
        "Ingestion",
        "Decay",
        "Needs & Candidates",
        "Utility",
        "Commit",
        "Propagation",
        "Comms",
    ];
    for i in 0..7 {
        println!(
            "{:<20}: {:.2?}",
            stage_names[i],
            total_stage_times[i] / (ticks as u32)
        );
    }
}
