use std::collections::HashMap;

// // Creating and accessing hash maps
// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
//     let team_name = String::from("Blue");
    
//     // Calling copied handles the Option returned by get
//     // to get an Option<i32> rather than an Option<&i32> 
//     // unwrap_or sets the score to 0 if scores doesn't have
//     // and entry for the key.
    
//     let score = scores.get(&team_name).copied().unwrap_or(0);
//     println!("Score: {score}");
// }

// // Iterating over a hash map
// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
    
//     for (key, value) in &scores {
//         println!("{key}: {value}");
//     }
// }

// // Ownership
// fn main() {
//     let field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");

//     let mut map = HashMap::new();
//     map.insert(field_name, field_value);
//     // field_name and field_value are invalid at this point:
//     // println!("field_value: {field_value}");

//     let field_name_2 = String::from("Favorite number");
//     let field_value_2 = 5;

//     let mut map_2 = HashMap::new();
//     map_2.insert(field_name_2, field_value_2);
//     // only field_name is invalid at this point:
//     println!("field_value_2: {field_value_2}");
// }

// // Updating a hash map: overwriting a value
// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Blue"), 25);

//     println!("{:?}", scores);
// }

// // Updating a hash map: adding a key/value only if a key isn't present
// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.entry(String::from("Yellow")).or_insert(50);
//     scores.entry(String::from("Blue")).or_insert(50);
//     println!("{:?}", scores);
// }

// Updating a hash map: updating a valuue based on the old value
fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // the split_whitespace method returns an iterator over sub-slices.
    // separated by whitespace, of the value in text.
    // The or_insert method returns a mutable reference (&mut V) to the
    // value for the specified key. In order to assign to that value,
    // we must first dereference count using the asterisk (*).
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}