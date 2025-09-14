use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

lazy_static! {
    static ref GLOBAL_FRAME_COUNTER: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
    static ref GLOBAL_FPS: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
}

pub fn increment_frame() {
    let mut counter = GLOBAL_FRAME_COUNTER.lock().unwrap();
    *counter += 1;
}

pub fn get_frame_count() -> u64 {
    *GLOBAL_FRAME_COUNTER.lock().unwrap()
}

pub fn update_fps(fps: u64) {
    let mut fps_ref = GLOBAL_FPS.lock().unwrap();
    *fps_ref = fps;
}

pub fn get_fps() -> u64 {
    *GLOBAL_FPS.lock().unwrap()
}