use std::thread::sleep;
use std::time::Duration;

const _ROADH_MARGIN: f64 = 15.0;
const _ROADV_MARGIN: f64 = 15.0;

const ROADH_WIDTH: f64 = 4.0;
const ROADV_WIDTH: f64 = 4.0;

const _ROADH_PERIMETER: f64 = 150.0;
const _ROADV_PERIMETER: f64 = 150.0;

const _CAR_WIDTH: f64 = 2.0;
const CAR_LENGTH: f64 = 4.0;

const ROAD_MAX_CARS: usize = 4;

const CRUISING_SPEED: f64 = 80.0 * (1000.0 / 3600.0);
const MAX_SPEED: f64 = 200.00 * (1000.0 / 3600.0);
const SPEEDUP_MAX: f64 = 3.0;
const SPEEDUP_MIN: f64 = -10.0;

#[derive(Debug, Clone)]
enum Road {
    RoadH,
    RoadV,
}

struct Car {
    plate: String,
    road: Road,
    speedup_max: f64,
    speedup_min: f64,
    max_speed: f64,
    length: f64,
    current_pos: f64,
    current_speed: f64,
    current_speedup: f64,
}

impl Car {
    fn new(plate: String, road: Road, speedup: f64) -> Self {
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
                Road::RoadH => -_ROADH_PERIMETER,
                Road::RoadV => -_ROADV_PERIMETER,
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

    fn show(&self) {
        println!(
            "@{} in the position {:?}{}, speed {}, speedup {}",
            self.plate, self.road, self.current_pos, self.current_speed, self.current_speedup
        )
    }

    fn tick(&mut self, tickms: f64) {
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

struct Traffic {
    num_created_cars_h: usize,
    num_leave_cars_h: usize,
    cars_road_h: [Car; 4],
    num_created_cars_v: usize,
    num_leave_cars_v: usize,
    cars_road_v: [Car; 4],
}

impl Traffic {
    fn new() -> Self {
        Self {
            num_created_cars_h: 0,
            num_leave_cars_h: 0,
            cars_road_h: [
                Car::new(String::from("AAA0000"), Road::RoadH, 0.0),
                Car::new(String::from("BBB0000"), Road::RoadH, 0.0),
                Car::new(String::from("CCC0000"), Road::RoadH, 0.0),
                Car::new(String::from("DDD0000"), Road::RoadH, 0.0),
            ],
            num_created_cars_v: 0,
            num_leave_cars_v: 0,
            cars_road_v: [
                Car::new(String::from("AAA1111"), Road::RoadV, 0.0),
                Car::new(String::from("BBB2222"), Road::RoadV, 0.0),
                Car::new(String::from("CCC3333"), Road::RoadV, 0.0),
                Car::new(String::from("DDD4444"), Road::RoadV, 0.0),
            ],
        }
    }

    fn collision_detect(&self) -> Option<&str> {
        let mut i: usize = self.num_leave_cars_h + 1;
        while i < self.num_created_cars_h {
            if self.cars_road_h[i - 1].current_pos - self.cars_road_h[i - 1].length
                <= self.cars_road_h[i].current_pos
            {
                return Some("===Collision on road H, cars {}===");
            }
            i += 1;
        }

        i = self.num_leave_cars_v + 1;
        while i < self.num_created_cars_v {
            if self.cars_road_v[i - 1].current_pos - self.cars_road_v[i - 1].length
                <= self.cars_road_v[i].current_pos
            {
                return Some("===Collision on road V, cars {}===");
            }
            i += 1;
        }

        let mut crossing_h = false;
        let mut crossing_v = false;
        i = self.num_leave_cars_h;
        while i < self.num_created_cars_h {
            crossing_h = crossing_h
                || (self.cars_road_h[i].current_pos > 0.0
                    && self.cars_road_h[i].current_pos
                        < 0.0 + ROADV_WIDTH + self.cars_road_h[i].length);

            i += 1;
        }

        i = self.num_leave_cars_v;
        while i < self.num_created_cars_v {
            crossing_v = crossing_v
                || (self.cars_road_v[i].current_pos > 0.0
                    && self.cars_road_v[i].current_pos
                        < 0.0 + ROADH_WIDTH + self.cars_road_v[i].length);

            i += 1;
        }
        if crossing_h && crossing_v {
            return Some("===CROSSING COLLISION===");
        }
        None
    }

    fn arrive_car(&mut self, road: Road, speedup: f64) -> bool {
        let already_has = match road {
            Road::RoadH => self.num_created_cars_h,
            Road::RoadV => self.num_created_cars_v,
        };

        if already_has == ROAD_MAX_CARS {
            return false;
        }

        let mut new_plate = String::from("EEE");
        new_plate.push_str(&format!("{:04}", already_has));

        let new_car = Car::new(new_plate, road.clone(), speedup);

        match road {
            Road::RoadH => {
                self.cars_road_h[self.num_created_cars_h] = new_car;
                self.num_created_cars_h += 1;
            }
            Road::RoadV => {
                self.cars_road_v[self.num_created_cars_v] = new_car;
                self.num_created_cars_v += 1;
            }
        }

        true
    }

    fn tick(&mut self, tickms: f64) {
        print!("-TRAFFTIC.TICK-");

        let mut i;

        i = self.num_leave_cars_h;
        while i < self.num_created_cars_h {
            self.cars_road_h[i].tick(tickms);
            i += 1;
        }

        i = self.num_leave_cars_v;
        while i < self.num_created_cars_v {
            self.cars_road_v[i].tick(tickms);
            i += 1;
        }

        if self.num_leave_cars_h < self.num_created_cars_h {
            let older_h = &self.cars_road_h[self.num_leave_cars_h];
            if older_h.current_pos > older_h.length + ROADV_WIDTH + _ROADH_MARGIN {
                println!("=== @{} LEAVE THE ROAD H===", older_h.plate);
                self.num_leave_cars_h += 1;
            }
        }

        if self.num_leave_cars_v < self.num_created_cars_v {
            let older_v = &self.cars_road_v[self.num_leave_cars_v];
            if older_v.current_pos > older_v.length + ROADH_WIDTH + _ROADV_MARGIN {
                println!("=== @{} LEAVE THE ROAD V===", older_v.plate);
                self.num_leave_cars_v += 1;
            }
        }
    }

    fn show_roads(&self) {
        println!("-----CARS ON ROAD H-----");

        let mut i = self.num_leave_cars_h;
        while i < self.num_created_cars_h {
            self.cars_road_h[i].show();
            i += 1;
        }

        println!("-----CARS ON ROAD V-----");

        let mut i = self.num_leave_cars_v;
        while i < self.num_created_cars_v {
            self.cars_road_v[i].show();
            i += 1;
        }
    }
}

fn cars_simulation() {
    const TIME_BETWEEN_ARRIVALS: f64 = 3000.0;

    let mut traffic = Traffic::new();

    traffic.arrive_car(Road::RoadH, SPEEDUP_MAX);

    traffic.arrive_car(Road::RoadV, SPEEDUP_MAX);

    let mut time_until_next_arrival = TIME_BETWEEN_ARRIVALS;

    println!("____CARS SIMULATION____");
    let mut tickms: f64;

    loop {
        tickms = 100.0;

        sleep(Duration::from_millis(tickms.round() as u64));
        traffic.tick(tickms);

        traffic.show_roads();

        match traffic.collision_detect() {
            Some(m) => panic!("Collision detected: {}", m),
            None => {}
        }

        if traffic.num_created_cars_h == traffic.num_leave_cars_h
            && traffic.num_created_cars_v == traffic.num_leave_cars_v
        {
            break;
        }

        time_until_next_arrival -= tickms;

        if time_until_next_arrival <= 0.0 {
            let speedup: f64 = 0.0;
            assert!(
                traffic.arrive_car(Road::RoadH, SPEEDUP_MAX),
                "FAIL TO ARRIVE A CAR ON ROAD H"
            );
            assert!(
                traffic.arrive_car(Road::RoadV, SPEEDUP_MAX),
                "FAIL TO ARRIVE A CAR ON ROAD V"
            );
            time_until_next_arrival += TIME_BETWEEN_ARRIVALS;
        }
    }
}

fn main() {
    println!("===BEGIN===");
    cars_simulation();
    println!("===END===");
}
