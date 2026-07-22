//const ONE_HOUR_IN_SECONDS: i32 = 1 * 60 * 60; //escopo geral

fn main() {

    const ONE_HOUR_IN_SECONDS: i32 = 1 * 60 * 60;//escopo interno
    println!("Begin");
    let mut x = 5;
    println!("O valor de x é: {x}");

    x = ONE_HOUR_IN_SECONDS;
    println!("O valor de x agora é: {x}");
}
