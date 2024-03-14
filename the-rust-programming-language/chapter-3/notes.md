## Variables and mutability

* By default, all variables are immutable.
* You can make variables mutable by adding `mut` infront of the variable name.
* Constants are values that are bound to a name and are not allowed to change.
  * You aren't allowed to use `mut` with constants.
  * Declare constants by using the `const` keyword instead of the `let` keyword.
  * The type of the value _must_ be annotated.
  * Constants can be declared in any scope, including the global scope.
  * Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
  * Rust's naming convention for constants is to use all uppercase with underscores between words.
  * Constants are valid for the entire time a program runs, within the scope in which they were declared.
* You can declare a new variable with the same name as a previous variable: the first variable is _shadowed_ by the second.
  * Shadowing is different from marking a variable as `mut` because we'll get a compile error if we try to reassign to this variable without using the `let` keyword.
  * When we use the `let` keyword again, we can change the type of the value but reuse the same name.

## Data Types

* Every value in Rust is of a certain _data type_.
* There are two data type subsets: scalar and compound.
  * A scalar type represents a single value. Rust has four primary scalar types: Integers, floating-point numbers, Booleans, and characters.
  * Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### Integers

* The following are the built-in integer types in Rust.

| Length  | Signed  | Unsigned |
| :------ | :------ | :------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

* The `isize` and `usize` types depend on the architecture of the computer your program is running on: 64 bits if you're running on a 64-bit architecture.
* You can write literals in any of the forms below. You can use an underscore character to separate numbers and improve readability.

| Number literals | Example       |
| :-------------- | :------------ |
| Decimal         | `98_222`      |
| Hex             | `0xff`        |
| Octal           | `0o77`        |
| Binary          | `0b1111_0000` |
| Byte (u8 only)  | `b'A'`        |

* Integer types default to i32.

### Floating-Point Types

* Rust's floating-point types are `f32` (32-bits) and `f64` (64-bits).
* The default type is `f64`.
* All floating-point types are signed.

### The Boolean type

* A Boolean type in Rus has two possible values: `true` and `false`.
* Booleans are one byte in size.
* The Boolean type in Rust is specified using `bool`.
* The main way to use Boolean values is through conditionals, such as an `if` expression.

### The character type

* The `char` type is Rust's most primitive alphabetic type.
* We specify `char` literals with single quotes, as opposed to string literals, which use double quotes.
* Rust's `char` type is four bytes in size and represents a Unicode scalar value- it can represent a lot more than just ASCII.
* Unicode scalar values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive.

### The tuple type

* A _tuple_ is a general way of grouping together a number of values with a variety of types into one compound type.
* Tuples have a fixes length: once declared, they cannot grow or shrink in size.
* The tuple without any values has a special name, _unit_. This value and its corresponding type are both writen `()` and represent an empty value or an empty return type.
* Expressions implicitly return the unit value if they don't return any other value.

### The array type

* Every element of an array must have the same type.
* Arrays in Rust have a fixed length.
* Arrays are useful when you want your data allocated on the stack rather than the heap, or when you want to ensure you always have a fixed number of elements.
* An array isn't as flexible as the vector type: a _vector_ is a similar collection type provided by the standard library that _is_ allowed to grow or shrink in size.
  * If you're unsure whether to use an array or a vector, chances are you should use a vector.
* The program will result in a _runtime_ error at the point of using an invalid value in the indexing operation.

## Functions

* Rust code uses _snake case_ as the conventional style for function and variable names, in which all leters are lowercase and underscores separate words.

### Parameters

* Parameters are special variables that are a part of a function's signature.
* When a function has parameters, you can provide it with concrete values (arguments) for those parameters.
* In function signatures, you_must_declare the type of each parameter.

### Statements and expressions

* Rust is an expression-based language.
* Statements are instructions that perform some action and do not return a value.
  * Function definitions are also statements.
* Expressions evaluate to a resultant value.
  * Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression.
  * Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

### Functions with return values

* Functions can return values to the code that calls them.
* We don't name return values, but we must declare their type after an arrow (`->`).
* The return value of the function is synonymous with the value of the final expression in the block of the body of a function.
* You can return early from a function by using the `return` keyword and specifying a value.

## Control flow

* The most common constructs for controlling execution flow are `if` expressions and loops.

### `if` expressions

* Blocks of code associated with the conditions in `if` expressions are sometimes called _arms_, just like the arms in `match` expressions.
* The condition in an `if` expression _must_ be a `bool`.
* We can use an `if` expression on the right side of a `let` statement.`
* The values that have the potential to be results from each arm of the `if` must be the same type.

### Repetition with loops

* The `break` keyword tells the program when to stop executing the loop.
* `continue` tells the program to skip over any remaining code in the current iteration and go to the next iteration.
* To return a value from a loop, add the value after the `break` expression.
* If you have loops within loops, `break` and `continue` apply to the innermost loop at that point.
* You can specify a _loop label_ on a loop that you can then use with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop.

### Conditional loops with while

### Looping through a collection with for