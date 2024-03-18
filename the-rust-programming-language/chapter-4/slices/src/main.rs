// // write a function that takes a string of words separated by spaces
// // and returns the first word it finds in that string.
// // if the function doesn't find a space, the entire string should be returned.
// fn main() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s); // word will be a string slice
    
//     s.clear(); // compiler error: this empties the String, making it equal to "": it requires a mutable reference
//     println!("The first word is: {word}");
// }
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

// using string slices as parameters
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("{word}");
    let word = first_word(&my_string[..]);
    println!("{word}");
    // first word also works on references to `String`s, which are equivalent to whole slices of `String`s
    let word = first_word(&my_string);
    println!("{word}");

    let my_string_literal = "hello world";

    // first_word works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("{word}");
    let word = first_word(&my_string_literal[..]);
    println!("{word}");
    // because string literals *are* string slices already,
    // this works too, without the slice syntax
    let word = first_word(my_string_literal);
    println!("{word}");
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s
}
