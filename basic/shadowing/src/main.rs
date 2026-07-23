fn main() {
    println!("BEGIN");
    const x:i32 = 5;
    let y = 6;
    let mut z = 7;
    z = z + 1;

    println!("No inicio os valores são: 
    X = {x}, y = {y}, z = {z}");

    {
        const x:i32 = 555;
        let y = 666;
        let mut z = 777;
        z = z + 1;
        println!("Dentro do bloco os valores são:
        X = {x}, y = {y}, z = {z}");
    }

    println!("Depois do bloco os valores são: x={x}, y={y}, z={z}");
}
