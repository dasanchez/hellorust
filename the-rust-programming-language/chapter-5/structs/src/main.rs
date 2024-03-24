// Defining and instantiating structs
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
//     println!("user1 username: {0}", user1.username);
// }


// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
//     println!("user1 username: {0}", user1.username);

//     user1.username = String::from("newusername456");
//     println!("user1 username: {0}", user1.username);    
// }


// build function:
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let user1 = build_user(String::from("someone@example.com"), String::from("someuser"));
//     println!("user1 username: {0}", user1.username);
// }
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

// Using the field init shorthand
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let user1 = build_user(String::from("someone@example.com"), String::from("someuser"));
//     println!("user1 username: {0}", user1.username);
// }
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// Creating instances from other instances with Struct Update Syntax: before
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let user1 = build_user(String::from("someone@example.com"), String::from("someuser"));
//     let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };
//     println!("user1 email: {0}", user1.email);
//     println!("user2 email: {0}", user2.email);
// }
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

// Creating instances from other instances with Struct Update Syntax: after
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let user1 = build_user(String::from("someone@example.com"), String::from("someuser"));
//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };
//     println!("user1 email: {0}", user1.email);
//     println!("user2 email: {0}", user2.email);
// }
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

// Using tuple structs without named fields to create different types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black.0: {0}", black.0);
    println!("origin.0: {0}", origin.0);
}
