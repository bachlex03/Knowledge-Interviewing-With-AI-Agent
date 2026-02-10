fn main() {
    println!("6. Working with Option");
    println!("   Must handle both Some and None cases\n");

    // Example 1: Using match
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("   Using match:");
    println!("   plus_one(Some(5)) = {:?}", six);
    println!("   plus_one(None) = {:?}", none);

    // Example 2: Using if let
    println!("\n   Using if let:");
    if let Some(value) = six {
        println!("   Got value: {}", value);
    } else {
        println!("   Got None");
    }

    // Example 3: Using unwrap_or
    println!("\n   Using unwrap_or (provide default):");
    let some_value = Some(10);
    let no_value: Option<i32> = None;

    println!("   Some(10).unwrap_or(0) = {}", some_value.unwrap_or(0));
    println!("   None.unwrap_or(0) = {}", no_value.unwrap_or(0));

    // Example 4: Using map
    println!("\n   Using map (transform the value):");
    let some_string = Some("hello");
    let string_length = some_string.map(|s| s.len());
    println!("   Some(\"hello\").map(|s| s.len()) = {:?}", string_length);

    // Example 5: Real-world example
    println!("\n   Real-world example - finding an item:");
    fn find_item(items: &[&str], search: &str) -> Option<usize> {
        for (index, &item) in items.iter().enumerate() {
            if item == search {
                return Some(index);
            }
        }
        None
    }

    let items = ["apple", "banana", "cherry"];
    match find_item(&items, "banana") {
        Some(index) => println!("   Found 'banana' at index {}", index),
        None => println!("   'banana' not found"),
    }

    match find_item(&items, "orange") {
        Some(index) => println!("   Found 'orange' at index {}", index),
        None => println!("   'orange' not found"),
    }

    println!();
}
