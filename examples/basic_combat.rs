//! Basic usage example — demonstrate a simple combat scenario.

use enemy_ai_engine::prelude::*;
use enemy_ai_engine::agent::personality::Personality;
use enemy_ai_engine::world::snapshot::*;

fn main() {
    println!("=== Enemy AI Engine - Basic Combat Example ===\n");

    // ── Create the engine ──
    let config = EngineConfig::default();
    let mut engine = AiEngine::<DefaultSystems>::with_seed(config, 42);

    // ── Create agents ──
    let mut agents = vec![
        // Team 0 — defenders
        AgentBuilder::new(EntityId(1), TeamId(0))
            .position(Vec3::new(0.0, 0.0, 0.0))
            .personality(Personality::soldier())
            .build(),
        AgentBuilder::new(EntityId(2), TeamId(0))
            .position(Vec3::new(5.0, 0.0, 0.0))
            .personality(Personality::sniper())
            .build(),

        // Team 1 — attackers
        AgentBuilder::new(EntityId(3), TeamId(1))
            .position(Vec3::new(30.0, 0.0, 10.0))
            .personality(Personality::berserker())
            .build(),
        AgentBuilder::new(EntityId(4), TeamId(1))
            .position(Vec3::new(35.0, 0.0, 5.0))
            .personality(Personality::elite())
            .build(),
    ];

    // ── Create entity snapshots (mirror of agents for world view) ──
    let entity_snapshots: Vec<EntitySnapshot> = agents.iter().map(|a| EntitySnapshot {
        id: a.id,
        team: a.team,
        squad: a.squad,
        position: a.position,
        rotation: a.rotation,
        velocity: a.velocity,
        health: a.health,
        max_health: a.max_health,
        is_alive: a.is_alive(),
        is_visible: true,
        weapon_id: a.weapon_id,
        in_cover: false,
        cover_id: CoverId::NONE,
    }).collect();

    // ── Run 10 ticks ──
    for tick_num in 0..10 {
        let snapshot = WorldSnapshot {
            tick: Tick(tick_num),
            delta_time: 1.0 / 60.0,
            entities: &entity_snapshots,
            sounds: &[],
            cover_points: &[],
            projectiles: &[],
            danger_zones: &[],
            objectives: &[],
            damage_events: &[],
        };

        let result = engine.tick(&snapshot, &mut agents);

        println!("── Tick {} ──", tick_num);
        for agent_result in &result.agent_actions {
            let agent = agents.iter().find(|a| a.id == agent_result.agent_id).unwrap();
            println!(
                "  Agent {} (Team {}) | State: {:?} | Health: {:.0}% | Actions: {:?}",
                agent.id.0,
                agent.team.0,
                agent.state,
                agent.health_ratio() * 100.0,
                agent_result.actions.iter().collect::<Vec<_>>(),
            );
        }
        println!();
    }

    println!("=== Simulation Complete ===");
    println!("Final tick: {}", engine.current_tick().0);
}
