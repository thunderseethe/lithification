use nalgebra as na;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GlobalTransform {
    pub isometry: na::Isometry3<f32>,
    pub scale: na::Vector3<f32>,
}

impl GlobalTransform {
    #[no_mangle]
    pub fn new(
        position: na::Translation3<f32>,
        rotation: na::UnitQuaternion<f32>,
        scale: na::Vector3<f32>,
    ) -> Self {
        Self {
            isometry: na::Isometry3::from_parts(position, rotation),
            scale,
        }
    }

    #[no_mangle]
    pub fn identity() -> Self {
        Self {
            isometry: na::Isometry3::identity(),
            scale: na::Vector3::new(1.0, 1.0, 1.0),
        }
    }

    /// Returns the local object matrix for the transform.
    #[inline]
    #[must_use]
    #[no_mangle]
    pub fn matrix(&self) -> na::Matrix4<f32> {
        self.isometry
            .to_homogeneous()
            .prepend_nonuniform_scaling(&self.scale)
    }

    #[inline]
    #[no_mangle]
    pub fn prepend_translation(&mut self, translation: na::Vector3<f32>) -> &mut Self {
        self.isometry.translation.vector += translation;
        self
    }

    #[inline]
    #[no_mangle]
    pub fn append_translation(&mut self, translation: na::Vector3<f32>) -> &mut Self {
        self.isometry.translation.vector += self.isometry.rotation * translation;
        self
    }
}

#[no_mangle]
pub fn new_vec3(x: f32, y: f32, z: f32) -> na::Vector3<f32> {
    na::Vector3::new(x, y, z)
}

#[no_mangle]
pub fn new_vec3_i32(x: i32, y: i32, z: i32) -> na::Vector3<i32> {
    na::Vector3::new(x, y, z)
}