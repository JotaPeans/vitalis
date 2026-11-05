use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::time::{Duration, Instant};

const REOPEN_GUARD: Duration = Duration::from_millis(250);

pub struct PopoverState {
    visible: AtomicBool,
    space_id: AtomicU64,
    hidden_at: Mutex<Option<Instant>>,
    wake: Sender<()>,
}

impl PopoverState {
    pub fn new(wake: Sender<()>) -> Self {
        Self {
            visible: AtomicBool::new(false),
            space_id: AtomicU64::new(0),
            hidden_at: Mutex::new(None),
            wake,
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible.load(Ordering::Relaxed)
    }

    pub fn mark_shown(&self, space_id: u64) {
        self.visible.store(true, Ordering::Relaxed);
        self.space_id.store(space_id, Ordering::Relaxed);
        let _ = self.wake.send(());
    }

    // Returns true when the active Space differs from the one the popover was last shown on, and adopts the new one.
    pub fn follow_space_change(&self, current_space_id: u64) -> bool {
        self.space_id.swap(current_space_id, Ordering::Relaxed) != current_space_id
    }

    pub fn mark_hidden(&self) {
        self.visible.store(false, Ordering::Relaxed);
        if let Ok(mut hidden_at) = self.hidden_at.lock() {
            *hidden_at = Some(Instant::now());
        }
    }

    // The tray click that closes the popover also fires a blur first; without this guard it would reopen.
    pub fn was_just_hidden(&self) -> bool {
        self.hidden_at
            .lock()
            .ok()
            .and_then(|h| *h)
            .is_some_and(|at| at.elapsed() < REOPEN_GUARD)
    }
}
