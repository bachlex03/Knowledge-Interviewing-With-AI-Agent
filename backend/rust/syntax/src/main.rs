fn main() {
    let mut x: u8 = u8::MAX;

    x = x.checked_add(1); // suppose return None

    println!("The value of x is: {x}");
}
