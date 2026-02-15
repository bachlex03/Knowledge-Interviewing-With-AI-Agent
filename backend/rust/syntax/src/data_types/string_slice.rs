// Slices in Rust
// Based on: https://doc.rust-lang.org/stable/book/ch04-03-slices.html

/*
  A slice is a kind of reference, so it does not have ownership.
  Slices let you reference a contiguous sequence of elements in a collection.
*/

pub fn slice_examples() {
    println!("=== The Slice Type in Rust ===\n");

    // 1. String Slices Basics
    demonstrate_string_slices();

    // 2. Syntax Shortcuts (Range syntax)
    demonstrate_range_shortcuts();

    // 3. Solving the "Dangling Index" problem
    demonstrate_slice_safety();

    // 4. String Literals as Slices
    demonstrate_literals_as_slices();

    // 5. String Slices as Parameters (Idiomatic Rust)
    demonstrate_idiomatic_params();

    // 6. Other Slices (Arrays)
    demonstrate_array_slices();
}

fn demonstrate_string_slices() {
    println!("1. String Slices (&str)");
    println!("   A slice is a reference to a *portion* of a String.\n");

    let s = String::from("hello world");

    let hello = &s[0..5]; // Starts at index 0, ends before index 5
    let world = &s[6..11];

    println!("   Full string: '{}'", s);
    println!("   hello = '{}'", hello);
    println!("   world = '{}'\n", world);
}

fn demonstrate_range_shortcuts() {
    println!("2. Range Syntax Shortcuts");

    let s = String::from("hello");

    let _slice1 = &s[0..2];
    let _slice1_short = &s[..2]; // Equivalent to [0..2]

    let len = s.len();
    let _slice2 = &s[3..len];
    let _slice2_short = &s[3..]; // Equivalent to [3..len]

    let _slice_all = &s[..]; // Entire string

    println!(
        "   Rust allows dropping the first or last index in Range syntax (..) for convenience.\n"
    );
}

fn demonstrate_slice_safety() {
    println!("3. Slice Safety vs. Manual Indices");

    let mut s = String::from("hello world");
    let word = first_word(&s); // Returns a slice

    // s.clear(); // ERROR! s.clear() needs a mutable borrow, but 'word' is an immutable borrow.

    println!("   Manual indices can become invalid if the string is emptied.");
    println!("   Slices prevent this bug at compile-time by tying the slice to the source string.");
    println!("   The first word is: '{}'\n", word);
}

// Logic from the Rust Book
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn demonstrate_literals_as_slices() {
    println!("4. String Literals as Slices");

    // String literals ('&str') are already slices!
    // They point to a specific point in the binary (RODATA).
    let s = "Hello, world!";

    println!(
        "   The type of '{}' is &str because literals are slices of the binary.",
        s
    );
    println!();
}

fn demonstrate_idiomatic_params() {
    println!("5. Idiomatic Function Parameters");
    println!("   Using &str instead of &String makes APIs more flexible.\n");

    let my_string = String::from("hello world");

    // first_word works on slices of String, partial or whole
    let _ = first_word(&my_string[0..6]);
    let _ = first_word(&my_string[..]);

    // It also works on references to Strings (due to deref coercion)
    let _ = first_word(&my_string);

    let my_string_literal = "hello world";
    // And it works on string literals directly
    let _ = first_word(my_string_literal);

    println!("   Functions taking &str can accept both Strings AND literals.");
}

fn demonstrate_array_slices() {
    println!("\n6. Other Slices (Arrays)");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // Type is &[i32]

    println!("   Array: {:?}", a);
    println!("   Slice [1..3]: {:?}", slice);
    println!("   Slices work on any contiguous collection, not just Strings.");
    println!();
}
