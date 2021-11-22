//  Pattern

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn convert_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn do_unwrap() {
    let x: i8 = 5;
    let y: Option<i8> = None;

    let sum: i8 = x + y.unwrap_or(0);
    println!("{}", sum);
}

fn main() {
    do_unwrap();
    println!("p:{}, n:{}, d:{}, q:{}",
        convert_coin(Coin::Penny),
        convert_coin(Coin::Nickel),
        convert_coin(Coin::Dime),
        convert_coin(Coin::Quarter)
    );

    let some_coin = Some(Coin::Penny);

    match some_coin {
        Some(Coin::Penny) => {
            println!("Got a penny");
        },
        _ => () //Does nothing
    }

    if let Some(Coin::Penny) = some_coin {
        println!("Just match on a penny!")
    }

}
