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
    cars_road_h: Vec<Car>,
    cars_road_v: Vec<Car>,
    created_cars: i32,
}

impl Traffic {
    pub fn new() -> Self {
        Self {
            cars_road_h: Vec::new(),
            cars_road_v: Vec::new(),
            created_cars: 0,
        }
    }

    pub fn collision_detect(&self) -> Option<&str> {
        if self.cars_road_h.len() >= 2 {
            for i in 0..self.cars_road_h.len() - 1 {
                let back_i = self.cars_road_h.get(i).unwrap().current_pos
                    - self.cars_road_h.get(i).unwrap().length;
                if back_i <= self.cars_road_h.get(i + 1).unwrap().current_pos {
                    return Some("Collision on ROAD H");
                }
            }
        }

        if self.cars_road_h.len() >= 2 {
            for i in 0..self.cars_road_v.len() - 1 {
                let back_i = self.cars_road_v.get(i).unwrap().current_pos
                    - self.cars_road_v.get(i).unwrap().length;
                if back_i <= self.cars_road_v.get(i + 1).unwrap().current_pos {
                    return Some("Collision on ROAD V");
                }
            }
        }

        let mut crossing_h = false;
        let mut crossing_v = false;

        for car in &self.cars_road_h {
            crossing_h = crossing_h
                || (car.current_pos > 0.0 && car.current_pos < 0.0 + ROADV_WIDTH + car.length)
        }

        for car in &self.cars_road_v {
            crossing_v = crossing_v
                || (car.current_pos > 0.0 && car.current_pos < 0.0 + ROADV_WIDTH + car.length)
        }

        if crossing_h && crossing_v {
            return Some("===CROSSING COLLISION===");
        }
        None
    }

    fn arrive_speed(&self, road: &Road) -> f64 {
        match road {
            Road::RoadH => {
                if self.cars_road_h.len() == 0 {
                    return vehicles::CRUISING_SPEED;
                } else {
                    let last_car = self.cars_road_h.last().unwrap();
                    let distance = _ROADH_PERIMETER + last_car.current_pos - last_car.length;

                    if distance < 0.5 {
                        return 0.0;
                    } else if distance < 4.0 {
                        return vehicles::CRUISING_SPEED.min(last_car.current_speed);
                    } else {
                        vehicles::CRUISING_SPEED
                    }
                }
            }
            Road::RoadV => {
                if self.cars_road_v.len() == 0 {
                    return vehicles::CRUISING_SPEED;
                } else {
                    let last_car = self.cars_road_v.last().unwrap();
                    let distance = _ROADH_PERIMETER + last_car.current_pos - last_car.length;

                    if distance < 0.5 {
                        return 0.0;
                    } else if distance < 4.0 {
                        return vehicles::CRUISING_SPEED.min(last_car.current_speed);
                    } else {
                        vehicles::CRUISING_SPEED
                    }
                }
            }
        }
    }

    pub fn arrive_car(&mut self, road: Road) -> bool {
        let spd = self.arrive_speed(&road);
        let speed_up: f64;

        if spd == 0.0 {
            return false;
        }

        let mut new_plate = String::from("CCC");
        new_plate.push_str(&format!("{:04}", self.created_cars));
        self.created_cars += 1;

        let new_car = Car::new(new_plate, road.clone(), 0.0);

        match road {
            Road::RoadH => {
                self.cars_road_h.push(new_car);
            }
            Road::RoadV => {
                self.cars_road_v.push(new_car);
            }
        }
        true
    }

    pub fn tick(&mut self, tickms: f64) {
        print!("-TRAFFTIC.TICK-");

        for car in &mut self.cars_road_h {
            car.tick(tickms);
        }

        for car in &mut self.cars_road_v {
            car.tick(tickms);
        }

        if self.cars_road_h.len() > 0 {
            let older_h = &self.cars_road_h.get(0).unwrap();
            if older_h.current_pos > 0.0 + older_h.length + ROADV_WIDTH {
                println!("@{} leave the road H", older_h.plate);
                self.cars_road_h.remove(0);
            }
        }

        if self.cars_road_v.len() > 0 {
            let older_v = &self.cars_road_v.get(0).unwrap();
            if older_v.current_pos > 0.0 + older_v.length + ROADH_WIDTH {
                println!("@{} leave the road V", older_v.plate);
                self.cars_road_v.remove(0);
            }
        }
    }

    pub fn show_roads(&self) {
        println!("-----CARS ON ROAD H-----");

        for car in &self.cars_road_h {
            car.show();
        }

        println!("-----CARS ON ROAD V-----");
        for car in &self.cars_road_v {
            car.show();
        }
    }

    pub fn empty(&self) -> bool {
        self.cars_road_h.len() == 0 && self.cars_road_v.len() == 0
    }
}
