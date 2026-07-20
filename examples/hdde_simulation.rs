use enemy_ai_engine::engine::HDDEngine;
use enemy_ai_engine::stages::ingestion::RawPerceptionEvent;
use enemy_ai_engine::belief::record::BeliefKind;
use enemy_ai_engine::core::math::Vec3;
use enemy_ai_engine::core::id::EntityId;
use enemy_ai_engine::core::id::HierarchyLevel;

fn main() {
    let mut engine = HDDEngine::new();

    let commander = engine.spawn_entity(Vec3::new(0.0, 0.0, 0.0));
    engine.registry.hierarchy_level[commander.index()] = HierarchyLevel::PlatoonCommander;

    let squad_leader = engine.spawn_entity(Vec3::new(5.0, 0.0, 5.0));
    engine.registry.hierarchy_level[squad_leader.index()] = HierarchyLevel::SquadLeader;
    engine.registry.set_parent(squad_leader, commander);

    let soldier_a = engine.spawn_entity(Vec3::new(10.0, 0.0, 10.0));
    engine.registry.set_parent(soldier_a, squad_leader);

    let soldier_b = engine.spawn_entity(Vec3::new(12.0, 0.0, 8.0));
    engine.registry.set_parent(soldier_b, squad_leader);

    let enemy_id = EntityId(999);

    println!("=== HDDE Simulation ===");
    println!("Entities: Commander({}), SquadLeader({}), SoldierA({}), SoldierB({})",
        commander.0, squad_leader.0, soldier_a.0, soldier_b.0);
    println!();

    let events = vec![
        RawPerceptionEvent {
            receiver: soldier_a,
            target: enemy_id,
            source_entity: soldier_a,
            kind: BeliefKind::Position(Vec3::new(20.0, 0.0, 15.0)),
            confidence: 0.9,
        },
        RawPerceptionEvent {
            receiver: soldier_a,
            target: enemy_id,
            source_entity: soldier_a,
            kind: BeliefKind::ThreatLevel(0.8),
            confidence: 0.9,
        },
    ];

    for tick in 0..15 {
        let incoming = if tick == 0 { &events[..] } else { &[] };
        engine.tick(incoming);

        println!("Tick {}: ", tick + 1);
        for (name, id) in [("Commander", commander), ("SquadLeader", squad_leader), ("SoldierA", soldier_a), ("SoldierB", soldier_b)] {
            println!("  {}: {:?}", name, engine.registry.chosen_action[id.index()]);
        }
        println!();
    }

    println!("=== Simulation Complete ===");
}
