use std::sync::atomic::{AtomicU64, Ordering};

// Global counter to generate unique incremental IDs for the events
static EVENT_ID_COUNTER: AtomicU64 = AtomicU64::new(0);


pub enum EventStatus {
    Ongoing(u32),
    Completed,
    Scheduled(u32),
}

pub trait EventHandler {
    fn get_name() -> String;
    fn get_description() -> String;
    fn start();
}

pub struct Event {
    id: u64,
    event: Box<dyn EventHandler>,
}

impl Event {
    pub fn new(id: u64, event: Box<dyn EventHandler>) -> Event {
        Event { id: EVENT_ID_COUNTER.fetch_add(1, Ordering::Relaxed), event }
    }
}
