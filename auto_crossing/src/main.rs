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

const MAX_SPEED: f64 = 200.00 * (1000.0 / 3600.0);
const SPEEDUP_MAX: f64 = 3.0;
const SPEEDUP_MIN: f64 = -10.0;

fn cars_simulation(road_car1:char, speedup_car1: f64, road_car2:char, speedup_car2: f64) -> bool {

    //CAR1
    let chassis1: i32 = 1111;
    let road1: char = road_car1;
    let _speedup_max1 = SPEEDUP_MAX;
    let _speedup_min1 = SPEEDUP_MIN;
    let max_speed1 = MAX_SPEED;
    let length1 = CAR_LENGTH;
    let mut current_pos1: f64 = -80.0;
    let mut current_speed1: f64 = 0.0;
    let current_speedup1: f64;

    //CAR2
    let chassis2: i32 = 2222;
    let road2: char = road_car2;
    let _speedup_max2 = SPEEDUP_MAX;
    let _speedup_min2 = SPEEDUP_MIN;
    let max_speed2 = MAX_SPEED;
    let lenght2 = CAR_LENGTH;
    let mut current_pos2: f64 = -100.0;
    let mut current_speed2: f64 = 0.0;
    let current_speedup2:f64;

    current_speedup1 = speedup_car1;
    current_speedup2 = speedup_car2;

    println!("==SIMULATION START==");
    let mut tickms:f64;

    loop {
        sleep(Duration::from_millis(100));

        tickms = 100.0;

        //UPDATE CAR1

        let old_position = current_pos1;

        current_pos1 = current_pos1 + current_speed1 * (tickms / 1000.0) + current_speedup1 * (tickms/1000.0) * (tickms / 1000.0) / 2.0;
        current_speed1 = current_speed1 + current_speedup1 * (tickms/1000.0);

        if current_pos1 < old_position {
            current_pos1 = old_position;
        }

        if current_speed1 < 0.0 {
            current_speed1 = 0.0;
        }

        if current_speed1 > max_speed1{
            current_speed1 = max_speed1;
        }

        println!("CAR1 {} in the position {}{}, speed: {}, speed up {}", chassis1, road1, current_pos1, current_speed1, current_speedup1);

        //UPDATE CAR2

        let old_position = current_pos2;

        current_pos2 = current_pos2 + current_speed2 * (tickms / 1000.0) + current_speedup2 * (tickms/1000.0) * (tickms/1000.0) / 2.0;
        current_speed2 = current_speed2 + current_speedup2 * (tickms/1000.0);

        if current_pos2 < old_position {
            current_pos2 = old_position;
        }

        if current_speed2 < 0.0 {
            current_speed2 = 0.0;
        }

        if current_speed2 > max_speed2{
            current_speed2 = max_speed2;
        }

        println!("CAR2 {} in the position {}{}, speed: {}, speed up {}", chassis2, road2, current_pos2, current_speed2, current_speedup2);

        //COLLISION DETECT

        if road1 == 'H' && road2 == 'H'{
          if collision_long(current_pos1, length1, current_pos2){
            println!("Collision on road H");
            return true;
          }  
        }

        if road1 == 'V' && road2 == 'V'{
            if collision_long(current_pos1, length1, current_pos2){
                println!("Collision on road V");
                return  true;
            }
        }


        if road1 != road2 {
            if inside_crossing(current_pos1, length1, road1) && inside_crossing(current_pos2, lenght2, road2){
                println!("Collision inside crossing");
                return true;
            }
        }

        if current_pos1 > length1 + if road1 == 'H' {ROADV_WIDTH} else {ROADH_WIDTH}{
            break;
        }

        if current_pos2 > lenght2 + if road2 == 'V' {ROADV_WIDTH} else {ROADH_WIDTH}{
            break;
        }
    }

    return false;
}

fn collision_long(front_pos:f64, lenght:f64, back_pos:f64) -> bool {
    return front_pos - lenght <= back_pos;
}

fn inside_crossing(position:f64, lenght:f64, road:char) -> bool {
    return position > 0.0 && position <= lenght + if road == 'H' {ROADV_WIDTH} else {ROADV_WIDTH};
}
fn main() {
    println!("===BEGIN===");
    cars_simulation('H', SPEEDUP_MAX/10.0, 'H', SPEEDUP_MAX);
    println!("===END===");

}
