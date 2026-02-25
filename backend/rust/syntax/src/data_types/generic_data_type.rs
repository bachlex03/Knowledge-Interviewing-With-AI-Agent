fn main() {
    println!("=== Generic Data Types in Rust ===\n");

    // 1. In Function Definitions
    demonstrate_generic_functions();

    // 2. In Struct Definitions
    demonstrate_generic_structs();

    // 3. In Enum Definitions
    demonstrate_generic_enums();

    // 4. In Method Definitions
    demonstrate_generic_methods();

    // 5. Performance (Monomorphization)
    demonstrate_performance();
}

// 1. In Function Definitions
fn demonstrate_generic_functions() {
    println!("1. Generic Function Definitions");
    println!("   Functions can use generic types to handle multiple concrete types\n");

    // We use trait bounds (PartialOrd) to allow comparison, which is needed for 'largest'
    // This is a common pattern when working with generics.
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("   The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("   The largest char is {}", result);
    println!();
}

// 2. In Struct Definitions
fn demonstrate_generic_structs() {
    println!("2. Generic Struct Definitions");
    println!("   Structs can hold fields with generic types\n");

    // Single generic type (x and y must be the same type)
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("   Single Type Generic Point:");
    println!(
        "   integer_point: x={}, y={}",
        integer_point.x, integer_point.y
    );
    println!("   float_point:   x={}, y={}", float_point.x, float_point.y);

    // Multiple generic types (x and y can be different types)
    struct MultiPoint<T, U> {
        x: T,
        y: U,
    }

    let mixed_point = MultiPoint { x: 5, y: 4.5 };
    println!("\n   Multiple Type Generic Point:");
    println!("   mixed_point: x={}, y={}", mixed_point.x, mixed_point.y);

    println!();
}

// 3. In Enum Definitions
fn demonstrate_generic_enums() {
    println!("3. Generic Enum Definitions");
    println!("   Enums like Option and Result are powered by generics\n");

    // Abstract concept of an optional value
    #[allow(dead_code)]
    enum MyOption<T> {
        Some(T),
        None,
    }

    // Abstract concept of an operation that can succeed or fail
    #[allow(dead_code)]
    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }

    let x: Option<i32> = Some(5);
    let y: Option<f64> = Some(5.0);

    println!("   Standard Option<T> instances:");
    println!("   x: {:?}", x);
    println!("   y: {:?}", y);

    println!();
}

// 4. In Method Definitions
fn demonstrate_generic_methods() {
    println!("4. Generic Method Definitions");
    println!("   Methods can use the struct's generics or define their own\n");

    struct Point<T> {
        x: T,
        y: T,
    }

    // Declaring T after 'impl' to implementation on Point<T>
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // Implementation specifically for Point<f32>
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 5, y: 10 };
    println!("   p.x() = {}", p.x());

    let p_float = Point { x: 3.0, y: 4.0 };
    println!("   p_float distance: {}", p_float.distance_from_origin());

    // Mixup example: Different generics for struct vs method
    struct NewPoint<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> NewPoint<X1, Y1> {
        fn mixup<X2, Y2>(self, other: NewPoint<X2, Y2>) -> NewPoint<X1, Y2> {
            NewPoint {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = NewPoint { x: 5, y: 10.4 };
    let p2 = NewPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("\n   Mixup Method Example:");
    println!("   p3.x = {}, p3.y = '{}'", p3.x, p3.y);

    println!();
}

// 5. Performance (Monomorphization)
fn demonstrate_performance() {
    println!("5. Performance of Code Using Generics");
    println!("   Rust uses 'Monomorphization' to ensure no runtime cost\n");

    println!("   The compiler expands generic code into concrete versions at compile time.");
    println!("   Example: Option<i32> and Option<f64> become two distinct generated types.");
    println!("   This means generic code runs just as fast as hand-coded specialized versions.");
    println!();
}
