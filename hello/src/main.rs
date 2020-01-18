fn main() {
    let tup: (i32, u32, f32) = (2_140_000_000, 4_000_000_001, 6001.41);
    let (x, y, z) = tup;
    println!("Hello, world! {}, {}, {}", x, y, z);
    println!("Tuple: {}, {}, {}", tup.0, tup.1, tup.2);

    let arr = [5, 6, 7];
    println!("Arr: {}, {}, {}", arr[0], arr[1], arr[2]);

    let mut index = 2;
    println!("Arr[{}]: {}", index, arr[index]);

    index = index + 1;
    println!("Arr[{}]: {}", index, arr[index]);
}
