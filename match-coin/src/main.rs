fn main() {
    println!("Hello, world!");
    coin_values();
    plus_values();
    if_let_example();
}

fn coin_values() {
    let coin: Coin = Coin::Penny;
    println!("Coin {:?} value is: {}", coin, value_in_cents(&coin));

    let coin = Coin::Nickel;
    println!("Coin {:?} value is: {}", coin, value_in_cents(&coin));

    let coin = Coin::Quarter(UsState::OhForFucksSake);
    println!("Coin {:?} value is: {}", coin, value_in_cents(&coin));
}

#[derive(Debug)]
enum UsState {
    OhForFucksSake,
    ItDoesNotReallyMatter,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>  {
            println!("Quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some (i + 1),
        None => None,
    }
}

fn plus_values() {
    let five = Some(5);
    let six = plus_one(five);
    println!("Five is: {:?}", five);
    println!("Six is: {:?}", six);
}

fn if_let_example() {
    let some_u8 = Some(3);

    if let Some(3) = some_u8 {
        println!("Some is 3");
    } else {
        println!("Some is not 3");
    }

    if let Some(5) = some_u8 {
        println!("Some is 5");
    } else {
        println!("Some is not 5");
    }
}
