// Creating a string

// fn main() {
//     // let mut s = String::new();
//     // println!("string: {s}");

//     // let data = "initial contents";
//     // println!("string slice: {data}");

//     // let data = "initial contents";
//     // let s = data.to_string();
//     // println!("string: {s}");

//     // let s = "initial contents".to_string();
//     // println!("string: {s}");

//     let s = String::from("initial contents");
//     println!("string: {s}");
// }

//Updating a string

// fn main() {
    // let mut s = String::from("foo");
    // println!("string: {s}");
    // s.push_str("bar");
    // println!("string: {s}");

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s1: {s1}");
    // println!("s2: {s2}");

    // let mut s = String::from("lo");
    // s.push('l');
    // s.push('s');
    // println!("s: {s}");
    // s.pop();
    // println!("s: {s}");

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("World!");
    // let s3 = s1 + &s2; // s1 has moved here and can no longer be used
    // println!("s3: {s3}");

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = format!("{s1}-{s2}-{s3}");
    // println!("s1: {s1}, s2: {s2}, s3: {s3}, s: {s}");
// }

// Slicing strings
// fn main() {
//     let s1 = String::from("Hi there!");
//     let s2 = &s1[0..2];
//     println!("s2: {s2}");
// }

// Iterating over strings
fn main() {
    let s1 = String::from("Hi there!");
    for c in s1.chars() {
        println!("{c}");
    }
    for b in s1.bytes() {
        println!("{b}");
    }
}