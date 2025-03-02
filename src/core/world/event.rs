use std::sync::atomic::{AtomicU64, Ordering};

// Global counter to generate unique incremental IDs for the events
static EVENT_ID_COUNTER: AtomicU64 = AtomicU64::new(0);


pub enum EventStatus {
    Ongoing,
    Completed,
    Timed(u32),
    Scheduled(u32),
}

pub trait EventHandler {
    fn get_name() -> String;
    fn get_description() -> String;
    fn start() -> EventStatus;
    fn update() -> EventStatus;
    fn terminate() -> EventStatus;
}

pub struct Event {
    id: u64,
    event: Box<dyn EventHandler>,
}

impl Event {
    pub fn new(id: u64, event: Box<dyn EventHandler>) -> Event {
        Event { id: EVENT_ID_COUNTER.fetch_add(1, Ordering::Relaxed), event }
    }

    pub fn event(&self) -> &dyn EventHandler {
        self.event.as_ref()
    }

    pub fn event_mut(&mut self) -> &mut dyn EventHandler {
        self.event.as_mut()
    }
}
