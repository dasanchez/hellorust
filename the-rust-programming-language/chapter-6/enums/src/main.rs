// 1.
// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }
// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     println!("Four is {:#?}", four);
// }

// 2.
// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }
// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
// fn main() {
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
//     println!("home is {:#?}", home);
// }

// 3.
// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// fn main() {
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
//     println!("home is {:#?}", home);
// }

// 4.
// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8 ,u8),
//     V6(String),
// }
// fn main() {
//     let home = IpAddr::V4(127,0,0,1);
//     let loopback = IpAddr::V6(String::from("::1"));
//     println!("home is {:#?}", home);
//     println!("loopback is {:#?}", loopback);
// }

// 5.
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// impl Message {
//     fn call(&self) {
//         println!("Message: {:#?}", self);
//     }
// }
// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

// 6. Option<T>
fn main() {
    let some_number = Some(5); // type of some_number: Option<i32>
    let some_char = Some('e'); // type of some_char: Option<char>
    let absent_number: Option<i32> = None;
    let number_value = some_number.unwrap();
    
    println!("number_value: {number_value}");
}