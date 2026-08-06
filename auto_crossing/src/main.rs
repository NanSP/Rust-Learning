use std::thread::sleep;
use std::time::Duration;

mod world;

use world::Road;
use world::Traffic;

fn cars_simulation() {
    const TIME_BETWEEN_ARRIVALS: f64 = 3000.0;

    let mut traffic = Traffic::new();

    traffic.arrive_car(Road::RoadH);

    traffic.arrive_car(Road::RoadV);

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

        if traffic.empty() {
            break;
        }

        time_until_next_arrival -= tickms;

        if time_until_next_arrival <= 0.0 {
            assert!(
                traffic.arrive_car(Road::RoadH),
                "FAIL TO ARRIVE A CAR ON ROAD H"
            );
            assert!(
                traffic.arrive_car(Road::RoadV),
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
