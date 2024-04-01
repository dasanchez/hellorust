// 1.
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
// fn main() {
//     let lucky_penny = Coin::Penny;
//     println!("my lucky penny's worth {0} cent(s).", value_in_cents(lucky_penny));
// }

// 2.
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
// fn main() {
//     let lucky_penny = Coin::Penny;
//     println!("My lucky penny's worth {0} cent(s).", value_in_cents(lucky_penny));
// }

// 3.
// #[derive(Debug)]
// enum UsState {
//     Alaska,
//     California,
//     Georgia,
//     NewYork,
// }
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }
// fn main() {
//     let quarter = Coin::Quarter(UsState::NewYork);
//     println!("My quarters's worth {0} cent(s).", value_in_cents(quarter));
// }

// 4. Matching with Option<T>
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     println!("six is {0}", six.unwrap());
// }

// 5. Catch-all patterns
// fn main() {
//     let dice_roll = 3;
//     match dice_roll {
//         3 => println!("three!"),
//         def_path => println!("default path: {def_path}"),
//     }
// }

// 6. Catch-all patterns
// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => println!("three!"),
//         _ => println!("default path: we don't care about the value"),
//     }
// }

// 7. Handle values that match one pattern while ignoring the rest: the verbose way
// fn main() {
//     let config_max = Some(2u8);
//     match config_max {
//         Some(max) => println!("The maximum is configured to be {}", max),
//         _ => (),
//     }
// }

// 8. Handle values that match one pattern while ignoring the rest: the if let
fn main() {
    let config_max = Some(2u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
