use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

// Frame ID
pub static GLOBAL_FRAME_COUNTER: Lazy<Arc<Mutex<u64>>> = Lazy::new(|| Arc::new(Mutex::new(0u64)));
// Frame time in milliseconds
pub static GLOBAL_FRAME_TIME: Lazy<Arc<Mutex<u64>>> = Lazy::new(|| Arc::new(Mutex::new(0u64)));
// Average frame time in milliseconds
pub static GLOBAL_AVERAGE_FRAME_TIME: Lazy<Arc<Mutex<u64>>> = Lazy::new(|| Arc::new(Mutex::new(0u64)));