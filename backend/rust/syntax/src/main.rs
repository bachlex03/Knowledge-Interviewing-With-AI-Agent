use std::fmt::Display;

// Using Trait Bounds (Static Dispatch)
fn print_static<T: Display>(item: &T) {
    println!("{}", item);
}

// Using Trait Objects (Dynamic Dispatch)
fn print_dynamic(item: &dyn Display) {
    println!("{}", item);
}

fn main() {
    let number = 42;
    let string = "hello";

    print_static(&number);
}
