### Hello, World!

* To compile a binary: `rustc main.rs`
* The `main` function is always the first code that runs in every executable Rust program.
* It's good style the place a function's opening curly bracket on the same line as the function declaration.
* You can use an automated formatter tool called `rustfmt` to cormat your code in a particular style.
* Rust style is to indent with four spaces, not a tab.
* `println!` calls a Rust macro.

### Hello, Cargo!

* Cargo is Rust's build system and package manager.
* To create a new project: `cargo new hello_cargo`.
* To build a Cargo project: `cargo build`.
* To build and run a Cargo project: `cargo run`.
* To build a project without producing a binary to check for errors: `cargo check`.
* Cargo stores the result of the build in the `target/build` directory.
* To compile with optimizations for release: `cargo build --release`.