fn main() {
    let tup: (i32, u32, f32) = (2_140_000_000, 4_000_000_001, 6001.41);
    let (x, y, z) = tup;
    println!("Hello, world! {}, {}, {}", x, y, z);
    println!("Tuple: {}, {}, {}", tup.0, tup.1, tup.2);

    print_arrays(1);
    println!("let in let: {}", let_in_let());
    println!("fn(5): {}", dub(5));

    if dub(1) > 1 {
        println!("Branch true");
    }
    println!("Half {}: {},", 5, odd_half(5));
    println!("Half {}: {},", 6, odd_half(6));

    looper(5);
    looper(6);

    while_looper(3);
    while_looper(5);

    array_loop_while();
    array_loop_for();
}

fn print_arrays(index: usize) {
    let arr = [5, 6, 7];
    println!("Arr: {}, {}, {}", arr[0], arr[1], arr[2]);

    println!("Arr[{}]: {}", index, arr[index]);

    let index = index + 1;
    println!("Arr[{}]: {}", index, arr[index]);
}

fn dub(x: i32) -> i32 {
    // Oh really...
    x*2
}

fn let_in_let() -> i32 {
    let x: i32 = 5;
    let y: i32 = {
        let x = 3;
        x + 10
    };

    println!("x: {}, y: {}", x, y);
    return y;
}

fn odd_half(x: i32) -> i32 {
    let half = if x % 2 == 0 {
        x / 2
    } else {
        x
    };
    half
}

fn looper(n: usize) {
    let mut i = 0;
    let result = loop {
        i += 1;
        if i >= n {
            break i*2;
        }
    };

    println!("Loop result for {}: {}", n, result);
}

fn while_looper(n: usize) {
    let mut i = 0;
    let mut sum = 0;

    while i < n {
        sum += i;
        i += 1;
    }
    println!("While result for {}: {}", n, sum);
}

fn array_loop_while() {
    let arr = [10, 20, 30, 40, 50];
    let mut i = 0;

    while i < arr.len() {
        println!("arr[{}]: {}", i, arr[i]);
        i += 1;
    }
}

fn array_loop_for() {
    let arr = [10, 20, 30, 40, 50];

    for elem in arr.iter() {
        println!("arr[]: {}", elem);
    }
}
