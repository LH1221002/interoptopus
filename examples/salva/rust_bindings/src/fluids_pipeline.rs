use rapier3d::dynamics::{RigidBodyBuilder, RigidBodySet};
use rapier3d::geometry::{ColliderBuilder, ColliderSet};
use salva3d::object::interaction_groups::InteractionGroups;
use crate::error::{Error, FFIError};
use crate::math::{Point3, Vec3};
use interoptopus::{ffi_type, ffi_service, ffi_service_ctor, ffi_service_method};
use interoptopus::patterns::slice::FFISlice;
use rapier3d::math::Vector;
use salva3d::integrations::rapier::ColliderSampling;
use salva3d::object::Boundary;
use salva3d::sampling::shape_surface_ray_sample;
use crate::fluid::Fluid;
use crate::global_index::GLOBAL_FLUIDS;

#[ffi_type(opaque)]
pub struct FluidsPipeline {
    pub pipeline: salva3d::integrations::rapier::FluidsPipeline,
    colliders: rapier3d::geometry::ColliderSet,
    bodies: rapier3d::dynamics::RigidBodySet,
}

#[ffi_service(error = "FFIError")]
impl FluidsPipeline {
    #[ffi_service_ctor]
    pub fn new(particle_radius: f32, smoothing_factor: f32) -> Result<Self, Error> {
        let mut pipeline = salva3d::integrations::rapier::FluidsPipeline::new(particle_radius, smoothing_factor);

        // Colliders and bodies placeholder from faucet3 example
        let mut bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();
        let ground_rad = 0.15;
        let ground_handle = bodies.insert(RigidBodyBuilder::fixed().build());
        let co = ColliderBuilder::ball(ground_rad).build();
        let ball_samples = shape_surface_ray_sample(co.shape(), particle_radius).unwrap();
        let co_handle = colliders.insert_with_parent(co, ground_handle, &mut bodies);

        let bo_handle = pipeline.liquid_world.add_boundary(Boundary::new(Vec::new(), InteractionGroups::default()));

        pipeline
            .coupling
            .register_coupling(bo_handle, co_handle, ColliderSampling::StaticSampling(ball_samples));

        Ok(Self { pipeline, colliders, bodies })
    }
    
    pub fn add_fluid(&mut self, fluid_idx: u32) {
        if let Some(fluid) = GLOBAL_FLUIDS.lock().unwrap().get_mut(fluid_idx as usize) {
            let fluid = std::mem::take(fluid);
            self.pipeline.liquid_world.add_fluid(fluid.fluid);
        }
    }

    // pub fn add_fluid(&mut self, fluid: &Fluid) -> Result<(), Error> {
    //     self.pipeline.liquid_world.add_fluid(fluid.fluid);
    //     Ok(())
    // }

    // #[ffi_service_method(on_panic = "return_default")]
    pub fn step(
        &mut self,
        gravity: Vec3,
        dt: f32,
        // Here you would normally pass references to the collider set, bodies, etc.
        // For simplicity, we will assume you have a global or static handle, or do nothing.
    ) -> Result<(), Error> {
        // For a real integration you'd need to supply the ColliderSet and RigidBodySet references.
        // This depends on how you've structured your code. If you have global singletons, you might
        // access them here. Otherwise, you need to pass them in some form.

        self.pipeline.step(&gravity.into_native(), dt, &self.colliders, &mut self.bodies);
        Ok(())
    }

    // #[ffi_service_method(on_panic = "return_default")]
    // pub fn step(
    //     &mut self,
    //     gravity: Vec3,
    //     dt: f32,
    //     colliders: &rapier::geometry::ColliderSet,
    //     bodies: &mut rapier::dynamics::RigidBodySet,
    // ) -> Result<(), Error> {
    //     self.pipeline.step(
    //         &gravity.into_native(),
    //         dt,
    //         colliders,
    //         bodies,
    //     );
    //     Ok(())
    // }

    // pub fn register_coupling(
    //     &mut self,
    //     boundary_handle: crate::BoundaryHandle,
    //     collider_handle: crate::ColliderHandle,
    //     sampling_method: ColliderSampling,
    // ) -> Result<Option<crate::BoundaryHandle>, Error> {
    //     let native_sampling_method = match sampling_method {
    //         ColliderSampling::StaticSampling(points) => {
    //             salva3d::pipeline::ColliderSampling::StaticSampling(
    //                 points.iter().map(|p| p.into_native()).collect(),
    //             )
    //         }
    //         ColliderSampling::DynamicContactSampling => {
    //             salva3d::pipeline::ColliderSampling::DynamicContactSampling
    //         }
    //     };
    //
    //     Ok(self
    //         .pipeline
    //         .coupling
    //         .register_coupling(boundary_handle.into_native(), collider_handle.into_native(), native_sampling_method)
    //         .map(crate::BoundaryHandle::from_native))
    // }
    //
    // pub fn unregister_coupling(
    //     &mut self,
    //     collider_handle: crate::ColliderHandle,
    // ) -> Result<Option<crate::BoundaryHandle>, Error> {
    //     Ok(self
    //         .pipeline
    //         .coupling
    //         .unregister_coupling(collider_handle.into_native())
    //         .map(crate::BoundaryHandle::from_native))
    // }
    //
    // pub fn transmit_forces(&mut self, timestep: f32, boundaries: &crate::BoundarySet) -> Result<(), Error> {
    //     let timestep_manager = crate::TimestepManager::from_native(timestep);
    //     self.pipeline
    //         .coupling
    //         .transmit_forces(&timestep_manager, &boundaries.into_native());
    //     Ok(())
    // }
    //
    // pub fn update_boundaries(
    //     &mut self,
    //     timestep: f32,
    //     h: f32,
    //     particle_radius: f32,
    //     hgrid: &crate::geometry::HGrid,
    //     fluids: &mut [crate::object::Fluid],
    //     boundaries: &mut crate::BoundarySet,
    // ) -> Result<(), Error> {
    //     let timestep_manager = crate::TimestepManager::from_native(timestep);
    //     self.pipeline
    //         .coupling
    //         .update_boundaries(
    //             &timestep_manager,
    //             h,
    //             particle_radius,
    //             hgrid,
    //             fluids,
    //             boundaries,
    //         );
    //     Ok(())
    // }
}

// #[ffi_type(opaque)]
// pub enum ColliderSampling {
//     StaticSampling(FFISlice<Point3>),
//     DynamicContactSampling,
// }
