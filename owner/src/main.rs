fn main() {
    copy_s();
    ownership_and_calling();
    return_and_ownership();
    reference_mutable();
    slice_first_word();
}

fn copy_s() {
    let mut s1 = String::from("hello");
    let s2 = s1.clone();

    s1.push_str(" world");

    println!("1: {}!", s1);
    println!("2: {}!", s2);
}

fn ownership_and_calling() {
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

fn reference_mutable() {
    let mut s = String::from("Ref Hello");
    println!("reference_mutable: s: {}", s);
    change_hello(&mut s);
    println!("reference_mutable: s: {}", s);

    let r1 = &mut s;
    // let r2 = &mut s; // fails
    println!("reference_mutable: r1: {}", r1);
    // println!("reference_mutable: {}", r2);

    let r1 = &s;
    // let r2 = &mut s; // fails
    println!("reference_mutable: r1: {}", r1);
    // println!("reference_mutable: {}", r2);

    let r1 = &s;
    let r2 = &s;
    println!("reference_mutable: r1: {}", r1);
    println!("reference_mutable: r2: {}", r2);

    let mut r3 = &mut s;
    change_hello(&mut r3);
    println!("reference_mutable: r3: {}", r3);

    let dangle_s = dangle_ref();
    println!("reference_mutable: dangle_s: {}", dangle_s);
}

fn change_hello(s: &mut String) {
    s.push_str(" World");
}

// Fails:
// fn dangle_ref() -> &String {
//     let s = String::from("Dangle Hello");
//     &s
// }

fn dangle_ref() -> String {
    let s = String::from("Dangle Hello");
    s
}

fn slice_first_word() {
    let s: String = String::from("First Word Counts");
    println!("slice_first_word: {}/{}", first_word(&s), s.len());
    println!("slice_first_word: man: {}/{}", first_word_manual(&s), s.len());
    println!("slice_first_word: sli: {}/{}", first_word_slice(&s).len(), s.len());

    let s: String = String::from("First_Word_Counts");
    println!("slice_first_word: {}/{}", first_word(&s), s.len());
    println!("slice_first_word: man: {}/{}", first_word_manual(&s), s.len());
    println!("slice_first_word: sli: {}/{}", first_word_slice(&s).len(), s.len());

    let mut s = String::from("First Word Counts");
    s.push_str(" Really");
    let s_first = first_word_slice(&s);
    // s.clear(); // fails
    println!("slice_first_word: s_first: {}", s_first);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_manual(s: &String) -> usize {
    let bytes = s.as_bytes();
    let mut i: usize = 0;

    for &item in bytes.iter() {
        if item == b' ' {
            return i;
        }
        i += 1;
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
