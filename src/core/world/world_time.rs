use crate::core::world::weather::{Weather, DayCycle};

const WORLD_TIME_MINUTE_STEP: u8 = 5;
const WORLD_TIME_HOUR_STEP: u8 = 1;
const WORLD_MINUTES_IN_A_HOUR: u8 = 60;
const WORLD_HOURS_IN_A_DAY: u8 = 24;

pub struct WorldTime {
    time: (u8, u8),
    day_count: u32,
    weather: Weather,
    day_cycle: DayCycle,
}

impl WorldTime {
    pub fn new() -> Self {
        Self {
            time: (9, 0),
            day_count: 0,
            weather: Weather::Clear,
            day_cycle: DayCycle::Day,
        }
    }

    pub fn set_time(&mut self, hour: u8, minute: u8) {
        self.time = (hour, minute);
    }

    pub fn hour_step(&mut self) {
        self.time.0 += WORLD_TIME_HOUR_STEP;
        if self.time.0 >= WORLD_HOURS_IN_A_DAY {
            self.time.0 -= WORLD_TIME_HOUR_STEP;
            self.day_count += 1;
        }
    }

    pub fn minute_step(&mut self) {
        self.time.1 += WORLD_TIME_MINUTE_STEP;
        if self.time.1 >= WORLD_MINUTES_IN_A_HOUR {
            self.hour_step();
            self.time.1 -= WORLD_MINUTES_IN_A_HOUR;
        }
    }

    pub fn advance_time(&mut self) {
        self.minute_step();
    }

    pub fn get_time(&self) -> (u8, u8) {
        self.time
    }

    pub fn get_hour_time(&self) -> u8 {
        self.time.0
    }

    pub fn get_minute_time(&self) -> u8 {
        self.time.1
    }

    pub fn day_cycle_step(&mut self) {
        match self.day_cycle {
            DayCycle::Sunrise => { self.day_cycle = DayCycle::Day; },
            DayCycle::Day => { self.day_cycle = DayCycle::Sunset; },
            DayCycle::Sunset => { self.day_cycle = DayCycle::Night; },
            DayCycle::Night => { self.day_cycle = DayCycle::Sunrise; },
        }
    }

    pub fn set_day_cycle(&mut self, day_cycle: DayCycle) {
        self.day_cycle = day_cycle;
    }

    pub fn get_day_cycle(&self) -> DayCycle {
        self.day_cycle
    }

    pub fn set_weather(&mut self, weather: Weather) {
        self.weather = weather;
    }

    pub fn get_weather(&self) -> Weather {
        self.weather
    }

    pub fn get_day_count(&self) -> u32 {
        self.day_count
    }
}
