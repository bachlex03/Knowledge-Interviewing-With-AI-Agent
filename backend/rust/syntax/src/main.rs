fn main() {
    //generic_data_type
    //1. In Function Definitions
    // 2. In Struct Definitions
    // 3. In Enum Definitions
    // 4. In Method Definitions

    // 1
    fn largest<T: PartialOrd>(arr: &[i32]) -> bool {
        true
    }

    // 2

    pub struct AStruct<T, U> {
        x: T,
        Y: U,
    }

    // 3

    pub enum AEnum<T> {
        Some(T),
        None,
    }

    // 4
    impl<T, U> AStruct<T, U> {
        fn get_x(&self) -> &T {
            return &self.x;
        }
    }
fn print_number_ref(number: &i32) {
    println!("The number in ref function is: {}", number);
}

fn print_number_value(number: i32) {
    println!("The number in value function is: {}", number);
}

fn print_number_value_mut(mut number: i32) {
    number = number + 3;
    println!("The number in mut value function is: {}", number);
}

fn print_number_value_mut_ref(number: &mut i32) {
    *number = *number + 1;
    println!("The number in mut ref function is: {}", number);
}

fn print_slice_ref(slice: &str) {
    println!("The slice is: {}", slice);
}

fn print_slice_mut(slice: &mut str) {
    slice.make_ascii_uppercase();
    // string = string + "word"; // Error: cannot add `&str` to `&mut str`
    // string.push_str(" word"); // Error: no method named `push_str` found for mutable reference `&mut str` in the current scope
    if slice.len() == 5 {
        let bytes = unsafe { slice.as_bytes_mut() };
        bytes[0] = b'w';
        bytes[1] = b'o';
        bytes[2] = b'r';
        bytes[3] = b'l';
        bytes[4] = b'd';
    }
    println!("The modified slice is: {}", slice);
}

fn print_string_value(string: String) {
    println!("The string is: {}", string);
}

fn print_string_value_mut(mut string: String) {
    string.push_str(" world");
    println!("The string is: {}", string);
}

fn print_string_value_mut_return(mut string: String) -> String {
    string.push_str(" world");
    println!("The string is: {}", string);
    return string;
}

fn print_string_value_ref(string: &String) {
    println!("The string is: {}", string);
}

fn print_string_value_mut_ref(string: &mut String) {
    // string = string + "word"; // Error: cannot add `&str` to `&mut String`
    string.push_str(" world");
    println!("The string is: {}", string);
}

fn main() {
    let mut number: i32 = 42;

    print_number_ref(&number);
    print_number_value(number);
    print_number_value_mut(number);

    let new_num = number + 5;
    println!("The new number is: {}", new_num);

    print_number_value_mut_ref(&mut number);

    println!("The old number is: {}", number);

    let mut string = String::from("hello");

    print_string(&string);

    let new_string = string + " world";

    println!("The new string is: {}", new_string);
}
