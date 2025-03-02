use std::collections::VecDeque;

pub struct EventCoordinator {
    updating_events: VecDeque<Event>,
    new_events: VecDeque<Event>,
}

impl EventCoordinator {}
