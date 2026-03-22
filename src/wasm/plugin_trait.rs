pub trait WasmPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn on_tick(&self, entity_id: u64);
    fn on_event(&self, entity_id: u64, event: &str, data: &str);
}
