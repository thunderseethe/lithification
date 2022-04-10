pub mod math {
  #[allow(unused_imports)]
  use wit_bindgen_wasmtime::{wasmtime, anyhow};
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
  }
  impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Vec3").field("x", &self.x).field("y", &self.y).field("z", &self.z).finish()}
  }
  impl wit_bindgen_wasmtime::Endian for Vec3 {
    fn into_le(self) -> Self {
      Self {
        x: self.x.into_le(),
        y: self.y.into_le(),
        z: self.z.into_le(),
      }
    }
    fn from_le(self) -> Self {
      Self {
        x: self.x.from_le(),
        y: self.y.from_le(),
        z: self.z.from_le(),
      }
    }
  }
  unsafe impl wit_bindgen_wasmtime::AllBytesValid for Vec3 {}
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct Mat3 {
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
  }
  impl std::fmt::Debug for Mat3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Mat3").field("x-axis", &self.x_axis).field("y-axis", &self.y_axis).field("z-axis", &self.z_axis).finish()}
  }
  impl wit_bindgen_wasmtime::Endian for Mat3 {
    fn into_le(self) -> Self {
      Self {
        x_axis: self.x_axis.into_le(),
        y_axis: self.y_axis.into_le(),
        z_axis: self.z_axis.into_le(),
      }
    }
    fn from_le(self) -> Self {
      Self {
        x_axis: self.x_axis.from_le(),
        y_axis: self.y_axis.from_le(),
        z_axis: self.z_axis.from_le(),
      }
    }
  }
  unsafe impl wit_bindgen_wasmtime::AllBytesValid for Mat3 {}
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct Affine3 {
    pub matrix3: Mat3,
    pub translation: Vec3,
  }
  impl std::fmt::Debug for Affine3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Affine3").field("matrix3", &self.matrix3).field("translation", &self.translation).finish()}
  }
  impl wit_bindgen_wasmtime::Endian for Affine3 {
    fn into_le(self) -> Self {
      Self {
        matrix3: self.matrix3.into_le(),
        translation: self.translation.into_le(),
      }
    }
    fn from_le(self) -> Self {
      Self {
        matrix3: self.matrix3.from_le(),
        translation: self.translation.from_le(),
      }
    }
  }
  unsafe impl wit_bindgen_wasmtime::AllBytesValid for Affine3 {}
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
  }
  impl std::fmt::Debug for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Vec4").field("x", &self.x).field("y", &self.y).field("z", &self.z).field("w", &self.w).finish()}
  }
  impl wit_bindgen_wasmtime::Endian for Vec4 {
    fn into_le(self) -> Self {
      Self {
        x: self.x.into_le(),
        y: self.y.into_le(),
        z: self.z.into_le(),
        w: self.w.into_le(),
      }
    }
    fn from_le(self) -> Self {
      Self {
        x: self.x.from_le(),
        y: self.y.from_le(),
        z: self.z.from_le(),
        w: self.w.from_le(),
      }
    }
  }
  unsafe impl wit_bindgen_wasmtime::AllBytesValid for Vec4 {}
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct Quat {
    pub data: Vec4,
  }
  impl std::fmt::Debug for Quat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Quat").field("data", &self.data).finish()}
  }
  impl wit_bindgen_wasmtime::Endian for Quat {
    fn into_le(self) -> Self {
      Self {
        data: self.data.into_le(),
      }
    }
    fn from_le(self) -> Self {
      Self {
        data: self.data.from_le(),
      }
    }
  }
  unsafe impl wit_bindgen_wasmtime::AllBytesValid for Quat {}
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct Mat4 {
    pub x_axis: Vec4,
    pub y_axis: Vec4,
    pub z_axis: Vec4,
    pub w_axis: Vec4,
  }
  impl std::fmt::Debug for Mat4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Mat4").field("x-axis", &self.x_axis).field("y-axis", &self.y_axis).field("z-axis", &self.z_axis).field("w-axis", &self.w_axis).finish()}
  }
  impl wit_bindgen_wasmtime::Endian for Mat4 {
    fn into_le(self) -> Self {
      Self {
        x_axis: self.x_axis.into_le(),
        y_axis: self.y_axis.into_le(),
        z_axis: self.z_axis.into_le(),
        w_axis: self.w_axis.into_le(),
      }
    }
    fn from_le(self) -> Self {
      Self {
        x_axis: self.x_axis.from_le(),
        y_axis: self.y_axis.from_le(),
        z_axis: self.z_axis.from_le(),
        w_axis: self.w_axis.from_le(),
      }
    }
  }
  unsafe impl wit_bindgen_wasmtime::AllBytesValid for Mat4 {}
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct GlobalTransform {
    pub affine: Affine3,
  }
  impl std::fmt::Debug for GlobalTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("GlobalTransform").field("affine", &self.affine).finish()}
  }
  impl wit_bindgen_wasmtime::Endian for GlobalTransform {
    fn into_le(self) -> Self {
      Self {
        affine: self.affine.into_le(),
      }
    }
    fn from_le(self) -> Self {
      Self {
        affine: self.affine.from_le(),
      }
    }
  }
  unsafe impl wit_bindgen_wasmtime::AllBytesValid for GlobalTransform {}
  
  /// Auxiliary data associated with the wasm exports.
  ///
  /// This is required to be stored within the data of a
  /// `Store<T>` itself so lifting/lowering state can be managed
  /// when translating between the host and wasm.
  #[derive(Default)]
  pub struct MathData {
  }
  pub struct Math<T> {
    get_state: Box<dyn Fn(&mut T) -> &mut MathData + Send + Sync>,
    append_translation: wasmtime::TypedFunc<(f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,), (i32,)>,
    identity_global_transform: wasmtime::TypedFunc<(), (i32,)>,
    matrix: wasmtime::TypedFunc<(f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,), (i32,)>,
    memory: wasmtime::Memory,
    new_global_transform: wasmtime::TypedFunc<(f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,), (i32,)>,
    new_vec3: wasmtime::TypedFunc<(f32,f32,f32,), (i32,)>,
    prepend_translation: wasmtime::TypedFunc<(f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,), (i32,)>,
    x_axis_vec3: wasmtime::TypedFunc<(), (i32,)>,
    y_axis_vec3: wasmtime::TypedFunc<(), (i32,)>,
    z_axis_vec3: wasmtime::TypedFunc<(), (i32,)>,
  }
  impl<T> Math<T> {
    #[allow(unused_variables)]
    
    /// Adds any intrinsics, if necessary for this exported wasm
    /// functionality to the `linker` provided.
    ///
    /// The `get_state` closure is required to access the
    /// auxiliary data necessary for these wasm exports from
    /// the general store's state.
    pub fn add_to_linker(
    linker: &mut wasmtime::Linker<T>,
    get_state: impl Fn(&mut T) -> &mut MathData + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()> {
      Ok(())
    }
    
    /// Instantiates the provided `module` using the specified
    /// parameters, wrapping up the result in a structure that
    /// translates between wasm and the host.
    ///
    /// The `linker` provided will have intrinsics added to it
    /// automatically, so it's not necessary to call
    /// `add_to_linker` beforehand. This function will
    /// instantiate the `module` otherwise using `linker`, and
    /// both an instance of this structure and the underlying
    /// `wasmtime::Instance` will be returned.
    ///
    /// The `get_state` parameter is used to access the
    /// auxiliary state necessary for these wasm exports from
    /// the general store state `T`.
    pub fn instantiate(
    mut store: impl wasmtime::AsContextMut<Data = T>,
    module: &wasmtime::Module,
    linker: &mut wasmtime::Linker<T>,
    get_state: impl Fn(&mut T) -> &mut MathData + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<(Self, wasmtime::Instance)> {
      Self::add_to_linker(linker, get_state)?;
      let instance = linker.instantiate(&mut store, module)?;
      Ok((Self::new(store, &instance,get_state)?, instance))
    }
    
    /// Low-level creation wrapper for wrapping up the exports
    /// of the `instance` provided in this structure of wasm
    /// exports.
    ///
    /// This function will extract exports from the `instance`
    /// defined within `store` and wrap them all up in the
    /// returned structure which can be used to interact with
    /// the wasm module.
    pub fn new(
    mut store: impl wasmtime::AsContextMut<Data = T>,
    instance: &wasmtime::Instance,
    get_state: impl Fn(&mut T) -> &mut MathData + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<Self> {
      let mut store = store.as_context_mut();
      let append_translation= instance.get_typed_func::<(f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,), (i32,), _>(&mut store, "append-translation")?;
      let identity_global_transform= instance.get_typed_func::<(), (i32,), _>(&mut store, "identity-global-transform")?;
      let matrix= instance.get_typed_func::<(f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,), (i32,), _>(&mut store, "matrix")?;
      let memory= instance
      .get_memory(&mut store, "memory")
      .ok_or_else(|| {
        anyhow::anyhow!("`memory` export not a memory")
      })?
      ;
      let new_global_transform= instance.get_typed_func::<(f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,), (i32,), _>(&mut store, "new-global-transform")?;
      let new_vec3= instance.get_typed_func::<(f32,f32,f32,), (i32,), _>(&mut store, "new-vec3")?;
      let prepend_translation= instance.get_typed_func::<(f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,), (i32,), _>(&mut store, "prepend-translation")?;
      let x_axis_vec3= instance.get_typed_func::<(), (i32,), _>(&mut store, "x-axis-vec3")?;
      let y_axis_vec3= instance.get_typed_func::<(), (i32,), _>(&mut store, "y-axis-vec3")?;
      let z_axis_vec3= instance.get_typed_func::<(), (i32,), _>(&mut store, "z-axis-vec3")?;
      Ok(Math{
        append_translation,
        identity_global_transform,
        matrix,
        memory,
        new_global_transform,
        new_vec3,
        prepend_translation,
        x_axis_vec3,
        y_axis_vec3,
        z_axis_vec3,
        get_state: Box::new(get_state),
        
      })
    }
    pub fn new_vec3(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,x: f32,y: f32,z: f32,)-> Result<Vec3, wasmtime::Trap> {
      let memory = &self.memory;
      let (result0_0,) = self.new_vec3.call(&mut caller, (x, y, z, ))?;
      let load1 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 0)?;
      let load2 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 8)?;
      let load3 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 16)?;
      Ok(Vec3{x:load1, y:load2, z:load3, })
    }
    pub fn x_axis_vec3(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,)-> Result<Vec3, wasmtime::Trap> {
      let memory = &self.memory;
      let (result0_0,) = self.x_axis_vec3.call(&mut caller, ())?;
      let load1 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 0)?;
      let load2 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 8)?;
      let load3 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 16)?;
      Ok(Vec3{x:load1, y:load2, z:load3, })
    }
    pub fn y_axis_vec3(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,)-> Result<Vec3, wasmtime::Trap> {
      let memory = &self.memory;
      let (result0_0,) = self.y_axis_vec3.call(&mut caller, ())?;
      let load1 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 0)?;
      let load2 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 8)?;
      let load3 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 16)?;
      Ok(Vec3{x:load1, y:load2, z:load3, })
    }
    pub fn z_axis_vec3(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,)-> Result<Vec3, wasmtime::Trap> {
      let memory = &self.memory;
      let (result0_0,) = self.z_axis_vec3.call(&mut caller, ())?;
      let load1 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 0)?;
      let load2 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 8)?;
      let load3 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 16)?;
      Ok(Vec3{x:load1, y:load2, z:load3, })
    }
    pub fn new_global_transform(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,position: Vec3,rotation: Quat,scale: Vec3,)-> Result<GlobalTransform, wasmtime::Trap> {
      let memory = &self.memory;
      let Vec3{ x:x0, y:y0, z:z0, } = position;
      let Quat{ data:data1, } = rotation;
      let Vec4{ x:x2, y:y2, z:z2, w:w2, } = data1;
      let Vec3{ x:x3, y:y3, z:z3, } = scale;
      let (result4_0,) = self.new_global_transform.call(&mut caller, (x0, y0, z0, x2, y2, z2, w2, x3, y3, z3, ))?;
      let load5 = memory.data_mut(&mut caller).load::<f32>(result4_0 + 0)?;
      let load6 = memory.data_mut(&mut caller).load::<f32>(result4_0 + 8)?;
      let load7 = memory.data_mut(&mut caller).load::<f32>(result4_0 + 16)?;
      let load8 = memory.data_mut(&mut caller).load::<f32>(result4_0 + 24)?;
      let load9 = memory.data_mut(&mut caller).load::<f32>(result4_0 + 32)?;
      let load10 = memory.data_mut(&mut caller).load::<f32>(result4_0 + 40)?;
      let load11 = memory.data_mut(&mut caller).load::<f32>(result4_0 + 48)?;
      let load12 = memory.data_mut(&mut caller).load::<f32>(result4_0 + 56)?;
      let load13 = memory.data_mut(&mut caller).load::<f32>(result4_0 + 64)?;
      let load14 = memory.data_mut(&mut caller).load::<f32>(result4_0 + 72)?;
      let load15 = memory.data_mut(&mut caller).load::<f32>(result4_0 + 80)?;
      let load16 = memory.data_mut(&mut caller).load::<f32>(result4_0 + 88)?;
      Ok(GlobalTransform{affine:Affine3{matrix3:Mat3{x_axis:Vec3{x:load5, y:load6, z:load7, }, y_axis:Vec3{x:load8, y:load9, z:load10, }, z_axis:Vec3{x:load11, y:load12, z:load13, }, }, translation:Vec3{x:load14, y:load15, z:load16, }, }, })
    }
    pub fn identity_global_transform(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,)-> Result<GlobalTransform, wasmtime::Trap> {
      let memory = &self.memory;
      let (result0_0,) = self.identity_global_transform.call(&mut caller, ())?;
      let load1 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 0)?;
      let load2 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 8)?;
      let load3 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 16)?;
      let load4 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 24)?;
      let load5 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 32)?;
      let load6 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 40)?;
      let load7 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 48)?;
      let load8 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 56)?;
      let load9 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 64)?;
      let load10 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 72)?;
      let load11 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 80)?;
      let load12 = memory.data_mut(&mut caller).load::<f32>(result0_0 + 88)?;
      Ok(GlobalTransform{affine:Affine3{matrix3:Mat3{x_axis:Vec3{x:load1, y:load2, z:load3, }, y_axis:Vec3{x:load4, y:load5, z:load6, }, z_axis:Vec3{x:load7, y:load8, z:load9, }, }, translation:Vec3{x:load10, y:load11, z:load12, }, }, })
    }
    pub fn matrix(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,this: GlobalTransform,)-> Result<Mat4, wasmtime::Trap> {
      let memory = &self.memory;
      let GlobalTransform{ affine:affine0, } = this;
      let Affine3{ matrix3:matrix31, translation:translation1, } = affine0;
      let Mat3{ x_axis:x_axis2, y_axis:y_axis2, z_axis:z_axis2, } = matrix31;
      let Vec3{ x:x3, y:y3, z:z3, } = x_axis2;
      let Vec3{ x:x4, y:y4, z:z4, } = y_axis2;
      let Vec3{ x:x5, y:y5, z:z5, } = z_axis2;
      let Vec3{ x:x6, y:y6, z:z6, } = translation1;
      let (result7_0,) = self.matrix.call(&mut caller, (x3, y3, z3, x4, y4, z4, x5, y5, z5, x6, y6, z6, ))?;
      let load8 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 0)?;
      let load9 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 8)?;
      let load10 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 16)?;
      let load11 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 24)?;
      let load12 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 32)?;
      let load13 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 40)?;
      let load14 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 48)?;
      let load15 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 56)?;
      let load16 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 64)?;
      let load17 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 72)?;
      let load18 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 80)?;
      let load19 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 88)?;
      let load20 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 96)?;
      let load21 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 104)?;
      let load22 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 112)?;
      let load23 = memory.data_mut(&mut caller).load::<f32>(result7_0 + 120)?;
      Ok(Mat4{x_axis:Vec4{x:load8, y:load9, z:load10, w:load11, }, y_axis:Vec4{x:load12, y:load13, z:load14, w:load15, }, z_axis:Vec4{x:load16, y:load17, z:load18, w:load19, }, w_axis:Vec4{x:load20, y:load21, z:load22, w:load23, }, })
    }
    pub fn prepend_translation(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,this: GlobalTransform,translation: Vec3,)-> Result<GlobalTransform, wasmtime::Trap> {
      let memory = &self.memory;
      let GlobalTransform{ affine:affine0, } = this;
      let Affine3{ matrix3:matrix31, translation:translation1, } = affine0;
      let Mat3{ x_axis:x_axis2, y_axis:y_axis2, z_axis:z_axis2, } = matrix31;
      let Vec3{ x:x3, y:y3, z:z3, } = x_axis2;
      let Vec3{ x:x4, y:y4, z:z4, } = y_axis2;
      let Vec3{ x:x5, y:y5, z:z5, } = z_axis2;
      let Vec3{ x:x6, y:y6, z:z6, } = translation1;
      let Vec3{ x:x7, y:y7, z:z7, } = translation;
      let (result8_0,) = self.prepend_translation.call(&mut caller, (x3, y3, z3, x4, y4, z4, x5, y5, z5, x6, y6, z6, x7, y7, z7, ))?;
      let load9 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 0)?;
      let load10 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 8)?;
      let load11 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 16)?;
      let load12 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 24)?;
      let load13 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 32)?;
      let load14 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 40)?;
      let load15 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 48)?;
      let load16 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 56)?;
      let load17 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 64)?;
      let load18 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 72)?;
      let load19 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 80)?;
      let load20 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 88)?;
      Ok(GlobalTransform{affine:Affine3{matrix3:Mat3{x_axis:Vec3{x:load9, y:load10, z:load11, }, y_axis:Vec3{x:load12, y:load13, z:load14, }, z_axis:Vec3{x:load15, y:load16, z:load17, }, }, translation:Vec3{x:load18, y:load19, z:load20, }, }, })
    }
    pub fn append_translation(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,this: GlobalTransform,translation: Vec3,)-> Result<GlobalTransform, wasmtime::Trap> {
      let memory = &self.memory;
      let GlobalTransform{ affine:affine0, } = this;
      let Affine3{ matrix3:matrix31, translation:translation1, } = affine0;
      let Mat3{ x_axis:x_axis2, y_axis:y_axis2, z_axis:z_axis2, } = matrix31;
      let Vec3{ x:x3, y:y3, z:z3, } = x_axis2;
      let Vec3{ x:x4, y:y4, z:z4, } = y_axis2;
      let Vec3{ x:x5, y:y5, z:z5, } = z_axis2;
      let Vec3{ x:x6, y:y6, z:z6, } = translation1;
      let Vec3{ x:x7, y:y7, z:z7, } = translation;
      let (result8_0,) = self.append_translation.call(&mut caller, (x3, y3, z3, x4, y4, z4, x5, y5, z5, x6, y6, z6, x7, y7, z7, ))?;
      let load9 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 0)?;
      let load10 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 8)?;
      let load11 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 16)?;
      let load12 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 24)?;
      let load13 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 32)?;
      let load14 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 40)?;
      let load15 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 48)?;
      let load16 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 56)?;
      let load17 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 64)?;
      let load18 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 72)?;
      let load19 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 80)?;
      let load20 = memory.data_mut(&mut caller).load::<f32>(result8_0 + 88)?;
      Ok(GlobalTransform{affine:Affine3{matrix3:Mat3{x_axis:Vec3{x:load9, y:load10, z:load11, }, y_axis:Vec3{x:load12, y:load13, z:load14, }, z_axis:Vec3{x:load15, y:load16, z:load17, }, }, translation:Vec3{x:load18, y:load19, z:load20, }, }, })
    }
  }
  use wit_bindgen_wasmtime::rt::RawMem;
}
