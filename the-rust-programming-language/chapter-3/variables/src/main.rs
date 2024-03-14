fn main() {
    // Compile error
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // // Mutable variable
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // Constant
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("Const value is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("THe value of x in the inner scope is: {x}");
    // }
    // println!("The value of x is: {x}");

    // Floating point numbers
    // let x = 2.0; // f64
    // let y: f32 = 3.0; //f32

    // Numeric operations
    // let sum = 5 + 10;
    // let difference = 95.5 - 4.3;
    // let product = 4 * 30;
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3; // Results in -1
    // let remainder = 43 % 5;

    // Boolean type
    // let t = true;
    // let f: bool = false; // explicit type annotation

    // Character type
    // let c = 'z';
    // let z: char = 'Z'; // explicit type annotation
    // let heart_eyed_cat = '😻';

    // Tuples
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup; // this is called destructuring
    
    // println!("The value of y is: {y}");

    // let five_hundred = tup.0;
    // let one = tup.2;
    // println!("five_hundred is: {five_hundred}");
    // println!("one is: {one}");

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];
    let d = ["January", "February", "March"];
    let a_first = a[0];
    let b_second = b[1];
    let c_fifth = c[4];
    let d_third = d[2];
    println!("a[0]: {a_first}, b[1]: {b_second}, c[4]: {c_fifth}, d[2]: {d_third}");
}
