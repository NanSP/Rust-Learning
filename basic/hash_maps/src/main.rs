use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        None => println!("P1: get--> {team_name} don't have score",),
        Some(i) => println!("P1: get--> {team_name} has score: {i}"),
    }

    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("P2: get(team_name score--> {score}");

    for (key, value) in &scores {
        println!("P4: {key}: {value}");
    }

    let name_color = String::from("Red");
    let num = 10;
    scores.insert(name_color, num);

    println!("P6: insert--> number {num}");

    scores.insert(String::from("Blue"), 25);
    println!("P7: {:?}", scores);

    let x = scores.entry(String::from("Yellow")).or_insert(50);
    let y = scores.entry(String::from("Blue")).or_insert(50);
    println!("P8: {:?}", scores);

    let text = "Hello world!!";
    for word in text.split_whitespace() {
        let ref_entry = scores.entry(word.to_string()).or_insert(0);

        *ref_entry += 1;
    }
    println!("P9: entry--> {:?}", scores);

    let value = scores.get_mut("hello");
    match value {
        None => (),
        Some(x) => *x += 100,
    }
    println!("P10: get_mut--> {:?}", scores);

    scores.remove("Red");
    println!("P11: remove--> {:?}", scores);
}
