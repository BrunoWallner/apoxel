use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Terminator {
    should_terminate: Arc<Mutex<bool>>,
}
impl Terminator {
    pub fn new() -> Self {
        Self {
            should_terminate: Arc::new(Mutex::new(false))
        }
    }
    pub fn terminate(&self) {
        let mut term = self.should_terminate.lock().unwrap();
        *term = true;
    }
    pub fn should_terminate(&self) -> bool {
        let term = self.should_terminate.lock().unwrap();
        *term
    }
}