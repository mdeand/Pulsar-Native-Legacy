use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use super::editor_plugin::EditorMetadata;

static EDITOR_REGISTRY: Lazy<Mutex<HashMap<&'static str, Box<dyn EditorMetadata>>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn register_editor(editor: impl EditorMetadata + 'static) {
    let mut registry = EDITOR_REGISTRY.lock().unwrap();
    registry.insert(editor.name(), Box::new(editor));
}

pub fn get_editor(name: &str) -> Option<Box<dyn EditorMetadata>> {
    let registry = EDITOR_REGISTRY.lock().unwrap();
    registry.get(name).cloned()
}

pub fn get_all_editors() -> Vec<Box<dyn EditorMetadata>> {
    let registry = EDITOR_REGISTRY.lock().unwrap();
    registry.values().cloned().collect()
}