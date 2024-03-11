Chapter 2: make a number-guessing game.

### Processing a guess

* To obtain input and print the output, we need to bring the `io` library into scope. This library comes from the standard library, known as `std`: `use std::io;`
* The `prelude` is a set of items defined in the standard library that Rust brings into scope of every program by default.
* We use a `let` statement to create variables.
* In Rust, variables are immutable by default.
* To make a variable mutable, we add `mut` before the variable name.
* `String` is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
* An associated function is implemented on a type, like `::new` on `String`.
* The `stdin` function returns an instance of `std::io::Stdin`, which is a type that represents a handle to the standard input for your terminal.
* References, like variables, are immutable by default.
* `read_line` returns a `Result` value, which is an `enum` type. An `enum` type can be in one of multiple possible states, or variants.
  * `Result`'s varianbt are `Ok` and `Err`. The `Ok` variant indicates a successful operation, and inside `Ok` is the successfully generated value. The `Err` variant means the operation failed, and `Err` contains information about how or why the operation failed.
  * Values of the `Result` type have methods defined on them. An instance of `Result` has an `expect` method you can call. If this instance of `Result` is an `Err` value, `expect` will cause the porgam to crash and display the message that you passed as an argument to `expect`.
* When printing the value of a variable, the variable name can go inside curly brackets.

### Generating a secret number

* To generate a randon number, we use the `rand` crate.
* To use the `rand` crate, add this line to the Cargo.toml dependencies: `rand = "0.8.5"`.
* When we include an external dependency, Cargo fetches the latest versions of everything from Crates.io. Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.
* The `Rng` trait in `use rand::Rng` defines methods that random number generators implement.
* Run `cargo doc --open` to build documentation provided by all your dependencies locally and open it in your browser.

### Comparing the guess to the secret number

* `Ordering` is an enum type with the varians `Less`, `Greater`, and `Equal`.
* A `match` expression is made up of _arms_. An arm consists of a _pattern_ to match against, and the code that should be run if the value given to `match` fits that arm's pattern.
* Rust allows us to shadow the previous value of `guess` with a new one. _Shadowing_ lets us reuse the `guess` variable name rather than forcing us to create two unique variables, such as `guess_str` and `guess`, for example.
* `trim` will remove whitespace at the beginning and end from a `String` instance.
* `parse` on strings converts a string to another type.

### Allowing multiple guesses with looping

* The `loop` keyword creates an infinite loop.