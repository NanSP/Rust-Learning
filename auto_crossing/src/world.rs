mod vehicles;

use vehicles::Car;

const _ROADH_MARGIN: f64 = 15.0;
const _ROADV_MARGIN: f64 = 15.0;

const ROADH_WIDTH: f64 = 4.0;
const ROADV_WIDTH: f64 = 4.0;

const _ROADH_PERIMETER: f64 = 150.0;
const _ROADV_PERIMETER: f64 = 150.0;

const ROAD_MAX_CARS: usize = 4;

#[derive(Debug, Clone)]
pub enum Road {
    RoadH,
    RoadV,
}

pub struct Traffic {
    num_created_cars_h: usize,
    num_leave_cars_h: usize,
    cars_road_h: [Car; 4],
    num_created_cars_v: usize,
    num_leave_cars_v: usize,
    cars_road_v: [Car; 4],
}

impl Traffic {
    pub fn new() -> Self {
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

    pub fn collision_detect(&self) -> Option<&str> {
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

    pub fn arrive_car(&mut self, road: Road) -> bool {
        let already_has = match road {
            Road::RoadH => self.num_created_cars_h,
            Road::RoadV => self.num_created_cars_v,
        };

        if already_has == ROAD_MAX_CARS {
            return false;
        }

        let mut new_plate = String::from("EEE");
        new_plate.push_str(&format!("{:04}", already_has));

        let new_car = Car::new(new_plate, road.clone(), vehicles::SPEEDUP_MAX);

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

    pub fn tick(&mut self, tickms: f64) {
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

    pub fn show_roads(&self) {
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

    pub fn empty(&self) -> bool {
        self.num_created_cars_h == self.num_leave_cars_h
            && self.num_created_cars_v == self.num_leave_cars_v
    }
}
