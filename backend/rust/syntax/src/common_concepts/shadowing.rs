// docs ref: `https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing`

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
