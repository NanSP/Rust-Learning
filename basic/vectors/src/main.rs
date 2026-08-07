fn main() {
    let vi1: Vec<i32> = Vec::new();

    let mut vi2 = Vec::new();
    vi2.push(90);
    vi2.push(91);
    vi2.push(100);
    vi2.push(101);

    let vi3 = vec![1, 2, 3, 4, 5];

    let vss1: Vec<&str> = Vec::new();

    let mut vss2 = Vec::new();
    vss2.push("aaa");
    vss2.push("bbb");
    vss2.push("vvv");

    let vss3 = vec!["ccc", "ddd", "eee"];

    let mut vs1: Vec<String> = Vec::new();

    let mut vs2 = Vec::new();
    vs2.push(String::from("fff"));
    vs2.push(String::from("ggg"));

    let x = vi1[2];
    let y = vss2[2];

    println!("X: {}  Y: {}", x, y);

    let xx = &vi2[2];
    let yy = vss2.get(2);
}
