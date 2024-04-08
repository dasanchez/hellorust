# Managing Growing Projects with Packages, Crates, and Modules

* As a project grows, you should organize code by splitting it into multiple modules and then multiple files.
* A package can contain multiple binary crates and optionally one library crate.
* The Rust module system includes:
  * **Packages**: A Cargo feature that lets you build, test, and share crates
  * **Crates**: A tree of modules that produces a library or executable
  * **Modules** and **use**: Let you control the organization, scope, and privacy of paths
  * **Paths**: A way of naming an item, such as a struct, function, or module

## Packages and Crates

* A _crate_ is the smallest amount of code that the Rust compiler considers at a time.
* Crates can contain modules, and the modules may be defined in other files that get compiled with the crate.
* A crate can come in one of two forms: a binary crate or a library crate.
* The _crate root_ is a source file that the Rust complier starts from and makes up the root module of your crate.
* A _package_ is a bundle of one or more crats that provides a set of functionality.
* A package contains a _Cargo.toml_ file that describes how to build those crates.
* A package can contain as many binary crates as you like, but at most only one library crate.
* A package must contain at least one crate, whether that's a library crate or a binary one.
* Cargo follows a convention that _src/main.rs_ is the crate root of a binary crate with the same name as the package.
* If the package directory contains _src/lib.rs_, the package contains a library crate with the same name as the package, and _src/lib.rs_ is its crate root.

## Defining modules to control scope and privacy

### Modules Cheat Sheet

* Start from the crate root (_src/lib.rs_ for a library crate or _src/main.rs_ for a binary crate).
* Declaring modules: You can declare new modules in the crate root file (e.g. `mod garden;`). The compiler will look for the module's code in these places:
  * Inline within curly brackets that replace the semicolon following `mod garden`
  * In the file _src/garden.rs_
  * In the file _src/garden/mod.rs_
* Declaring submodules: YOu can declare submodules in any other file than the crate root.
  * Inline directly following `mod vegetables` within curly braces in _src/garden.rs_
  * In _src/garden/vegetables.rs_
  * In _src/garden/vegetables/mod.rs_
* Paths to code in modules: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example: `crate::garden::vegetables::Asparagus` for the `Asparagus` type.
* Private vs public: Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. Use `pub` before items within a public module to make them public as well.
* The `use` keyword: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. You can create a shortcut with `use crate::garden::vegetables::Asparagus` and from then on you only need to write `Asparagus` to make use of that type in the scope.

### Grouping related code in modules

* Modules let us organizee code within a crate for readability and easy reuse.
* Modules also allow us to control the _privacy_of items, because code within a module is private by default.
* To create a new library, use `cargo new <library_name> --lib`.
* We define a module with the `mod` keyword followed by the name of the module.
* The body of a module then goes inside curly braces.
* Inside modules, we can place other modules.
* Modules can also hold definitions for other items, such as structs, enums, constants, traits, and functions.


## Paths for referring to an item in the module tree

* Rst needs a path to find an item in a module tree, just like when navigating a filesystem.
* A path can take two forms: _absolute_ and _relative.
  * An absolute path starts from the `crate` root, or the crate name for code from an external crate.
  * A relative path starts from the current module and uses `self`, `super`, or an identifier in the current module.
  * Both absolute and relative paths are followed by one or more identifiers separated by double colons (`::`).
  * Starting with a module name means that the path is relative.
* Items in a parent module can't use the private items inside child modules, but items in child modules can use the items in their ancestor modules.
* If you plan on sharing your library crate so other projects can use your code, your public API is your contract with users of your crate that determines how they can interact with your code.

### Best practices for packages with a binary and a library

* Packages that include both a _src/main.rs_ binary crate root as well as a _src/lib.rs_ library crate root will have just enough code in the binary crate to start an executable that calls code with the library crate.
* The module tree should be defined in _src/lib.rs_. Then, any public items ban be used in the binary crate by starting paths with the name of the package. The binary crate becomes a user of the library crate just like a completely external crate would use it.

### Starting relative paths with `super`

* We can construct relative paths that begin in the parent module by using `super` at the start of the path.

### Making structs and enums public

* We can also use `pub` to designate structs and enums as public.
* If we use `pub` before a struct definition, we make the struct public, but the struct fields will still be private. We can make each field public or not on a case-by-case basis.
* If we make an enum public, all of its variants are then public. We only need the `pub` before the `enum` keyword.
* Enums aren't very useful unless their variants are public: the default for enum variants is to be public.
* Structs are often useful without their fields being public.

## Bringing paths into scope with the `use` keyword

* We can create a shortcut to a path with the `use` keyword once, and then use the shorter name everywhere else in the scope.
* Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem.

### Creating idiomatic `use` paths

* Bringing a function's parent module into scope with `use` means we have to specify the parent module when calling the function. Specifying the parent module when calling the function makes it clear that the function isn't locally defined while still minimizing repetition of the full path.
* When bringing in structs, enums, and other items with `use`, it's idiomatic to specify the full path (i.e. `use std::collections::HashMap`).
* If we're bringing two items with the same name into scope with `use`, we only specify the module path.

### Providing new names with the `as` keyword

* We can specify `as` and a local new name, or _alias_, for the type after the `use` path:
  ```
  use std::fmt::Result
  use std::io::Result as IoResult
  ```

### Re-exporting names with `pub use`

* When we bring a name into scop with the `use` keyword, the name available in the new scope is private. TO enable the code that calls our code to refer to that name, we can combine `pub` and `use`. This technique is called _re-exporting_ because we're bringing an item itno scope but also making that item available for others to bring into their scope.
* Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain.

### Using external packages

* Members of the Rust community have made many packages available at crates.io, and pulling any of them into your package involves these steps: listing them in your package's `Cargo.toml` file and using `use` to bring items from their crates into scope.
* The standard `std` library is also a crate that's external to our package, but we don't need to change `Cargo.toml` to include `std`.

### Using nested paths to clean up large `use` lists

* We can use nested paths to bring multiple items from the same crate or module into scope. We can replace this:
  ```
  use std::cmp::Ordering;
  use std::io;
  ```
  with this:
  ```
  use std::{cmp::Ordering, io};
  ```
* We would also replace this:
  ```
  use std::io;
  use std::io::Write;
  ```
  with:
  ```
  use std::io::{self, Write};
  ```

### The glob operator

* If we want to bring _all_ public items defined in a path into scope, we can specify that path followed by the `*` glob operator: `use std::collections::*;`
* Be careful when using the glob operator - it can make it harder to tell what names are in scope and where a name used in your program was defined.

## Separating modules into different files

* You can remove the code inside a module declaration's curly braces and move it to its own _.rs_ file. Replace the curly braces with a semicolon.
* You only need to load a file using a `mod` declaration _once_ in your module tree. Once the compiler knows the file is part of the project, other files in your project should refer to the loaded file's code using a path to where it was declared.

### Alternate file paths

* For a module named `front_of_house` declared in the crate root, the compiler will look for the module's code in:
  * _src/front_of_house.rs_
  * _src/front_of_house/mod.rs_ (older style)
* For a module named `hosting` that is a submodule of `front_of_house`, the compiler will look for the module's code in:
  * _src/front_of_house/hosting.rs_
  * _src/front_of_house/hosting/mod.rs_ (older style)
* You cannot use both styles for the same module.
* The module tree remains the same after moving each module's code to a separate file.