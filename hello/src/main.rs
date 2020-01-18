fn main() {
    let tup: (i32, u32, f32) = (2_140_000_000, 4_000_000_001, 6001.41);
    let (x, y, z) = tup;
    println!("Hello, world! {}, {}, {}", x, y, z);
    println!("Tuple: {}, {}, {}", tup.0, tup.1, tup.2);

    print_arrays(1);
    println!("let in let: {}", let_in_let());
}

fn print_arrays(index: usize) {
    let arr = [5, 6, 7];
    println!("Arr: {}, {}, {}", arr[0], arr[1], arr[2]);

    println!("Arr[{}]: {}", index, arr[index]);

    let index = index + 1;
    println!("Arr[{}]: {}", index, arr[index]);
}

fn let_in_let() -> i32 {
    let x: i32 = 5;
    let y: i32 = {
        let x = 3;
        x + 10
    };

    println!("x: {}, y: {}", x, y);
    y
}
