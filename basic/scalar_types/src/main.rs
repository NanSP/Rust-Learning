const VELOCIDADE_MAXIMA: f64 = 200.0;

fn main() {
    let chassi: i32 = 123456;
    let acel_max: f64 = 3.0;
    let acel_min: f64 = 10.0;
    let vel_max: f32 = VELOCIDADE_MAXIMA as f32;
    let comprimento: i32 = 4;
    let posicao_atual: f64 = -100.00;
    let vel_atual: f64 = 0.00;
    let acel_atual: f64 = 0.00;

    let sum = posicao_atual + 10.0;

    let difference = vel_atual - 4.3;

    let product = comprimento * 2;
    
    let quotient = acel_atual / 2.0;
    let floored = 2 / 3;

    let remainder = 43 % 5;

    //tranformação de tipos
    //let xx :f64 = 123 as f64;
    let xx :f64 = 123.55;

    let _yyy = xx + 88f64;

    println!("trunc{}, round{}, ceil {}, floor {}",
                xx.trunc(), xx.round(), xx.ceil(), xx.floor());

    println!("Numéricos");
}
