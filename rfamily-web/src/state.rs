use rfamily_core::preset_registry::PresetRegistry;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    registry: Arc<PresetRegistry>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(PresetRegistry::new()),
        }
    }

    pub fn preset_registry(&self) -> &PresetRegistry {
        &self.registry
    }
}
