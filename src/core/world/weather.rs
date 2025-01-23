#[derive(Copy, Clone, Debug)]
pub enum Weather {
    Clear,
    Cloudy,
    PartiallyCloudy,
    Rainy,
}

#[derive(Copy, Clone, Debug)]
pub enum DayCycle {
    Sunrise,
    Day,
    Sunset,
    Night,
}
