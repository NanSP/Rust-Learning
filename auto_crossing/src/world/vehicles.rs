const _CAR_WIDTH: f64 = 2.0;
const CAR_LENGTH: f64 = 4.0;

pub const CRUISING_SPEED: f64 = 80.0 * (1000.0 / 3600.0);
pub const MAX_SPEED: f64 = 200.00 * (1000.0 / 3600.0);
pub const SPEEDUP_MAX: f64 = 3.0;
pub const SPEEDUP_MIN: f64 = -10.0;

use super::Road;

pub struct Car {
    pub plate: String,
    road: Road,
    speedup_max: f64,
    speedup_min: f64,
    max_speed: f64,
    pub length: f64,
    pub current_pos: f64,
    pub current_speed: f64,
    current_speedup: f64,
}

impl Car {
    pub fn new(plate: String, road: Road, speedup: f64) -> Self {
        let (res, msg) = Car::valid_plate(&plate);
        assert!(res, " ===INVALID PLATE: {} @{}=== ", msg, plate);

        assert!(
            speedup >= SPEEDUP_MIN && speedup <= SPEEDUP_MAX,
            " ===INVALID SPEED UP: {} @{}=== ",
            plate,
            speedup
        );

        Self {
            plate,
            road: road.clone(),
            speedup_max: SPEEDUP_MAX,
            speedup_min: SPEEDUP_MIN,
            max_speed: MAX_SPEED,
            length: CAR_LENGTH,
            current_pos: match road {
                Road::RoadH => -super::_ROADH_PERIMETER,
                Road::RoadV => -super::_ROADV_PERIMETER,
            },
            current_speed: CRUISING_SPEED,
            current_speedup: speedup,
        }
    }

    fn valid_plate(plate: &str) -> (bool, &str) {
        if !plate.is_ascii() {
            return (false, "----The plate is not ASCII----");
        }

        if plate.len() != 7 {
            return (false, "----The don't have the correct length----");
        }

        let begin = &plate[0..3];
        for x in begin.chars() {
            if !x.is_alphabetic() {
                return (false, "----The plate has no letters on begin----");
            }
        }

        let end = &plate[3..];
        for x in end.chars() {
            if !x.is_ascii_digit() {
                return (false, "----The plate has no digits in the end----");
            }
        }
        (true, "")
    }

    pub fn show(&self) {
        println!(
            "@{} in the position {:?}{}, speed {}, speedup {}",
            self.plate, self.road, self.current_pos, self.current_speed, self.current_speedup
        )
    }

    pub fn tick(&mut self, tickms: f64) {
        let prev_pos = self.current_pos;

        self.current_pos = self.current_pos
            + self.current_speed * (tickms / 1000.0)
            + self.current_speedup * (tickms / 1000.0) / 2.0;

        self.current_speed = self.current_speed + self.current_speedup * (tickms / 1000.0);

        if self.current_pos < prev_pos {
            self.current_pos = prev_pos;
        }

        if self.current_speed < 0.0 {
            self.current_speed = 0.0;
        }

        if self.current_speed > self.max_speed {
            self.current_speed = self.max_speed;
        }
    }
}
