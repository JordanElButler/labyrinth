use std::time::{Instant};

pub struct Timer {
    running: bool,
    before: Option<Instant>,
    elapsed_ms: i32,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            running: false,
            before: None,
            elapsed_ms: 0i32,
        }
    }
    pub fn start(&mut self) {
        self.running = true;
        self.before = Some(Instant::now());
    }
    pub fn stop(&mut self) {
        self.running = false;
        self.before = None;
    }
    pub fn get_elapsed_ms(&self) -> i32 {
        self.elapsed_ms
    }
    pub fn update(&mut self) {
        let now = Some(Instant::now());
        
        let duration = now.unwrap().duration_since(self.before.unwrap());
        self.elapsed_ms = duration.as_millis() as i32;
        self.before = now;
    }
}