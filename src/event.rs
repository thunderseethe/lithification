use string_interner::symbol::Symbol;

wit_bindgen_wasmtime::export!("./interface/event.wit");

impl event::Event for crate::world::World {
    fn new_event_id(&mut self, s: &str) -> event::EventId {
        let id = self.intern_event_id(s).to_usize() as u32;
        event::EventId { id }
    }

    fn event_id_eq(&mut self, 
        this: event::EventId, 
        other: event::EventId) -> bool {
        this.id == other.id
    }

    fn new_message(&mut self, tag: event::EventId, data: &[u8]) -> event::Message {
        event::Message { tag, data: data.to_vec() }
    }
}