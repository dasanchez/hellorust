// infinite loop
// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// returning values from loops
// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// loop labels
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("End count = {count}");
// }

// while loop
// fn main() {
//     let mut number = 5;
//     while number != 0 {
//         println!("{number}!");
//         number -= 1;
//     }
//     println!("LIFTOFF!");
// }

// use while to loop over the elements of a collection
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5{
//         println!("the value is: {}", a[index]);
//         index += 1;
//     }
// }

// looping through a collection with for
// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }

// countdown with for
fn main() {
    for number in (1..6).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!");
}