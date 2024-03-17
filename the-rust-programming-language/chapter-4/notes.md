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

### References an Borrowing