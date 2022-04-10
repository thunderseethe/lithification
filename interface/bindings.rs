pub mod resources {
  #[allow(unused_imports)]
  use wit_bindgen_wasmtime::{wasmtime, anyhow};
  #[derive(Clone)]
  pub struct ResourceParam<'a,> {
    pub repr: &'a [u8],
  }
  impl<'a,> std::fmt::Debug for ResourceParam<'a,> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("ResourceParam").field("repr", &self.repr).finish()}
  }
  #[derive(Clone)]
  pub struct ResourceResult {
    pub repr: Vec<u8>,
  }
  impl std::fmt::Debug for ResourceResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("ResourceResult").field("repr", &self.repr).finish()}
  }
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct ResourceId {
    pub id: u32,
  }
  impl std::fmt::Debug for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("ResourceId").field("id", &self.id).finish()}
  }
  impl wit_bindgen_wasmtime::Endian for ResourceId {
    fn into_le(self) -> Self {
      Self {
        id: self.id.into_le(),
      }
    }
    fn from_le(self) -> Self {
      Self {
        id: self.id.from_le(),
      }
    }
  }
  unsafe impl wit_bindgen_wasmtime::AllBytesValid for ResourceId {}
  pub trait Resources: Sized {
    fn init_resource(&mut self,resource_name: & str,res: ResourceParam<'_,>,) -> ResourceId;
    
    fn lookup_resource(&mut self,resource_name: & str,) -> Option<ResourceId>;
    
    fn get_resource(&mut self,resource_id: ResourceId,) -> ResourceResult;
    
    fn write_resource(&mut self,resource_id: ResourceId,res: ResourceParam<'_,>,);
    
  }
  
  pub fn add_to_linker<T, U>(linker: &mut wasmtime::Linker<T>, get: impl Fn(&mut T) -> &mut U+ Send + Sync + Copy + 'static) -> anyhow::Result<()> 
  where U: Resources
  {
    use wit_bindgen_wasmtime::rt::get_memory;
    use wit_bindgen_wasmtime::rt::get_func;
    linker.func_wrap("resources", "init-resource", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32,arg3:i32| {
      let memory = &get_memory(&mut caller, "memory")?;
      let (mem, data) = memory.data_and_store_mut(&mut caller);
      let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
      let host = get(data);
      let ptr0 = arg0;
      let len0 = arg1;
      let ptr1 = arg2;
      let len1 = arg3;
      let param0 = _bc.slice_str(ptr0, len0)?;
      let param1 = ResourceParam{repr:_bc.slice(ptr1, len1)?, };
      let result2 = host.init_resource(param0, param1, );
      let ResourceId{ id:id3, } = result2;
      Ok(wit_bindgen_wasmtime::rt::as_i32(id3))
    })?;
    linker.func_wrap("resources", "lookup-resource", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32| {
      let memory = &get_memory(&mut caller, "memory")?;
      let (mem, data) = memory.data_and_store_mut(&mut caller);
      let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
      let host = get(data);
      let ptr0 = arg0;
      let len0 = arg1;
      let param0 = _bc.slice_str(ptr0, len0)?;
      let result1 = host.lookup_resource(param0, );
      let (result3_0,result3_1,) = match result1{
        None => { (0i32, 0i32)}
        Some(e) => { {
          let ResourceId{ id:id2, } = e;
          (1i32, wit_bindgen_wasmtime::rt::as_i32(id2))
        }}
      };
      let caller_memory = memory.data_mut(&mut caller);
      caller_memory.store(arg2 + 8, wit_bindgen_wasmtime::rt::as_i32(result3_1))?;
      caller_memory.store(arg2 + 0, wit_bindgen_wasmtime::rt::as_i32(result3_0))?;
      Ok(())
    })?;
    linker.func_wrap("resources", "get-resource", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32| {
      
      let func = get_func(&mut caller, "canonical_abi_realloc")?;
      let func_canonical_abi_realloc = func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
      let memory = &get_memory(&mut caller, "memory")?;
      let host = get(caller.data_mut());
      let param0 = ResourceId{id:arg0 as u32, };
      let result0 = host.get_resource(param0, );
      let ResourceResult{ repr:repr1, } = result0;
      let vec2 = repr1;
      let ptr2 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec2.len() as i32) * 1))?;
      let caller_memory = memory.data_mut(&mut caller);
      caller_memory.store_many(ptr2, vec2.as_ref())?;
      caller_memory.store(arg1 + 8, wit_bindgen_wasmtime::rt::as_i32(vec2.len() as i32))?;
      caller_memory.store(arg1 + 0, wit_bindgen_wasmtime::rt::as_i32(ptr2))?;
      Ok(())
    })?;
    linker.func_wrap("resources", "write-resource", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32| {
      let memory = &get_memory(&mut caller, "memory")?;
      let (mem, data) = memory.data_and_store_mut(&mut caller);
      let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
      let host = get(data);
      let ptr0 = arg1;
      let len0 = arg2;
      let param0 = ResourceId{id:arg0 as u32, };
      let param1 = ResourceParam{repr:_bc.slice(ptr0, len0)?, };
      host.write_resource(param0, param1, );
      Ok(())
    })?;
    Ok(())
  }
  use wit_bindgen_wasmtime::rt::RawMem;
}
