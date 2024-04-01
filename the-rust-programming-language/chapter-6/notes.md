# Enums and pattern matching

## Enums

* Enums allow you to define a type by enumerating its possible _variants_. They give you a way of saying a value is one of a possible set of values.
* We can attach data to each variant of an enum: the name of each enum variant that we define becomes a function that constructs an instance of the enum.
  * Each variant can have different types and amounts of associated data.
* Just as we're able to define methods on structs using `impl`, we're also able to define methods on enums.
* An enum called `Option` expresses that a value can be either something or nothing. It is defined by the standard library as follows:
  ```
  enum Option<T> {
    None,
    Some(T),
  }
  ```
* The `Option<T>` enum is included in the prelude. Its variants are also included in the prelude.
* You have to convert an `Option<T>` to a `T` before you can perform `T` operations with it.
* In general, in order to use an `Option<T>` value, you want to have code that will handle each variant. The `match` expression will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.

## The `match` control flow construct

* `match` allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
* When the `match` expression executes, it compares the resultant value against the pattern of each arm, in order.
* The code associated with each arm is an expression, and the resultant value of the expression in the matching arm is the value that that gets returned for the entire match expression.
* Match arms can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.
* The patterns in a match arms must cover all possibilities: if the expression results in an `Option<T>` type, `None` and `Some` types must both be covered.
* Using enums, we can take special actions for a few particular values but take one default action for all other values.

## Concise control flow with if let

* `if let` takes a pattern and an expression separated by an equal sign. The code in the `if let` block isn't run if the value doesn't match the pattern. `if let` is syntax sugar for a `match` that runs code when the value matches one pattern and then ignores all other values.
* We can include an `else` with an `if let` to run "default" code for all other cases that don't match the one from `if let`.