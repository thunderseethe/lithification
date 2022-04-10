use string_interner::{StringInterner, backend::BufferBackend, symbol::SymbolU32};
use std::{collections::HashMap, any::Any};

pub struct World {
    pub(crate) symbols: StringInterner<BufferBackend<SymbolU32>>,
    pub(crate) resources: HashMap<SymbolU32, Vec<u8>>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            symbols: StringInterner::new(),
            resources: HashMap::new(),
        }
    }
}

impl World {
    pub fn serialize_symbols(&self) -> anyhow::Result<Vec<u8>> {
        bincode::serialize(&self.symbols).map_err(|e| anyhow::anyhow!("Failed to serialize symbol table: {:?}", e))
    }

    pub fn intern_event_id(&mut self, s: &str) -> SymbolU32 {
        self.symbols.get_or_intern(s.to_owned() + "_event_")
    }

    pub fn intern_resource_id(&mut self, s: &str) -> SymbolU32 {
        self.symbols.get_or_intern(s.to_owned() + "_resource_")
    }
}