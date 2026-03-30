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
}
