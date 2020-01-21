fn main() {
    copy_s();
    ownership_and_callling();
    return_and_ownership();
}

fn copy_s() {
    let mut s1 = String::from("hello");
    let s2 = s1.clone();

    s1.push_str(" world");

    println!("1: {}!", s1);
    println!("2: {}!", s2);
}

fn ownership_and_callling() {
    let s = String::from("hello");

    take_ownership(s);
    // println!("after take_ownership: {}", s); // fails: error[E0382]: borrow of moved value: `s`

    let i: i32 = 5;
    make_copy(i);
    println!("after make_copy: {}", i);
}

fn take_ownership(s_own: String) {
    println!("take_ownership: {}", s_own);
} // s_own is freed

fn make_copy(i_copy: i32) {
    println!("make_copy: {}", i_copy);
}

fn return_and_ownership() {
    let s1 = gives_ownership();
    let s2 = String::from("Hello Old!");
    let s3 = takes_and_gives_back(s2);

    println!("return_and_ownership: s1: {}", s1);
    // println!("return_and_ownership: s2: {}", s2); // does not compile, ownership moved
    println!("return_and_ownership: s3: {}", s3);

    let (len, s4) = calc_len(s3);
    println!("return_and_ownership: s4: {}, len: {}", s4, len);

    let len = calc_len_ref(&s4);
    println!("return_and_ownership: s4 ref: {}, len: {}", s4, len);
}

fn gives_ownership() -> String {
    let s = String::from("New Hello");
    s
}

fn takes_and_gives_back(s_in: String) -> String {
    s_in
}

fn calc_len(s: String) -> (usize, String) {
    // In place, order is important:
    // (s.len(), s)
    // Or with a variable:
    let len = s.len();
    (len, s)
}

fn calc_len_ref(s: &String) -> usize {
    s.len()
}

