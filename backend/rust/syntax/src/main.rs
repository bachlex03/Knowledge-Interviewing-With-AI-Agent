fn main() {
    // Using guards with multiple patterns
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("   x is 4, 5, or 6 AND y is true"),
        _ => println!("   Other case"),
    }
    println!();
}
