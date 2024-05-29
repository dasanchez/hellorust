# Chapter 8: Common Collections

* Most other data types represent one specific value, but colletions con contai multiple values.
* Unlike the array and typle types, the data that collections point to is stored on the heap- the amount of data does not need to be known at compile time, so it can grow or shrink as the program runs.
* This chapter covers three types of collections:
  * A _vector_ allows you to store a variable number of values next to each other.
  * A _string_ is a collection of characters.
  * A _hash map_ allows you to associated a value with a particular key. It's a particular implementation of the more general data structure called a _map_.
* There are other kinds of collections available in Rust.

## Storing lists of values with vectors

* Vectors allow you to store more thaan one value in a single data structure that puts all the values next to each other in memory.
* Vectors can only store values of the same type.
* They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

### Creating a new Vector

* Vectors are implemented using generics.
* The `Vec<T>` type provided by the standard library can hold any type.
* When we create a vector to hold a specific type, we can specify the type within angle brackets:
  ```
  let v: Vec<i32> = Vec::new(); # create an empty vector
  ```
* Rust will infer the type of value you want to store whe you create a `Vec<T>` with initial values. The following statement will create a new `Vec<i32>` vector:
  ```
  let v = vec![1, 2, 3];

### Updating a vector

* We can use the `push` method to add elements to a mutable vector.
  ```
  let mut Vec::new();
  v.push(5);
  ```
* The `pop` method removes and returns the last element of the vector.

### Reading elements of vecctors

* There are two ways to reference a value stored in a vector: via indexing or using the `get` method.
  * We can use `&v[2]` to get a reference to the third element.
  * We can use `v.get(2)` to get an `Option<&T>` that we can use  with `match`.
* The indexing method will cause the program to panic when attempting to access an out-of-bounds item because it references a nonexistent element.
* The `get` method will return `None` without panicking when attempting to access an out-of-bounds item.
  * You would use this method if accessing an element beyond the range of  the vector may happen occasionally under normal circumstances. Handling the out-of-bounds index may be more user-friendly than crashing the program.
  * If we hold an immutable reference to an element in a vector and we try to add an element to the end of the vector, the program won't wwork.
    * Vectors put the values next to each other in memory. Adding a new elemnt onto the end of the vector may require allocating new memory and copying the old elments to the new space if there isn't enough room to put all the elements next to each other where the vector is currently stored. The borrowing rules prevent programs ending up in a situation where the reference to the vector element ends up pointing to deallocated memory.

### Iterating over the values in a vector

* To access each element in a vector in turn, we iterate through all the elements rather than use indices to access one at a time. We use a `for` loop to get mutable or immutable references to each element in a vector.
* To alter the values of a vector as we're iterating through its elements, the vector and the reference must be mutable.
  * To change the value that the mutable reference refers to, we have to use the `*` dereference operator to get the value before changing it.
  * Iterating over a vector is safe because of the borrow checker's  rules.

### Using an enum to store multiple types

* The variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum.
* If we want to get values from a row in a spreadsheet in which some of the columns contain integers, some floats, and some strings, we can define an enum whose variants hold the different value types. All the enum variants will be considered the same type.

### Dropping a vector drops its elements

* A vector is freed when it goes out of scope.
* When the vector gets dropped, all of its contents are also dropped. The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.

## Storing UTF-8 encoded text with Strings

* Strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text.

### What is a string?

* Rust has only one string type in the core language, which is the slice `str` that is usually seen in its borrowed form `&str`.
* String slices are references to some UTF-8 encoded string data stored elsewhere.
* String literals are stored in the program's binary and are therefore string slices.
* The `String` type, which is provided by Rust's standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.
* Both `String` and string slices are UTF-8 encoded.

### Creating a new string

* Many of the same operations available with `Vec<T>` are available with `String` as well, because `String` is implemented as a wrapper around a vector of bytes. The `new` function works the same way as with `Vec<T>`- the following line creates a new empty string that we can load data into.
  ```
  let mut s = String::new();
  ```
* To start the string with some initial data:
  ```
  let data = "initial contents";
  let s = data.to_string();
  ```
  or:
  ```
  let s = "initial contents".to_string();
  ```
  or:
  ```
  let s = String::from("initial contents");
  ```

### Updating a string

* A `String` can grow in size and its contents can change, just like the contents of a `Vec<T>`.
* You can use the `+` operator or the `format!` macro to concatenate `String` values.

#### Appending to a String with `push_str` and `push`

* We can grow a `String` by using the `push_str` method to append a string slice.
* The `push_str` method takes a string slice because we don't necessarily want to take ownership of the parameter. If it took ownership of the parameter, we wouldn't be able to use it afterwards.
* The `push` method takes a single character as a parameter and adds it to the `String`.

#### Concatenation with the `+` operator and the `format!` macro

* One way to combine two existing srings is to use the `+` operator.
* The `+` operator uses the `add` method, whose signature looks something like this:
```
fn add(self, s: &str) -> String {
```
* We can only add a `&str` to a `String`, we can't add two `String` values together.
* The second argument of the `+` operator is stll valid after the operation, but not the first one.
* To concatenate multiple strings, we can use  the `format!` macro:
```
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{s1}-{s2}-{s3}");
```
* The code generated by the `format!` macro uses references so that this call doesn't take ownership of any of its parameters.

### Indexing into strings

* Rust strings don't support indexing.
* A `String` is a wrapper over a `Vec<u8>`. Some characters may take more than one byte to encode in UTF-8.
* There are three relevant ways to look at strings from Rust's perspective: as bytes, scalar values, and grapheme clusters. Rust provides different ways of interpreting the raw string data that computers store so that each program can choose the interpretation it needs.
* Indexing operations are expected to always take constant time (O(1)). It isn't possible to guarantee that performance with a `String`, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

### Slicing strings

* Indexing into a string is often a bad idea because it's not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice.
* Rather than indexing using `[ ]`, with a single number, you can use `[ ]` with a range to create a string slice containing particular bytes. In the example below, s will be Зд because each character is 2 bytes.
```
let hello = "Здравствуйте";

let s = &hello[0..4];
```
* You should use ranges to create string slices with caution, because doing so can crash your program.

### Methods for iterating over strings

* The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.
* For individual Unicode scalar values, use the `chars` method.
* The `bytes` method returns each byte.
* Remember that valid Unicode scalar values may be made up of more than 1 byte.
* Crates that provide the functionality to get grapheme clusters from strings are available on crates.io.

## Storing keys with associated values in hash maps

* The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a _hashing function_, which determines how it places these keys and values into memory.
* Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type. 

### Creating a new hash map

* We need to `use` the `HashMap` from the collections portion of the standard library, it's not included in the features brought into scope automatically in the prelude.
* Hash maps also have less support from the standard library- there's not built-in macro to construct them.
* One way to create an empty hash map is using `new` and adding elements with `insert`.
* Hash maps store their data on the heap. Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.

### Accessing values in a hash map

* We can get a value out of the hash map by providing its key to the `get` method.

### Hash maps and ownership

* For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map.
* For owned values like `String`, the values will be moved and the hash map will be the owner of those values.
* If we insert references to values into the hash map, the values won't be moved into the hash map. The values that the refeerences point to must be valid for at least as long as the hash map is valid.

### Updating a hash map

* Although the number of key and value pairs is growable, each unique key can only have one value associated with it at a time (but not vice versa).
* WHen you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned: 
  * You could replace the old valuue with the new value
  * You could keep the old value and ignore the new value, only adding the new vvalue if the key _doesn't_ already have a value.
  * You could combine the old and new values.

#### Overwriting a value

* If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced.

#### Adding a key and value only if a key isn't present

* If the key does exist in the hash map, the existing value should remain the way it is. If the key doesn't exist, insert it and a value for it.
* Hash maps have a special API for this called `entry` that thakes the key you want to check as a parameter. The return value of the `entry` method is an enum called `Entry` that represents a value that might or might not exist.

#### Updating a value based on the old value

* Look up a key's value and then update it based on the old value.
