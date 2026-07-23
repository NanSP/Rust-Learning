fn other_function(){
    println!("Other function");
}

fn other_function_with_param(x: i32){
    println!("Other function received {x}.");
}

fn print_labeled_measurement(value: f64, unit: char){
    println!("The measurement is: {value}{unit}.");
}

fn sum(x: i32, y: i32) -> i32{
    x+y
}

fn sumret(x:i32, y:i32) -> i32{
    return x+y;
}

fn main() {
    other_function();
    other_function_with_param(55);
    print_labeled_measurement( 123.4, 'm');

    let x = 999;
    print_labeled_measurement( x as f64, 'm');

    let xy = sum(3 , 4);
    println!("The sum of {} and {} is: ", xy, sumret(3,4));

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {:?}", y);

    let y = {
        let x = 3;
        x + 1;
    };
    println!("The value of y is: {:?}", y);

}
