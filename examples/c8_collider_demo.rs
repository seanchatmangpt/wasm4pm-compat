//! Example: Collider topology mutation detection.
//!
//! Demonstrates the Construct8 collider: when topology pressure forces hidden bodies
//! to manifest as visible vertices, and emitting collision proofs.
//!
//! Run: cargo run --example c8_collider_demo

fn main() {
    println!("=== Collider Demo ===\n");

    // Simulate a particle graph with hidden bodies (latency nodes)
    let mut topology = ParticleTopology {
        visible_vertices: vec![
            ("order_source", 0),
            ("market_gateway", 1),
            ("execution_engine", 2),
        ],
        hidden_bodies: vec![("network_delay", 0), ("queue_delay", 0), ("gc_pause", 0)],
        pressure_level: 0,
    };

    println!(
        "Initial topology: {} visible, {} hidden",
        topology.visible_vertices.len(),
        topology.hidden_bodies.len()
    );

    // Simulate increasing pressure (congestion)
    for iteration in 1..=4 {
        let pressure_delta = 25;
        topology.pressure_level += pressure_delta;

        println!(
            "\nIteration {}: Pressure increased to {}",
            iteration, topology.pressure_level
        );

        // Check if any hidden bodies should manifest
        let threshold = 50;
        if topology.pressure_level >= threshold {
            let hidden_count = topology.hidden_bodies.len();
            let manifest_count = (topology.pressure_level / 25).min(hidden_count as u64);

            for i in 0..manifest_count as usize {
                if i < topology.hidden_bodies.len() {
                    let (name, _) = topology.hidden_bodies[i];
                    topology
                        .visible_vertices
                        .push((name, topology.visible_vertices.len()));
                    println!("  ✓ Manifested hidden body: '{}'", name);
                }
            }

            emit_collision_proof(&topology, iteration as u64);
        }
    }

    println!(
        "\nFinal topology: {} visible vertices",
        topology.visible_vertices.len()
    );
    println!("✓ Collider demo complete");
}

#[derive(Debug)]
struct ParticleTopology {
    visible_vertices: Vec<(&'static str, usize)>,
    hidden_bodies: Vec<(&'static str, usize)>,
    pressure_level: u64,
}

fn emit_collision_proof(topology: &ParticleTopology, iteration: u64) {
    println!("  [CollisionProof]");
    println!("    iteration: {}", iteration);
    println!("    visible_count: {}", topology.visible_vertices.len());
    println!("    pressure_level: {}", topology.pressure_level);
    println!("    proof_hash: {}", compute_topology_hash(topology));
}

fn compute_topology_hash(topology: &ParticleTopology) -> String {
    let mut hash: u64 = 0;
    for (_, id) in &topology.visible_vertices {
        hash = hash.wrapping_mul(31).wrapping_add(*id as u64);
    }
    for (_, id) in &topology.hidden_bodies {
        hash = hash.wrapping_mul(31).wrapping_add(*id as u64);
    }
    hash = hash.wrapping_mul(31).wrapping_add(topology.pressure_level);
    format!("{:016x}", hash)
}
