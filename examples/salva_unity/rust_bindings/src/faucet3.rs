extern crate nalgebra as na;

use na::{Point3, Vector3};
use rapier3d::geometry::{ColliderBuilder, ColliderSet};
use rapier3d::{
    dynamics::{RigidBodyBuilder, RigidBodySet},
    prelude::{ImpulseJointSet, IntegrationParameters, MultibodyJointSet},
};
use salva3d::integrations::rapier::{ColliderSampling, FluidsPipeline};
use salva3d::object::interaction_groups::InteractionGroups;
use salva3d::object::{Boundary, Fluid};
use salva3d::sampling::shape_surface_ray_sample;
use salva3d::solver::{Akinci2013SurfaceTension, XSPHViscosity};
use std::f32;

#[path = "./helper.rs"]
mod helper;

const PARTICLE_RADIUS: f32 = 0.025 / 2.0;
const SMOOTHING_FACTOR: f32 = 2.0;

fn mainn() {
    // Gravity and integration parameters.
    let gravity = Vector3::y() * -9.81;
    let dt = 1.0 / 50.0; // Fixed timestep for simplicity.
    let integration_parameters = IntegrationParameters {
        dt,
        ..IntegrationParameters::default()
    };

    // Initialize Rapier sets
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut impulse_joints = ImpulseJointSet::new();
    let mut multibody_joints = MultibodyJointSet::new();

    // Create the fluids pipeline directly
    let mut fluids_pipeline = FluidsPipeline::new(PARTICLE_RADIUS, SMOOTHING_FACTOR);

    // Initialize the fluid
    let viscosity = XSPHViscosity::new(0.5, 0.0);
    let tension = Akinci2013SurfaceTension::new(1.0, 10.0);
    let mut fluid = Fluid::new(Vec::new(), PARTICLE_RADIUS, 1000.0, InteractionGroups::default());
    fluid.nonpressure_forces.push(Box::new(viscosity));
    fluid.nonpressure_forces.push(Box::new(tension));
    let fluid_handle = fluids_pipeline.liquid_world.add_fluid(fluid);

    // Setup the ground boundary
    let ground_rad = 0.15;
    let ground_handle = bodies.insert(RigidBodyBuilder::fixed().build());
    let co = ColliderBuilder::ball(ground_rad).build();
    let ball_samples = shape_surface_ray_sample(co.shape(), PARTICLE_RADIUS).unwrap();
    let co_handle = colliders.insert_with_parent(co, ground_handle, &mut bodies);

    let bo_handle = fluids_pipeline.liquid_world.add_boundary(Boundary::new(Vec::new(), InteractionGroups::default()));

    fluids_pipeline
        .coupling
        .register_coupling(bo_handle, co_handle, ColliderSampling::StaticSampling(ball_samples));

    // Variables for particle insertion
    let mut last_t = 0.0;
    let mut current_time = 0.0;
    let max_time = 1.0; // Run the simulation for 1 second.

    // Simulation loop
    while current_time < max_time {
        current_time += dt;

        // Remove particles that have fallen below y = -2.0
        {
            let fluid = fluids_pipeline.liquid_world.fluids_mut().get_mut(fluid_handle).unwrap();
            for i in 0..fluid.num_particles() {
                if fluid.positions[i].y < -2.0 {
                    fluid.delete_particle_at_next_timestep(i);
                }
            }
        }

        // Inject new particles every 0.06 seconds
        if current_time - last_t >= 0.06 {
            last_t = current_time;
            let fluid = fluids_pipeline.liquid_world.fluids_mut().get_mut(fluid_handle).unwrap();

            let height = 0.6;
            let diam = PARTICLE_RADIUS * 2.0;
            let nparticles = 10;
            let mut particles = Vec::new();
            let mut velocities = Vec::new();
            let shift = -nparticles as f32 * PARTICLE_RADIUS;
            let vel = 0.0;

            for i in 0..nparticles {
                for j in 0..nparticles {
                    let pos = Point3::new(i as f32 * diam, height, j as f32 * diam);
                    particles.push(pos + Vector3::new(shift, 0.0, shift));
                    velocities.push(Vector3::y() * vel);
                }
            }

            fluid.add_particles(&particles, Some(&velocities));
        }

        // Step the fluids simulation forward by dt
        fluids_pipeline.step(&gravity, dt, &colliders, &mut bodies);

        // Print particle positions
        let fluid = fluids_pipeline.liquid_world.fluids_mut().get_mut(fluid_handle).unwrap();

        println!(
            "Time: {:.3} s, Number of particles: {}",
            current_time,
            fluid.num_particles()
        );
        for (i, pos) in fluid.positions.iter().enumerate() {
            if i >= 1 { continue; }
            println!("Particle {}: {:?}", i, pos);
        }
    }
}
