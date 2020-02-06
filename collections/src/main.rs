use std::collections::HashMap;

fn main() {
    borrow_one_more_time();
    vectors_borrow();
    vectors_iterate();
    walking_in_strings();
    indexing_in_strings();
    hashmaps();
}

fn borrow_one_more_time() {
    let mut x: i32 = 5;
    x += 1;

    // One owner but borrowed, one mutable borrow
    let borrow = &mut x;
    *borrow += 1;
    println!("borrow: {}", borrow);
    *borrow += 1;
    // Older rust would complain but newer accepts it: borrow goes early out of scope (even before end of block)
    // so borrowing ends.
    println!("x: {}", x);

    // Again but...
    let borrow = &mut x;
    // Try to modify through ownership reference
    // x += 1; // fails: x was borrowed mutably
    *borrow += 1;
    println!("borrow: {}", borrow);
    *borrow += 1;
    println!("x: {}", x);

    // Again but...
    let borrow = &mut x;
    *borrow += 1;
    println!("borrow: {}", borrow);
    *borrow += 1;
    // Try to borrow x one more time, immutable to println!
    // println!("x: {}", x); // fails: x was borrowed mutably
    println!("borrow: {}", borrow);
}

fn vectors_borrow() {
    let v: Vec<i32> = Vec::new();
    println!("Vector: {:?}", v);

    let v = vec![1, 2, 3];
    println!("Vector: {:?}", v);
    println!("Vector reset");

    let mut v: Vec<i32> = Vec::new();
    v.push(6);
    v.push(5);
    println!("Vector: {:?}", v);

    let mut i = 1;
    let velem: &mut i32 = &mut v[i];
    println!("Vector[{}]: {}", i, velem);
    *velem = 3; // Borrow ends
    v.push(55);
    i += 1;
    println!("Vector: {:?}", v);

    let velem: &mut i32 = &mut v[i];
    println!("Vector[{}]: {}", i, velem);
    // v.push(55); // fails: v is borrowed mutably
    // i += 1;
    *velem = 3;
    println!("Vector: {:?}", v);

    let velem = &v[i]; // immutable borrow
    println!("Vector[{}]: {}", i, velem); // immutable borrow
    println!("Vector: {:?}", v); // immutable borrow
    let velem = &mut v[i]; // mutable borrow - still okay
    // println!("Vector: {:?}", v); // fails: immutable borrow while it is already borrowed mutably
    *velem = 3;
    println!("Vector[{}]: {}", i, velem);
    v.push(555);
    i += 1;
    println!("Vector: {:?}", v);

    let velem = &v[i];
    println!("Vector[{}]: {}", i, velem);
    let velem = &mut v[i];
    println!("Vector[{}]: {}", i, velem);
    // v.push(555); // fails: immutable borrow while it is already borrowed mutably
    // i += 1;
    *velem = 3;
    println!("Vector: {:?}", v);

    let velem = &v[i];
    // v.push(555); // fails: immutable borrow while it is already borrowed mutably
    println!("Vector[{}]: {}", i, velem);

    i += 1;
    // let mut velem: &mut i32 = &mut v[i]; // will panic
    // let velem = &v[i]; // will panic
    match v.get(i) {
        Some(elem) => println!("Vector[{}]: {}", i, elem),
        None => println!("Vector[{}]: {}", i, "None"),
    }
}

fn vectors_iterate() {
    let mut v = vec![10, 50, 32];

    // for i in v { // will fail: ownership moved
    for i in &v {
        println!("Vector[]: {}", i);
    }
    println!("Vector: {:?}", v);

    for i in &mut v {
        *i += 5;
    }
    println!("Vector: {:?}", v);
}

fn walking_in_strings() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("String: {}", s);

    let mut s = String::from("źdźbło");
    s.push('ł');
    println!("String: {}", s);

    let s1 = String::from("Hello ");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    // println!("s1: {}", s1); // fails
    println!("s2: {}", s2);
    println!("s3: {}", s3);

    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = format!("{} {}", s1, s2);
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
}

fn indexing_in_strings() {
    let hello = "źdźbło";
    let c = "";
    // let c = &hello[0]; // fails
    // let c = &hello[0..1]; // panics: thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'ź' (bytes 0..2) of `źdźbło`', src/libcore/str/mod.rs:2068:5

    println!("hello[]: {} = {}", hello, c);

    for c in hello.chars() {
        println!("[]: {}", c);
    }

    for b in hello.bytes() {
        println!("[]: {:#x}", b);
    }
}

fn hashmaps() {
    let mut scores = HashMap::new();
    let team1 = String::from("Blue");
    let team2 = String::from("Red");
    scores.insert(team1, 10);
    scores.insert(team2, 15);
    println!("Hashmap: {:?}", scores);
    // println!("Teams: {}, {}", team1, team2); // fails

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Score for {} is {:?}", team_name, score);
    if let Some(score) = score { // This works but it is not really readable (scope of "score")
        println!("Score for {} is {:?}", team_name, score);
    }
    if let Some(v) = score {
        println!("Score for {} is {:?}", team_name, v);
    }
    for (key, value) in &scores {
        println!("Score[{}]: {}", key, value);
    }
    scores.insert(String::from("Blue"), 30);
    println!("Overwrite Hashmap: {:?}", scores);
    scores.entry(String::from("Blue")).or_insert(40);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("Or Insert Hashmap: {:?}", scores);

    // let blue_score = scores.entry(team_name).or_insert(45); // Works but not good because moves the ownership of team_name
    let blue_score = scores.entry(String::from(&team_name)).or_insert(45);
    println!("Score for {} is {}", team_name, blue_score);
    *blue_score += 13;
    println!("Score for {} is {}", team_name, blue_score);
    println!("After modifying Hashmap: {:?}", scores);

    let mut scores: HashMap<String, _> = HashMap::new();
    let team1 = String::from("Blue");
    let team2 = String::from("Red");
    scores.insert(team1, 10);
    scores.insert(team2, 15);
    println!("Hashmap: {:?}", scores);
    // println!("Teams: {}, {}", team1, team2); // fails

    let mut scores: HashMap<&String, _> = HashMap::new();
    let mut team1 = String::from("Blue");
    let team2 = String::from("Red");
    scores.insert(&team1, 10);
    scores.insert(&team2, 15);
    println!("Teams: {}, {}", team1, team2);
    println!("Hashmap: {:?}", scores);
    // team1.push_str("NotReally"); // fails
    println!("Teams: {}, {}", team1, team2);
    println!("Hashmap: {:?}", scores);

    let teams = vec![String::from("Yellow"), String::from("Black")];
    let initial_scores = vec![2000, 5];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("Hashmap: {:?}", scores);
    println!("teams {:?}", teams);

    let scores: HashMap<&String, &u16> = teams.iter().zip(initial_scores.iter()).collect();
    println!("Hashmap: {:?}", scores);
}
