use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use rust_extensions::ApplicationStates;

pub struct AppStates {
    pub is_initialized: AtomicBool,
    pub is_shutting_down: Arc<AtomicBool>,
}

impl AppStates {
    pub fn new() -> Self {
        Self {
            is_initialized: AtomicBool::new(false),
            is_shutting_down: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.is_initialized.load(Ordering::Relaxed)
    }

    pub fn set_as_initialized(&self) {
        self.is_initialized.store(true, Ordering::SeqCst);
    }

    pub fn is_shutting_down(&self) -> bool {
        self.is_shutting_down.load(Ordering::Relaxed)
    }
}

impl ApplicationStates for AppStates {
    fn is_initialized(&self) -> bool {
        self.is_initialized.load(Ordering::Relaxed)
    }

    fn is_shutting_down(&self) -> bool {
        self.is_shutting_down.load(Ordering::Relaxed)
    }
}
