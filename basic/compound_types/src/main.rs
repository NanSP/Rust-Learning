fn main() {
    //TUPLA
    let tup1: (i32, f64, bool) = (500, 6.4, true);
    let tup2: (i32, f64, bool) = (500, 6.4, true);

    println!("Minha tupla tem: {:?}", tup2);

    let(x1, y1, z1) = tup2;
    println!("Minha tupla tem: {x1} {y1} {z1}");

    println!("Minha tuple tem: {:?} {:?} {:?}", tup1.0, tup1.1, tup1.2);

    println!("Tupla vazia: {:?}", ());

    //ARRAYS
    let array1 = [1,2,3,4,5];
    let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];

    //tipagem e tamanho declarados
    let array2: [i32; 5] = [1,2,3,4,5];

    let array3: [i32; 5] = [3;5];//5 elementos com valor 3
    let array4: [i32; 2] = [3,5];

    println!("Array3: {:?}",array3);
    println!("Array4: {:?}",array4);

    println!("Elemento 2 do array meses é: {:?}", meses[2]);
}
