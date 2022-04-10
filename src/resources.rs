use crate::world::World;
use string_interner::symbol::{Symbol, SymbolU32};

wit_bindgen_wasmtime::export!("./interface/resources.wit");
use resources::{ResourceId, Resources};
impl From<SymbolU32> for ResourceId {
    fn from(s: SymbolU32) -> Self {
        ResourceId { id: s.to_usize() as u32 }
    }
}
impl Into<SymbolU32> for ResourceId {
    fn into(self) -> SymbolU32 {
        SymbolU32::try_from_usize(self.id as usize).unwrap()
    }
}


impl Resources for World {

    fn lookup_resource(&mut self, name: &str) -> Option<ResourceId> {
        self.symbols.get(name).map(|s| ResourceId { id: s.to_usize() as u32 })
    }

    fn init_resource(&mut self, name: &str, res:resources::ResourceParam<'_>) -> ResourceId {
        let id = self.symbols.get_or_intern(name);
        self.resources.insert(id, res.repr.to_owned());
        ResourceId::from(id)
    }


    fn get_resource(&mut self,resource_id:ResourceId,) -> resources::ResourceResult {
        let sym = resource_id.into();
        self.resources.get(&sym).map(|bytes| {
            resources::ResourceResult { repr: bytes.clone() }
        }).ok_or_else(|| {
            self.symbols.resolve(sym)
                .map(|name| anyhow::anyhow!("No resource initialized for {}", name))
                .unwrap_or_else(|| anyhow::anyhow!("No resource initialized for id {}", resource_id.id))
        })
        .expect("Failed to find initialized resource")
    }


    fn write_resource(&mut self,resource_id:ResourceId,res:resources::ResourceParam< '_,>,) {
        self.resources.insert(resource_id.into(), res.repr.to_owned());
    }


    /*fn get_resource(&mut self, res_id: ResourceId) -> Vec<u8> { 
        SymbolU32::try_from_usize(res_id.id as usize)
            .as_ref()
            .and_then(|sym| self.resources.get(sym))
            .map(|b| b.clone())
            .expect("Did not have resource")
    }

    fn init_resource(&mut self,resource_name: &str,res:resources::ResourceParam< '_,>,) -> ResourceId {
        todo!()
    }*/
}