use std::collections::BTreeMap;

fn main() {
    let mut btree_class = BTreeMap::new();

    let names = vec![
        "Alana",
        "Inez",
        "Marinez",
        "Carlos",
        "José Carlos",
        "Hernandes",
        "David",
        "Delmonte",
        "Rosangela",
        "Renan",
    ];
    let grades = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for i in 0..10 {
        btree_class.insert(names[i], grades[i]);

        println!("\n SEARCH ");
        let student = "Carlos";

        match btree_class.get(&student) {
            Some(n) => println!("BTreeMap has {} with grade {}", student, n),
            None => println!("BTreeMap don't have grade for {}", student),
        }

        println!("----Iteration----");
        for (name, grade) in &btree_class {
            println!("BTreeMap iteration--> {} -> {}", name, grade);
        }
    }

    let gap = btree_class.range("H"..="R");

    for (name, grade) in gap {
        println!("Iteration--> {} --> {}", name, grade);
    }

    println!("Hello, world!");
}
