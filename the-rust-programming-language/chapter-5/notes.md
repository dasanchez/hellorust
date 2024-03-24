## structs

* A `struct` is a custom data type that lets you package together and name multiple related values that make up a meaningful group.
* Like tuples, the pieces of a struct can be different types.
* Unlike with tuples, each peace of data must be named.
* To get a specific value from a struct, we use dot notation.
* We can change a value of with dot notation if the entire instance is mutable. Rust doesn't allow us to mark only certain fields as mutable.
* We can construct a new instance of a `struct` as the last expression in a function body to implicitly return that new instance.
* Rust also supports structs that look similar to tuples, called _tuple structs_.
  * Tuple structs have the added meaning the struct name provides but don't have names associated with their fields.
* You can define structs that don't have any fields: these are called _unit-like structs_ because they behave similarly to `()`.
  ```
  struct AlwaysEqual;
  fn main() {
    let subject = AlwaysEqual;
  }
  ```
* structs don't have a provided implementation of `Display` to use with `println!` and the `{}` placeholder.
  * Putting the specifier `:?` inside the curly brackets tells `println!` we want to use an output format called `Debug`.
  * We have to explicitly opt in to make printing debugging info available for our struct. We do that by adding `#[derive(Debug)]` just before the struct definition.
  * When we have larger structs, we can use `:#?` instead of `:?`.
  * Another way to print out a value using the `Debug` format is to use the `dbg!` macro, which takes ownership of an expression, prints the file and line number of where the macro occurs along with the resultant value of the expression, and returns ownership of the value.

## Method syntax

* Methods are similar to functions: we declare them with the `fn` keywords and a name, they can have parameters and a return value.
* Unlike functions, methods are defined within the context of a `struct`, and their first parameter is always `self`, which represents the instance of the `struct` the method is being called on.
* To define a function in the context of a struct, we start an `impl` (implementation) block. Everything in the `impl` block will be associated with that struct type. The first parameter must be `self` in the signature and everywhere within the body.
* We can use _method syntax_ to call the method on a struct instance by adding a dot after the instance followed by the method name, parentheses, and any arguments.
* Methods must have a parameter named `self` of type `Self` for their first parameter, so Rust lets you abbreviate this with only the name `self` in the first parameter spot.
* Methods can take ownership of `self`, borrow `self` immutably, or borrow `self` mutably.
* We can choose to give a method the same name as one of the struct's fields.
* Often (not always), when we give methods with the same name as a field we want them to return the value in the field and do nothing else. Methods like this are called _getters_, and Rust does not implement them automatically for struct fields as some other languages do.

### Associated functions

* All functions defined within an `impl` block are called _associated functions_ because they're associated with the type named after the `impl`.
* We can define associated functions that don't have `self` as their first parameter (and thus are not methods) because the don't need an instance of the type to work with. An example of this function is `String::from`.
* Associated functions that aren't methods are often used for constructors that will return a new instance of the struct.
* We could choose to provide an associated function named `square` that would have one dimension parameter and use that as both width and height:
  ```
  impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
  }
  ```
  * To call this function, we use the `::` syntax with the struct name, for example `let sq = Rectangle::square(3);` .
* Each struct is allowed to have multiple `impl` blocks.