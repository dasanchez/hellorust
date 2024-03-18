### Ownership

* Ownership is a set of rules that governs how a program manages memory.

#### The stack and the heap

* In a systems programming language like Rust, whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions.
* The stack and the heap are parts of memory available to your code to use at runtime, but the are structured in different ways.
* The stack stores values in the order it gets them and removes the values in the opposite order: last in, first out.
* Adding data is called _pushing onto the stack_, and removing data is called _popping off the stack_.
* All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead of the stack.
* The heap is less organized: When you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a _pointer_, which is the address of that location. This process is called _allocating on the heap_. Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.
* Pushing to the stack is faster than allocating on the heap because the allocation never has to search for a place to store new data; that location is always at the top of the stack. Allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
* Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory.
* When your code calls a function, the values passed into the function and the function's local variables get pushed onto the stack. When the function is over, those values get popped off the stack.
* Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don't run out of space are all problems that ownership addresses.

#### Ownership rules

* Each value in Rust has an _owner_.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

#### Variable scope

* A scope is the range within a program for which in item is valid. A variable is valid from the point at which it's declared until the end of the current scope.

### The String type

* String literals: a string value is hardcoded into the program.
* Rust has a second string type, `String`. This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
* You can create a `String` from a string literal using the `from` function:
```
let s = String::from("hello");
```
* The `::` operator allows us to namespace this particular `from` function under the `String` type.

#### Memory and allocation

* With the `String` type, we need to allocate an amount of memory on the heap, unknwon at compile time, to hold the text contents. This means:
  1. The memory must be requested from the memory allocaator at runtime.
     * This is done by us when we call `String::from`.
  2. We need a way of returning the memory to the allocator when we're done with our String.
     * The memory is automatically returned when the variable goes out of scope.
* When a variable goes out of scope, Rust calls a special function for us. It's called `drop`, and it's where he author of `String` can put the code to return the memory
* Rust calls `drop` automatically at the closing curly brackets.

#### Variables and data interacting with Move and Clone

* When we assign a String variable to a new variable, the operation is called a _move_, and the previous variable is no longer valid.
* Rust will never automatically create "deep" copies of your data.
* If we _do_ want to deeply copy the heap data of a `String` variable, not just the stack data, we can use a common method called `clone`.

#### Ownership and functions

* Passing a variable to a function will move or copy, just as assignment does.
* Returnnig values can also transfer ownership.
* Rust lets us return multiple values using a tuple.

### References and Borrowing

* A reference is like a pinter in that it's an address we can follow to acces the data stored at that address; that data is owned by some other variable.
* Unlike apointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
* Ampersands represent references, and they allow you to refer to some value without taking ownership of it.
* The opposite of referencing by using & is dereferencing, which is accomplished with the derefernece operator, `*`.
* When functions have references as parameters instead of the actual values, we won't need to return the values in order to give back ownership, because we never had ownership.
* We call the action of creating a reference _borrowing_.
* Just as variables are immutable by default, so are references.

#### Mutable references

* We can make mutable references by using the `mut` keyword before the function parameter and passing a mutable variable to that function.
* If you have a mutable reference to a value, you can have no other references to that value.
* The benefit of having this restrictuion is that Rust can prevent data races at compile time.
* A _data race_ is similar to a race condition and happens when these three behaviours occur:
  * Two or more pointers access the same data at the same time.
  * At least one of the pointers is being used to write to the data.
  * There's no mechanism being used to synchronize access to the data.
* Data races cause undefined behaviour and can be difficult to diagnose and fix when you're trying to track them down at runtime. Rust prevents this problem by refusing to compile code with data races.
* We _also_ cannot have a mutable reference while we habe an immutable one to the same value: users of an immutable reference don't expect the value to suddenly change.

#### Dangling references

* In languages with pointers, it's easy to erroneously create a _dangling pointer_- a pointer that references a location in memory that may have been given to someone else- by freeing some memory while preserving a pointer to that memory.
* In Rust, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

#### Rules of references

* At any given time, you can have _either_ one mutable reference _or_ any number of immutable references.
* References must always be valid.

### The Slice type

* _Slices_ let you reference a contiguous sequence of elements in a collection rather than the whole collection.
* A slice is a kind of reference, so it does not have ownership.
* A _string slice_ is a reference to part of a `String`.
* We create slices using a range within brrackets by specifying _[starting_index..ending_index]_, where _starting_index_ is the first position in the slice and _ending_index_ is one more than the last position in the slice.
* Internally, the slice data structure stores the starting position and the length of the slice.
* String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.
* The type that signifies "string slice" is written as `&str`, and it is an immutable reference.
* Defining a function to take a string slice instead of a reference to a `String` makes our API more general and useful without losing any functionality.

#### Other slices

* String slices are specific to strings, but there's a more general slice type too:
```
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);
```
* This slice has the type `&[i32]`, and it works the same way as string slices do, by storing a reference to the first element and a length.
