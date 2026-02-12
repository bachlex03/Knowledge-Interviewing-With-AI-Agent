fn main() {
    println!("--- String Methods ---");

    let s = String::from("  Hello, Rust!  ");

    // Length
    println!("len(): {}", s.len());
    println!("is_empty(): {}", s.is_empty());

    // Trimming
    println!("trim(): '{}'", s.trim());
    println!("trim_start(): '{}'", s.trim_start());
    println!("trim_end(): '{}'", s.trim_end());

    // Case conversion
    let s2 = String::from("Hello World");
    println!("to_lowercase(): {}", s2.to_lowercase());
    println!("to_uppercase(): {}", s2.to_uppercase());

    // Checking content
    let s3 = String::from("Hello, Rust!");
    println!("contains('Rust'): {}", s3.contains("Rust"));
    println!("starts_with('Hello'): {}", s3.starts_with("Hello"));
    println!("ends_with('!'): {}", s3.ends_with("!"));

    // Finding
    println!("find('Rust'): {:?}", s3.find("Rust"));

    // Splitting
    let s4 = String::from("apple,banana,cherry");
    let parts: Vec<&str> = s4.split(',').collect();
    println!("split result: {:?}", parts);

    // Repeat
    let s5 = "Ha".repeat(3);
    println!("repeat(3): {}", s5);

    // Checking ASCII
    let s6 = String::from("Hello");
    let s7 = String::from("Hello 世界");
    println!("'{}' is_ascii(): {}", s6, s6.is_ascii());
    println!("'{}' is_ascii(): {}", s7, s7.is_ascii());

    println!();
}
