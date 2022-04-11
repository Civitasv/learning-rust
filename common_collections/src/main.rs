use std::collections::HashMap;

fn main() {
    iterate_over_vector();
    string_use();
}

fn iterate_over_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

fn string_use() {
    let mut s = String::new();
    let data = String::from("initial contents");

    let mut s = String::from("initial contents");
    s.push_str(&data);

    let s2 = s + &data;
    let s3: &str = &s2[0..3];
    let hello: &str = "Здравствуйте";
    let s: &str = &hello[0..4];
    println!("{}", s);
    for c in hello.chars() {
        println!("{}", c);
    }
}

fn hashmap() {
    let mut scores: HashMap<&str, &str> = HashMap::new();
    scores.insert("SSS", "INDEX");

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
}
