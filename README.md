# HDDE: A Zero-Allocation Hierarchical Utility Architecture

A high-performance, purely data-oriented (ECS/SoA based) game AI engine written in Rust.

## Architecture

This engine implements a Hierarchical Distributed Decision Engine (HDDE). It replaces traditional monolithic AI controllers (like Behavior Trees) with a 7-stage data pipeline running across flat arrays (Struct of Arrays).

### Key Features
- **Zero Allocations**: All data is stored in pre-allocated static-size arrays. No runtime heap allocations occur during the core update loop.
- **Data-Oriented (SoA)**: Maximum CPU cache efficiency.
- **Utility AI**: Replaces complex behavior trees with multiplicative considerations, providing smooth, unscripted emergent behaviors.
- **Hierarchical Propagation**: Bottom-up status reporting and top-down intent directives with simulated communication latency.
- **A-Life Needs System**: Agents have internal needs (hunger, fatigue, curiosity, self-preservation) that naturally decay and affect their tactical utility scores over time, enabling offline open-world simulation.

## Building and Running

Ensure you have Rust installed.

```bash
# Compile the engine
cargo build --release

# Run the hierarchy simulation example
cargo run --example hdde_simulation
```

## Structure

- `src/core/`: Basic data types (Vec3, ActionKinds, EntityId).
- `src/registry/`: The `SoARegistry` which holds all entity arrays.
- `src/belief/`: Belief system and finite-size belief stores.
- `src/comm/`: The communication channel for delaying and routing messages.
- `src/stages/`: The 7-stage pipeline (Ingestion, Decay, Needs, Candidates, Utility, Commit, Comms/Propagation).

## License

MIT OR Apache-2.0
