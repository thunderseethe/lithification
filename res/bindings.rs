pub mod event {
  #[allow(unused_imports)]
  use wit_bindgen_wasmtime::{wasmtime, anyhow};
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct Symbol {
    pub id: u32,
  }
  impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Symbol").field("id", &self.id).finish()}
  }
  impl wit_bindgen_wasmtime::Endian for Symbol {
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
  unsafe impl wit_bindgen_wasmtime::AllBytesValid for Symbol {}
  #[derive(Clone)]
  pub struct Message {
    pub tag: Symbol,
    pub data: Vec<u8>,
  }
  impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Message").field("tag", &self.tag).field("data", &self.data).finish()}
  }
  pub trait Event: Sized {
    fn symbol(&mut self,s: & str,) -> Symbol;
    
    fn symbol_eq(&mut self,this: Symbol,other: Symbol,) -> bool;
    
    fn new_message(&mut self,tag: Symbol,data: &[u8],) -> Message;
    
  }
  
  pub fn add_to_linker<T, U>(linker: &mut wasmtime::Linker<T>, get: impl Fn(&mut T) -> &mut U+ Send + Sync + Copy + 'static) -> anyhow::Result<()> 
  where U: Event
  {
    use wit_bindgen_wasmtime::rt::get_memory;
    use wit_bindgen_wasmtime::rt::get_func;
    linker.func_wrap("event", "symbol", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32| {
      let memory = &get_memory(&mut caller, "memory")?;
      let (mem, data) = memory.data_and_store_mut(&mut caller);
      let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
      let host = get(data);
      let ptr0 = arg0;
      let len0 = arg1;
      let param0 = _bc.slice_str(ptr0, len0)?;
      let result1 = host.symbol(param0, );
      let Symbol{ id:id2, } = result1;
      Ok(wit_bindgen_wasmtime::rt::as_i32(id2))
    })?;
    linker.func_wrap("event", "symbol-eq", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32| {
      let host = get(caller.data_mut());
      let param0 = Symbol{id:arg0 as u32, };
      let param1 = Symbol{id:arg1 as u32, };
      let result0 = host.symbol_eq(param0, param1, );
      let result1 = match result0{
        false => { 0i32}
        true => { 1i32}
      };
      Ok(result1)
    })?;
    linker.func_wrap("event", "new-message", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32,arg3:i32| {
      
      let func = get_func(&mut caller, "canonical_abi_realloc")?;
      let func_canonical_abi_realloc = func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
      let memory = &get_memory(&mut caller, "memory")?;
      let (mem, data) = memory.data_and_store_mut(&mut caller);
      let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
      let host = get(data);
      let ptr0 = arg1;
      let len0 = arg2;
      let param0 = Symbol{id:arg0 as u32, };
      let param1 = _bc.slice(ptr0, len0)?;
      let result1 = host.new_message(param0, param1, );
      let Message{ tag:tag2, data:data2, } = result1;
      let Symbol{ id:id3, } = tag2;
      let vec4 = data2;
      let ptr4 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec4.len() as i32) * 1))?;
      let caller_memory = memory.data_mut(&mut caller);
      caller_memory.store_many(ptr4, vec4.as_ref())?;
      caller_memory.store(arg3 + 16, wit_bindgen_wasmtime::rt::as_i32(vec4.len() as i32))?;
      caller_memory.store(arg3 + 8, wit_bindgen_wasmtime::rt::as_i32(ptr4))?;
      caller_memory.store(arg3 + 0, wit_bindgen_wasmtime::rt::as_i32(wit_bindgen_wasmtime::rt::as_i32(id3)))?;
      Ok(())
    })?;
    Ok(())
  }
  use wit_bindgen_wasmtime::rt::RawMem;
}
