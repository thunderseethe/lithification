#[no_mangle]
pub static GLOBAL_TRANSFORM_SIZE: usize = std::mem::size_of::<GlobalTransform>();

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GlobalTransform {
    affine: glam::Affine3A,
}

impl GlobalTransform {
    #[no_mangle]
    pub fn new_global_transform(
        position: glam::Vec3,
        rotation: glam::Quat,
        scale: glam::Vec3,
    ) -> Self {
        Self {
            affine: glam::Affine3A::from_scale_rotation_translation(
                scale,
                rotation,
                position
            )
        }
    }

    #[no_mangle]
    pub fn global_transform_identity() -> Self {
        Self {
            affine: glam::Affine3A::IDENTITY,
        }
    }

    /// Returns the local object matrix for the transform.
    #[inline]
    #[must_use]
    #[no_mangle]
    pub fn matrix(&self) -> glam::Mat4 {
        glam::Mat4::from(self.affine)
    }

    #[inline]
    #[no_mangle]
    pub fn prepend_translation(&mut self, translation: glam::Vec3A) -> &mut Self {
        self.affine.translation += translation;
        self
    }

    #[inline]
    #[no_mangle]
    pub fn append_translation(&mut self, translation: glam::Vec3A) -> &mut Self {
        self.affine.translation += self.affine.matrix3 * translation;
        self
    }
}

#[no_mangle]
pub fn new_vec3(x: f32, y: f32, z: f32) -> glam::Vec3 {
    glam::Vec3::new(x, y, z)
}

#[no_mangle]
pub fn vec3_x_axis() -> glam::Vec3 {
    glam::Vec3::X
}

#[no_mangle]
pub fn vec3_y_axis() -> glam::Vec3 {
    glam::Vec3::Y
}

#[no_mangle]
pub fn vec3_z_axis() -> glam::Vec3 {
    glam::Vec3::Z
}

pub fn quat_id() -> glam::Quat {
    glam::Quat::IDENTITY
}

