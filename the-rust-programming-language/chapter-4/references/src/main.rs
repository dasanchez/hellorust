// // function parameter uses a reference instead of taking ownership of the value
// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("The lenght of '{s1}' is {len}.");
// }
// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } // s goes out of scope, but because it does not have ownership of what it refers to, the String is not dropped

// // attempting to change a reference will result in a compiler error
// fn main() {
//     let s = String::from("hello");
//     change(&s);
// }
// fn change(some_string: &String) {
//     some_string::push_str(", world");
// }

// // mutable reference
// fn main() {
//     let mut s = String::from("hello");
//     change (&mut s);
//     println!("{s}");
// }
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// // we cannot borrow a variable as mutable more than once at a time
// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s;
//     println!("{r1}, {r2}");
// }

// // we can use scope to manage immutable and mutable references to the same value
// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{r1} and {r2}"); // r1 and r2 are not used after this point, we can borrow s as mutable now
//     let r3 = &mut s;
//     println!("{r3}");
// }

// dangling references will be caught at compile time
fn main() {
    let reference_to_nothing = dangle();
    println!("{reference_to_nothing}");
}
fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s  // we return a reference to the String, s
} // s goes out of scope and is dropped, so its memory goes away - danger!