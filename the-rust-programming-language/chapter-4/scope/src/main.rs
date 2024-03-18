// fn main() {
//     let s = String::from("hello"); // immutable
//     println!("{s}");

//     let mut s = String::from("hello"); // mutable shadow
//     s.push_str(". world!"); // push_str() appends a literal to a String
//     println!("{s}");
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1; // move operation: only the stack data is copied

//     println!("{s2}, world!"); // OK
//     // println!("{s1}, world!"); // Compiler error: borrow of moved value
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone(); // "deep copy": both the stack and heap data are copied

//     println!("s1 = {s1}, s2 = {s2}");
// }

// fn main() {
//     let x = 5;
//     let mut y = x;
//     y += 1;
//     println!("x = {x}, y = {y}");
// }

// // ownership and functions: String move operation
// fn main() {
//     let s = String::from("hello"); // s comes into scope
//     takes_ownership(s); // s's value moves into the function and is no longer valid here
//     // println!("{s}"); // compiler error
// }
// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{some_string}");
// } // some_String goes out of scope and drop is called; the backing memory is freed

// // ownership and functions: i32 copy operation
// fn main() {
//     let x = 5; // x comes into scope
//     makes_copy(x); // x's value is copied into the function scope
//     println!("x: {x}");
// }
// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{some_integer}");
// } // some_integer goes out of scope

// // Return values and scope
// fn main() {
//     let s1 = gives_ownership(); // gives_ownership moves its return value into s1
//     let s2 = String::from("hello"); // s2 comes into scope
//     let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
//     println!("s1: {s1}, s3: {s3}");
// } // s3 goes out of scope and is dropped, s2 was moved so nothing happens, s1 goes out of scope and is dropped
// fn gives_ownership() -> String {
//     let some_string = String::from("yours"); // some_string comes into scope
//     some_string // some_string is returned and moves out to the calling function
// }
// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// Return multiple values using tuples
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
