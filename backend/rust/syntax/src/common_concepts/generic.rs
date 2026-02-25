use std::fmt::{Debug, Display};
use std::marker::PhantomData;

fn main() {
    println!("=== Generics in Rust (Rust By Example) ===\n");

    // 1. Basic Generics (Syntax, Structs, Functions)
    demonstrate_basic_generics();

    // 2. Generic Implementations
    demonstrate_generic_impl();

    // 3. Generic Traits
    demonstrate_generic_traits();

    // 4. Bounds and Multiple Bounds
    demonstrate_bounds();

    // 5. Where Clauses
    demonstrate_where_clauses();

    // 6. New Type Idiom
    demonstrate_new_type_idiom();

    // 7. Associated Items (Types)
    demonstrate_associated_types();

    // 8. Phantom Type Parameters
    demonstrate_phantom_types();
}

// 1. Basic Generics
fn demonstrate_basic_generics() {
    println!("1. Basic Generics");
    println!("   Type parameters use angle brackets <T>\n");

    struct A; // Concrete type
    struct Single(A); // Concrete struct using A
    struct SingleGen<T>(T); // Generic struct over T

    let _s = Single(A);
    let _char = SingleGen('a'); // Implicitly char
    let _i32: SingleGen<i32> = SingleGen(6); // Explicitly i32

    // Functions
    fn generic<T>(_s: SingleGen<T>) {
        println!("   Called generic function");
    }

    generic(_char);
    generic::<i32>(_i32); // Turbofish syntax
    println!();
}

// 2. Generic Implementations
fn demonstrate_generic_impl() {
    println!("2. Generic Implementations");
    println!("   Declaring <T> after 'impl' makes the implementation generic\n");

    struct GenericVal<T>(T);

    impl<T> GenericVal<T> {
        fn value(&self) -> &T {
            &self.0
        }
    }

    let y = GenericVal(3.14f64);
    println!("   Value: {}", y.value());
    println!();
}

// 3. Generic Traits
fn demonstrate_generic_traits() {
    println!("3. Generic Traits");
    println!("   Traits can be generic over types\n");

    struct Empty;
    struct Null;

    trait DoubleDrop<T> {
        fn double_drop(self, _: T);
    }

    // Implement DoubleDrop<T> for any type U
    impl<T, U> DoubleDrop<T> for U {
        fn double_drop(self, _: T) {
            println!("   Both items dropped!");
        }
    }

    let empty = Empty;
    let null = Null;
    empty.double_drop(null);
    println!();
}

// 4. Bounds and Multiple Bounds
fn demonstrate_bounds() {
    println!("4. Bounds and Multiple Bounds");
    println!("   Restricting generic types with traits using ':' and '+'\n");

    fn printer<T: Display>(t: T) {
        println!("   Display: {}", t);
    }

    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("   Debug:   {:?}", t);
        println!("   Display: {}", t);
    }

    printer("Hello Bounds");
    compare_prints(&"Double Bound");
    println!();
}

// 5. Where Clauses
fn demonstrate_where_clauses() {
    println!("5. Where Clauses");
    println!("   More expressive syntax for complex bounds\n");

    trait PrintInOption {
        fn print_in_option(self);
    }

    // Using where clause to apply bounds to a compound type Option<T>
    impl<T> PrintInOption for T
    where
        Option<T>: Debug,
    {
        fn print_in_option(self) {
            println!("   Option: {:?}", Some(self));
        }
    }

    let vec = vec![1, 2, 3];
    vec.print_in_option();
    println!();
}

// 6. New Type Idiom
fn demonstrate_new_type_idiom() {
    println!("6. New Type Idiom");
    println!("   Compile-time type safety for primitive wrappers\n");

    struct Years(i64);
    struct Days(i64);

    impl Years {
        fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }

    fn is_adult(age: &Years) -> bool {
        age.0 >= 18
    }

    let age = Years(25);
    println!("   Is adult (25 years)? {}", is_adult(&age));
    println!("   Days: {}", age.to_days().0);
    println!();
}

// 7. Associated Items (Types)
fn demonstrate_associated_types() {
    println!("7. Associated Types");
    println!("   Defining types inside traits for cleaner signatures\n");

    struct Container(i32, i32);

    trait Contains {
        type A;
        type B;

        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    }

    impl Contains for Container {
        type A = i32;
        type B = i32;

        fn contains(&self, n1: &i32, n2: &i32) -> bool {
            (&self.0 == n1) && (&self.1 == n2)
        }
    }

    let container = Container(3, 10);
    println!("   Contains 3 and 10? {}", container.contains(&3, &10));
    println!();
}

// 8. Phantom Type Parameters
fn demonstrate_phantom_types() {
    println!("8. Phantom Type Parameters");
    println!("   Types used only for compile-time markers, no runtime cost\n");

    // PhantomData is a zero-sized marker
    struct PhantomStruct<A, B> {
        first: A,
        _phantom: PhantomData<B>,
    }

    // Different markers mean different types even if data is the same
    #[derive(Debug)]
    struct Kg;
    #[derive(Debug)]
    struct Lb;

    let weight_kg: PhantomStruct<f32, Kg> = PhantomStruct {
        first: 70.0,
        _phantom: PhantomData,
    };

    let weight_lb: PhantomStruct<f32, Lb> = PhantomStruct {
        first: 154.3,
        _phantom: PhantomData,
    };

    println!("   Weight in Kg: {}", weight_kg.first);
    println!("   Weight in Lb: {}", weight_lb.first);
    // weight_kg == weight_lb; // COMPILATION ERROR: mismatching types
    println!();
}
