// References and Borrowing in Rust
// Based on: https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html

/*
  Rules of References:
  1. At any given time, you can have either:
     - One mutable reference (&mut T)
     - OR any number of immutable references (&T)
  2. References must always be valid (preventing dangling pointers).
*/

pub fn reference_examples() {
    println!("=== References & Borrowing in Rust ===\n");

    // 1. Immutable Borrowing (Multiple Readers)
    demonstrate_immutable_borrow();

    // 2. Mutable Borrowing (Single Writer)
    demonstrate_mutable_borrow();

    // 3. Data Race Protection (The Restriction)
    demonstrate_borrowing_restrictions();

    // 4. Non-Lexical Lifetimes (NLL Optimization)
    demonstrate_nll_behavior();

    // 5. Memory Safety: Dangling Reference Prevention
    demonstrate_dangling_prevention();
}

fn demonstrate_immutable_borrow() {
    println!("1. Immutable Borrowing");
    println!("   References (&T) allow you to read data without owning it.\n");

    let s1 = String::from("hello");

    // Passing a reference (&s1)
    let len = calculate_len(&s1);

    println!("   The length of '{}' is {}.", s1, len);
    println!("   [Status] s1 is still valid because we 'borrowed' but did not 'move' it.\n");
}

fn calculate_len(s: &String) -> usize {
    s.len()
} // Here s goes out of scope, but since it doesn't own the data, nothing is dropped.

fn demonstrate_mutable_borrow() {
    println!("2. Mutable Borrowing");
    println!("   Mutable references (&mut T) allow you to modify borrowed data.\n");

    let mut s = String::from("hello");
    println!("   Before: '{}'", s);

    change(&mut s);

    println!("   After : '{}'\n", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn demonstrate_borrowing_restrictions() {
    println!("3. Data Race Protection");
    println!("   Rust prevents data races by forbidding simultaneous mutable access.\n");

    let mut s = String::from("hello");

    println!("   [Rule] You cannot have two mutable references at the same time.");

    let r1 = &mut s;
    // let r2 = &mut s; // ERROR: cannot borrow `s` as mutable more than once

    println!("   Reference r1 is active here: '{}'", r1);
    println!("   Any attempt to create r2 here would fail at compile time.");

    println!("\n   [Rule] You cannot mix immutable and mutable references.");
    let test = String::from("test");
    let _r_immut = &test;
    // let _r_mut = &mut test; // ERROR: cannot borrow as mutable because it is already borrowed as immutable

    println!(
        "   This ensures that while someone is reading, others don't change the data underneath.\n"
    );
}

fn demonstrate_nll_behavior() {
    println!("4. Non-Lexical Lifetimes (NLL)");
    println!("   Modern Rust ends a reference's lifetime at its last usage point.\n");

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("   r1 and r2 used here: {} and {}", r1, r2);

    // After the print above, r1 and r2 are no longer used.
    // Rust allows r3 to start even though r1 and r2 are still in the function scope!

    let r3 = &mut s;
    r3.push_str(" (Mutated)");

    println!("   r3 is now allowed: '{}'", r3);
    println!();
}

fn demonstrate_dangling_prevention() {
    println!("5. Dangling Reference Prevention");
    println!("   Rust guarantees that references will never point to invalid memory.\n");

    // let reference_to_nothing = dangle();
    println!("   [Safety] If a function tries to return a reference to local data,");
    println!("   the compiler blocks it because local data is 'dropped' when the function ends.");
    println!();
}

/*
// This code will NOT compile:
fn dangle() -> &String {
    let s = String::from("hello");
    &s // ERROR: s is dropped here! The reference would point to garbage memory.
}
*/
