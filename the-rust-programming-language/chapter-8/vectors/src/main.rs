// fn main() {
//     println!("Hello, world!");
//     // let v: Vec<i32> = Vec::new(); // creating a new empty vector to hold values of type i32
//     let v = vec![1, 2, 3];
//     println!("Vector: {:#?}", v);
// }

// fn main() {
//     println!("Hello, world!");
//     // let v: Vec<i32> = Vec::new(); // creating a new empty vector to hold values of type i32
//     let mut v = vec![1, 2, 3];
//     v.push(4);  // updating a vector with push
//     v.push(5);
//     println!("Vector: {:#?}", v);
// }

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];
    
//     let third: &i32 = &v[2]; // reading an element via indexing
//     println!("The third element is {third}");
    
//     let third: Option<&i32> = v.get(2); // reading an element via get method
//     match third {
//         Some(third) => println!("The third element is {third}"),
//         None => println!("There is no third element."),
//     }
// }

    
// fn main() {
//     let v = vec![1, 2, 3, 4, 5];
    
//     // let does_not_exist = &v[100];  // access nonexistent index -> panic
    
//     let does_not_exist = v.get(100); // access nonexistent index
//     match does_not_exist {
//         Some(does_not_exist) => println!("The element is {does_not_exist}"),
//         None => println!("There is no element."),
//     }

// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];
    
//     let first = &v[0]; // create a reference to the first element: immutable borrow
//     println!("First: {first}");
//     v.push(6);  // mutable borrow
//     // println!("First: {first}"); // immutable borrow used -> no-no
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];
    
//     let first = &v[0]; // create a reference to the first element: immutable borrow
//     println!("First: {first}");
//     v.push(6);  // mutable borrow
//     println!("First: {first}"); // immutable borrow used -> no-no
// }

// fn main() {
//     let v = vec![100, 32, 57];
    
//     for i in &v {   // iterating over the vector elements as immutable references  
//         println!("{i}");
//     }
// }

// fn main() {
//     let mut v = vec![100, 32, 57];
    
//     for i in &v {   // iterating over the vector elements as immutable references  
//         println!("{i}");
//     }

//     for i in &mut v {   // iterating over the vector elements asmutable references  
//         *i += 50;
//     }

//     for i in &v {   // iterating over the vector elements as immutable references  
//         println!("{i}");
//     }
// }

fn main() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),    
    ];
    
    for cell in &row {   // iterating over the vector elements as immutable references  
        println!("{:?}", cell);
    }

}