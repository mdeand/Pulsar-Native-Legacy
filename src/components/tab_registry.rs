use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use super::editors::level::LevelEditor;
use super::editor_plugin::EditorMetadata;

static EDITOR_REGISTRY: Lazy<Mutex<HashMap<&'static str, LevelEditor>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));
pub fn register_editor(editor: LevelEditor) {
    let mut registry = EDITOR_REGISTRY.lock().unwrap();
    registry.insert(editor.name(), editor);
}

pub fn get_editor(name: &str) -> Option<LevelEditor> {
    let registry = EDITOR_REGISTRY.lock().unwrap();
    registry.get(name).cloned()
}

pub fn get_all_editors() -> Vec<LevelEditor> {
    let registry = EDITOR_REGISTRY.lock().unwrap();
    registry.values().cloned().collect()
}