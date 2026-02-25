// Ownership in Rust
// Based on: https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html

/*
  Ownership Rules:
  1. Each value in Rust has an owner.
  2. There can only be one owner at a time.
  3. When the owner goes out of scope, the value will be dropped.
*/

pub fn ownership_examples() {
    println!("=== Ownership in Rust ===\n");

    // 1. Variable scope
    demonstrate_variable_scope();

    // 2. The String type and heap allocation
    demonstrate_string_type();

    // 3. Move semantics
    demonstrate_move_semantics();

    // 4. Clone for deep copy
    demonstrate_clone();

    // 5. Copy trait for stack data
    demonstrate_copy_trait();

    // 6. Ownership and functions
    demonstrate_ownership_and_functions();

    // 7. Return values and ownership
    demonstrate_return_values();

    // 8. Scope and assignment
    demonstrate_scope_and_assignment();

    // 9. Deep Dive: Memory Drop Visualizer (Premium)
    demonstrate_drop_behavior();
}

// 1. Variable scope
fn demonstrate_variable_scope() {
    println!("1. Variable Scope");
    println!("   Variables are valid from declaration until end of scope\n");

    {
        // s is not valid here, it's not yet declared
        let s = "hello"; // s is valid from this point forward
        println!("   Inside scope: s = {}", s);
        // do stuff with s
    } // this scope is now over, and s is no longer valid

    println!("   Outside scope: s is no longer accessible\n");
}

// 2. The String type and heap allocation
fn demonstrate_string_type() {
    println!("2. The String Type and Heap Allocation");
    println!("   String is allocated on the heap and can grow\n");

    // String literal (immutable, on stack/binary)
    let s1 = "hello";
    println!("   String literal: {}", s1);

    // String type (mutable, on heap)
    let mut s2 = String::from("hello");
    println!("   String before: {}", s2);

    s2.push_str(", world!");
    println!("   String after push_str: {}", s2);

    println!("\n   Summary:");
    println!("   - String literals: immutable, fixed size, fast.");
    println!("   - String type: mutable, dynamic size, stored on heap.\n");
}

// 3. Move semantics
fn demonstrate_move_semantics() {
    println!("3. Move Semantics");
    println!("   Heap data is moved, not copied, to prevent double-free errors\n");

    // String (no Copy trait) - moved
    let s1 = String::from("hello");
    let s2 = s1; // s1's pointer, len, and capacity are moved into s2. s1 is invalidated.

    println!("   Strings (no Copy trait):");
    println!("   s2 = {}", s2);
    // println!("   s1 = {}", s1); // Error! borrow of moved value: `s1`
    println!("   s1 is no longer valid after move. This prevents memory bugs.\n");
}

// 4. Clone for deep copy
fn demonstrate_clone() {
    println!("4. Clone for Deep Copy");
    println!("   Use .clone() to explicitly duplicate heap data\n");

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Deep copy of heap data

    println!("   s1 = {}", s1);
    println!("   s2 = {}", s2);
    println!("   Both stay valid because heap memory was disconnected and copied.\n");
}

// 5. Copy trait for stack data
fn demonstrate_copy_trait() {
    println!("5. Copy Trait for Stack Data");
    println!("   Simple scalar types are copied, staying valid after assignment\n");

    // Types that implement Copy
    let x = 5; // i32
    let y = x;
    println!("   Integers: x = {}, y = {}", x, y);

    let b1 = true; // bool
    let b2 = b1;
    println!("   Booleans: b1 = {}, b2 = {}", b1, b2);

    let c1 = 'a'; // char
    let c2 = c1;
    println!("   Characters: c1 = {}, c2 = {}", c1, c2);

    let t1 = (1, 2); // tuple of Copy types
    let t2 = t1;
    println!("   Tuples: t1 = {:?}, t2 = {:?}", t1, t2);

    println!("\n   Types that implement 'Copy':");
    println!("   - Integers, Booleans, Floats, Chars.");
    println!("   - Tuples (if they only contain Copy types).\n");
}

// 6. Ownership and functions
fn demonstrate_ownership_and_functions() {
    println!("6. Ownership and Functions");
    println!("   Passing values to functions follows variable assignment rules\n");

    let s = String::from("hello");
    takes_ownership(s); // s's value moves into the function
                        // println!("{}", s);           // Error! s is gone.

    let x = 5;
    makes_copy(x); // x is copied into the function
    println!(
        "   After function calls: x ({}) is still here, but s is moved.",
        x
    );
}

fn takes_ownership(some_string: String) {
    println!("   [Function] takes ownership of: {}", some_string);
} // some_string goes out of scope and drop is called.

fn makes_copy(some_integer: i32) {
    println!("   [Function] makes a copy of: {}", some_integer);
}

// 7. Return values and ownership
fn demonstrate_return_values() {
    println!("\n7. Return Values and Ownership");
    println!("   Ownership can be transferred back to the caller\n");

    let s1 = gives_ownership(); // moves its return value into s1
    println!("   s1 (given): {}", s1);

    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 moves in, moves back into s3

    println!("   s3 (taken and returned): {}", s3);
    // println!("   s2 = {}", s2);        // Error! s2 was moved
    println!();
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // returns and moves ownership out
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // returns and moves ownership back out
}

// 8. Scope and assignment
fn demonstrate_scope_and_assignment() {
    println!("8. Scope and Assignment");
    println!("   Reassigning a variable drops the initial value immediately\n");

    let mut s = String::from("hello");
    println!("   Initial value: {}", s);

    s = String::from("ahoy"); // The memory for "hello" is freed right here
    println!("   After reassignment: {}\n", s);

    // Using tuple to return multiple values (the "pre-reference" workaround)
    let (s2, len) = calculate_length_with_tuple(s);
    println!("   Tuple workaround: String '{}' has length {}.", s2, len);
    println!();
}

fn calculate_length_with_tuple(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // Return both the String and its length to the caller
}

// --- Visualizing Drop ---

struct CustomData {
    name: String,
}

impl Drop for CustomData {
    fn drop(&mut self) {
        println!(
            "   [Memory] !!! Dropping CustomData: '{}' (Memory freed!)",
            self.name
        );
    }
}

fn demonstrate_drop_behavior() {
    println!("9. Visualizing the 'Drop' Trait");

    {
        let _data_a = CustomData {
            name: String::from("Component X"),
        };
        println!("   Component X was created inside a local block.");
    } // _data_a is dropped here

    println!("   We are now outside the block.");
    println!();
}
