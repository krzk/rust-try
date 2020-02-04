fn main() {
    borrow_one_more_time();
    vectors_borrow();
    vectors_iterate();
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
