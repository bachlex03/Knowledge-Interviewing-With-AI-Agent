fn main() {
    println!("=== Structs and Custom Data Types in Rust ===\n");

    // 1. Defining and Instantiating Structs
    demonstrate_defining_instantiating();

    // 2. Field Init Shorthand
    demonstrate_field_init_shorthand();

    // 3. Struct Update Syntax
    demonstrate_struct_update_syntax();

    // 4. Tuple Structs
    demonstrate_tuple_structs();

    // 5. Unit-Like Structs
    demonstrate_unit_like_structs();

    // 6. Methods and Associated Functions
    demonstrate_methods_and_associated_functions();
}

// 1. Defining and Instantiating Structs
fn demonstrate_defining_instantiating() {
    println!("1. Defining and Instantiating Structs");
    println!("   Structs name each piece of data to clarify their meaning\n");

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // Creating an instance
    let mut user1 = User {
        active: true,
        username: String::from("alice_vec"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
    };

    println!("   User name: {}", user1.username);

    // Modifying a field (instance must be mutable)
    user1.email = String::from("alice.new@example.com");
    println!("   Updated email: {}", user1.email);
    println!();
}

// 2. Field Init Shorthand
fn demonstrate_field_init_shorthand() {
    println!("2. Field Init Shorthand");
    println!("   Avoid repetition when parameter and field names match\n");

    struct User {
        username: String,
        email: String,
    }

    fn build_user(email: String, username: String) -> User {
        User {
            email,    // Shorthand for email: email
            username, // Shorthand for username: username
        }
    }

    let user = build_user(String::from("bob@example.com"), String::from("bob_smith"));
    println!("   Built user: {} ({})", user.username, user.email);
    println!();
}

// 3. Struct Update Syntax
fn demonstrate_struct_update_syntax() {
    println!("3. Struct Update Syntax");
    println!("   Create a new instance using values from another instance\n");

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        active: true,
        username: String::from("charlie"),
        email: String::from("charlie@example.com"),
        sign_in_count: 1,
    };

    // Use .. to fill remaining fields from user1
    let user2 = User {
        email: String::from("charlie.work@example.com"),
        ..user1
    };

    println!("   User2 shares active={} from User1", user2.active);
    println!("   Note: user1.username was MOVED to user2, so user1 is now invalid!");
    println!();
}

// 4. Tuple Structs
fn demonstrate_tuple_structs() {
    println!("4. Tuple Structs");
    println!("   Structs that look like tuples but have a specific type name\n");

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // black and origin are different types despite having same field types
    println!("   Color values: {}, {}, {}", black.0, black.1, black.2);
    println!("   Point values: {}, {}, {}", origin.0, origin.1, origin.2);
    println!();
}

// 5. Unit-Like Structs
fn demonstrate_unit_like_structs() {
    println!("5. Unit-Like Structs");
    println!("   Structs without any fields, useful for implementing traits\n");

    struct AlwaysEqual;

    let _subject = AlwaysEqual;
    println!("   Created AlwaysEqual unit struct instance");
    println!();
}

// 6. Methods and Associated Functions
fn demonstrate_methods_and_associated_functions() {
    println!("6. Methods and Associated Functions");
    println!("   Define behavior in 'impl' blocks\n");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // Method: takes &self
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // Method with extra parameter
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        // Associated Function (Constructor): doesn't take self
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let sq = Rectangle::square(20);

    println!("   Rectangle 1: {:?}", rect1);
    println!("   Area: {}", rect1.area());
    println!("   Can hold rect2? {}", rect1.can_hold(&rect2));
    println!("   Created square: {:?}", sq);
    println!();
}
