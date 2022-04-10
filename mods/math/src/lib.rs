wit_bindgen_rust::export!("./math.wit");

macro_rules! glam_vec_iso {
    ($glam: ty, $math: ty, $($field:ident),*) => {
        impl From<$glam> for $math {
            fn from(v: $glam) -> Self {
                Self {
                    $($field: v.$field),*
                }
            }
        }
        impl Into<$glam> for $math {
            fn into(self) -> $glam {
                <$glam>::new($(self.$field),*)
            }
        }
    }
}

glam_vec_iso!(glam::Vec3A, math::Vec3, x, y, z);
glam_vec_iso!(glam::Vec3, math::Vec3, x, y, z);

glam_vec_iso!(glam::Vec4, math::Vec4, x, y, z, w);


macro_rules! glam_mat_iso {
    ($glam: ty, $math: ty, $($axis:ident),*) => {
        impl From<$glam> for $math {
            fn from(m: $glam) -> Self {
                Self {
                    $($axis: m.$axis.into()),*
                }
            }
        }
        impl Into<$glam> for $math {
            fn into(self) -> $glam {
                <$glam>::from_cols($(self.$axis.into()),*)
            }
        }
    }
}

glam_mat_iso!(glam::Mat3A, math::Mat3, x_axis, y_axis, z_axis);
glam_mat_iso!(glam::Mat3, math::Mat3, x_axis, y_axis, z_axis);
glam_mat_iso!(glam::Mat4, math::Mat4, x_axis, y_axis, z_axis, w_axis);

impl From<glam::Affine3A> for math::Affine3 {
    fn from(a: glam::Affine3A) -> Self {
        Self {
            matrix3: a.matrix3.into(),
            translation: a.translation.into(),
        }
    }
}
impl Into<glam::Affine3A> for math::Affine3 {
    fn into(self) -> glam::Affine3A {
        glam::Affine3A {
            matrix3: self.matrix3.into(),
            translation: self.translation.into(),
        }
    }
}

impl From<glam::Quat> for math::Quat {
    fn from(q: glam::Quat) -> Self {
        Self { data: glam::Vec4::from(q).into() }
    }
}
impl Into<glam::Quat> for math::Quat {
    fn into(self) -> glam::Quat {
        glam::Quat::from_vec4(self.data.into())
    }
}


struct Math {}
impl math::Math for Math {
    fn new_global_transform(position:math::Vec3, rotation:math::Quat, scale:math::Vec3,) -> math::GlobalTransform {
        math::GlobalTransform {
            affine: glam::Affine3A::from_scale_rotation_translation(
               scale.into(),
               rotation.into(),
               position.into()
            ).into()
        }
    }


    fn identity_global_transform() -> math::GlobalTransform {
        math::GlobalTransform {
            affine: glam::Affine3A::IDENTITY.into()
        }
    }


    fn matrix(trans: math::GlobalTransform,) -> math::Mat4 {
        let gaffine: glam::Affine3A = trans.affine.into();
        glam::Mat4::from(gaffine).into()
    }


    fn prepend_translation(this: math::GlobalTransform, translation: math::Vec3) -> math::GlobalTransform {
        let mut gaffine: glam::Affine3A = this.affine.into();
        let gtranslation: glam::Vec3A = translation.into();
        gaffine.translation += gtranslation;
        math::GlobalTransform { affine: gaffine.into() }
    }


    fn append_translation(this:math::GlobalTransform, translation:math::Vec3) -> math::GlobalTransform {
        let mut gaffine: glam::Affine3A = this.affine.into();
        let gtranslation: glam::Vec3A = translation.into();
        gaffine.translation += gaffine.matrix3 * gtranslation;
        math::GlobalTransform { affine: gaffine.into() }
    }

    fn new_vec3(x:f32,y:f32,z:f32,) -> math::Vec3 {
        math::Vec3 { x, y, z }
    }


    fn x_axis_vec3() -> math::Vec3 { 
        glam::Vec3::X.into()
    }
    fn y_axis_vec3() -> math::Vec3 {
        glam::Vec3::Y.into()
    }
    fn z_axis_vec3() -> math::Vec3 {
        glam::Vec3::Z.into()
    }
}
/*

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
*/

