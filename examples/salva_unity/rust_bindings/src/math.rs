use nalgebra::UnitQuaternion;
use salva3d::math::Real;
use interoptopus::ffi_type;

#[ffi_type]
#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[ffi_type]
#[derive(Copy, Clone)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[ffi_type]
#[derive(Copy, Clone)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn into_native(self) -> UnitQuaternion<Real> {
        UnitQuaternion::new_normalize(nalgebra::Quaternion::new(self.w, self.x, self.y, self.z))
    }

    pub fn from_native(q: UnitQuaternion<Real>) -> Self {
        let q = q.quaternion();
        Self { x: q.i, y: q.j, z: q.k, w: q.w }
    }
    
}

impl Vec3 {
    pub fn into_native(self) -> salva3d::math::Vector<Real> {
        salva3d::math::Vector::new(self.x, self.y, self.z)
    }

    pub fn from_native(v: salva3d::math::Vector<Real>) -> Self {
        Self { x: v.x, y: v.y, z: v.z }
    }
}

impl Point3 {   // TODO: Maybe use surrogate types for this conversion, more efficient?
    pub fn into_native(self) -> salva3d::math::Point<Real> {
        salva3d::math::Point::new(self.x, self.y, self.z)
    }

    pub fn to_vec(self) -> salva3d::math::Vector<Real> {
        salva3d::math::Vector::new(self.x, self.y, self.z)
    }

    pub fn from_native(p: salva3d::math::Point<Real>) -> Self {
        Self { x: p.x, y: p.y, z: p.z }
    }
}
