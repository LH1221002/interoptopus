use crate::error::{Error, FFIError};
use crate::math::{Point3, Quaternion, Vec3};
use interoptopus::patterns::slice::FFISlice;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_service_method, ffi_type};
use salva3d::math::Translation;
use salva3d::object::interaction_groups::InteractionGroups;
use salva3d::solver::{Akinci2013SurfaceTension, XSPHViscosity};
use crate::fluids_pipeline::FluidsPipeline;
use crate::global_index::GLOBAL_FLUIDS;

#[ffi_type(opaque)]
pub struct Fluid {
    pub fluid: salva3d::object::Fluid,
}

impl Default for Fluid {
    fn default() -> Self {
        Self {
            fluid: salva3d::object::Fluid::new(Vec::new(), 0.0, 0.0, InteractionGroups::default()),
        }
    }
}

#[ffi_service(error = "FFIError")]
impl Fluid {
    #[ffi_service_ctor]
    pub fn new(particle_positions: FFISlice<Point3>, particle_radius: f32, density0: f32) -> Result<Self, Error> {
        // let positions_native = particle_positions.as_slice().iter().map(|p| p.into_native()).collect::<Vec<_>>();
        let positions_native = particle_positions.iter().map(|p| p.into_native()).collect::<Vec<_>>();

        let mut fluid = salva3d::object::Fluid::new(positions_native, particle_radius, density0, InteractionGroups::default());
        
        // TODO: nonpressure forces
        // Placeholder:
        let viscosity = XSPHViscosity::new(0.5, 0.0);
        let tension = Akinci2013SurfaceTension::new(1.0, 10.0);
        fluid.nonpressure_forces.push(Box::new(viscosity));
        fluid.nonpressure_forces.push(Box::new(tension));
        
        Ok(Self { fluid })
    }

    pub fn move_to_global_ownership(&mut self) -> Result<(), Error> {
        let fluid = std::mem::take(self);
        GLOBAL_FLUIDS.lock().unwrap().push(fluid);
        Ok(())
    }

    pub fn move_to_pipeline_ownership(&mut self, pipeline: &mut FluidsPipeline) -> Result<(), Error> {
        let fluid = std::mem::take(self);
        pipeline.pipeline.liquid_world.add_fluid(fluid.fluid);
        Ok(())
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn num_particles(&self) -> u32 {
        self.fluid.num_particles() as u32
    }
    
    pub fn delete_particle_at_next_timestep(&mut self, particle: u32) -> Result<(), Error> {
        if particle < self.fluid.num_particles() as u32 {
            self.fluid.delete_particle_at_next_timestep(particle as usize);
            Ok(())
        } else {
            // Err(Error::Other("Particle index out of bounds".to_string()))
            Err(Error::Bad)
        }
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn particle_radius(&self) -> f32 {
        self.fluid.particle_radius()
    }
    
    pub fn add_particles(
        &mut self,
        positions: FFISlice<Point3>,
        velocities: FFISlice<Vec3>, // originally Vec3
    ) -> Result<(), Error> {
        let native_positions = positions.iter().map(|p| p.into_native()).collect::<Vec<_>>();

        let native_velocities = if velocities.len() != positions.len() {
            Some(velocities.iter().map(|v| v.into_native()).collect::<Vec<_>>())
        } else {
            None
        };

        self.fluid.add_particles(&native_positions, native_velocities.as_deref());
        Ok(())
    }
    
    pub fn transform_by(&mut self, rotation: Quaternion, translation: Vec3) -> Result<(), Error> {
        let isometry = salva3d::math::Isometry::from_parts(Translation::from(translation.into_native()), rotation.into_native());
        self.fluid.transform_by(&isometry);
        Ok(())
    }
    
    pub fn z_sort(&mut self) -> Result<(), Error> {
        self.fluid.z_sort();
        Ok(())
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn default_particle_volume(&self) -> f32 {
        self.fluid.default_particle_volume()
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn particle_mass(&self, i: u32) -> f32 {
        if i < self.fluid.num_particles() as u32 {
            self.fluid.particle_mass(i as usize)
        } else {
            0.0
        }
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn particle_inv_mass(&self, i: u32) -> f32 {
        if i < self.fluid.num_particles() as u32 {
            self.fluid.particle_inv_mass(i as usize)
        } else {
            0.0
        }
    }
}
