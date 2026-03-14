# Bloom's Taxonomy: Rust Traits

### Level 1: Remembering
en: What is a trait in Rust, and how is it defined?
vi: Trait trong Rust là gì, và nó được định nghĩa như thế nào?

**Answer:** 
en: A trait in Rust is a collection of methods defined for an unknown type: `Self`. They can access other methods declared in the same trait. Traits define shared behavior that different types can implement. It is defined using the `trait` keyword followed by the trait name and a block containing method signatures.
vi: Trait (đặc điểm/hành vi) trong Rust là một tập hợp các phương thức được định nghĩa cho một kiểu chưa biết: `Self`. Chúng có thể truy cập các phương thức khác được khai báo trong cùng một trait. Các trait định nghĩa các hành vi chung (shared behavior) mà các kiểu dữ liệu khác nhau có thể tự triển khai (implement). Nó được định nghĩa bằng cách sử dụng từ khóa `trait` theo sau là tên trait và một khối chứa các chữ ký phương thức (method signatures).

---
**Question 2:**
en: In Rust, can a single struct implement more than one trait?
vi: Trong Rust, một struct có thể triển khai (implement) nhiều hơn một trait không?

**Answer:** 
en: Yes, a single type can implement any number of traits as long as the orphan rule is respected. For example, a `Book` struct can safely implement both a `Summary` trait and a `GetInfo` trait using separate `impl` blocks.
vi: Có, một kiểu dữ liệu có thể triển khai bất kỳ số lượng trait nào miễn là tuân thủ quy tắc mồ côi (orphan rule). Ví dụ, một struct `Book` có thể triển khai cả trait `Summary` và trait `GetInfo` một cách an toàn bằng cách sử dụng các khối `impl` riêng biệt.

- **orphan rule**: 
  - en: A rule stating you can only implement a trait for a type if either the trait or the type is defined in your current crate.
  - vi: Quy tắc quy định bạn chỉ có thể triển khai bất kì trait nào cho một kiểu dữ liệu nếu trait hoặc kiểu dữ liệu đó được định nghĩa trong crate hiện tại của bạn.

### Level 2: Understanding
en: Explain the difference between implementing a required method and using a default method in a Rust trait.
vi: Giải thích sự khác biệt giữa việc triển khai một phương thức bắt buộc (required method) và sử dụng một phương thức mặc định (default method) trong một trait của Rust.

**Answer:** 
en: A required method only provides the method signature within the trait definition; any type implementing this trait *must* provide its own concrete implementation for that method. A default method provides a concrete implementation within the trait itself. A type implementing the trait can choose to use this default behavior without writing additional code, or it can override it with its own custom implementation.
vi: Một phương thức bắt buộc chỉ cung cấp chữ ký phương thức bên trong phần định nghĩa trait; bất kỳ kiểu dữ liệu nào triển khai trait này *bắt buộc* phải cung cấp phần thực thi cụ thể của riêng nó cho phương thức đó. Một phương thức mặc định cung cấp sẵn một phần thực thi thực tế ngay bên trong trait. Một kiểu dữ liệu khi triển khai trait này có thể chọn sử dụng luôn hành vi mặc định này mà không cần viết thêm mã, hoặc nó có thể ghi đè (override) bằng phần thực thi tùy chỉnh của riêng mình.

---
**Question 2:**
en: If a trait defines a method with a default behavior (e.g., `fn summarize(&self) {}`), what happens when a struct implementing this trait provides its own `summarize()` method inside the `impl` block?
vi: Nếu một trait định nghĩa một phương thức với hành vi mặc định (ví dụ: `fn summarize(&self) {}`), điều gì sẽ xảy ra khi một struct triển khai trait này nhưng lại cung cấp phương thức `summarize()` của riêng nó bên trong khối `impl`?

**Answer:** 
en: The struct's specific implementation overrides the trait's default behavior. When the method is called on an instance of that struct, the custom implementation is executed.
vi: Phần triển khai cụ thể của struct sẽ ghi đè (override) hành vi mặc định của trait. Khi phương thức đó được gọi trên một thể hiện của struct, phần thực thi tùy chỉnh đó sẽ được chạy thay vì phần mặc định.

### Level 3: Applying
en: Given a struct `Laptop` and a struct `Smartphone`, how would you implement a `Rechargeable` trait that has a `get_battery_level()` method for both of these structs?
vi: Cho một struct `Laptop` và một struct `Smartphone`, làm thế nào bạn có thể triển khai một trait `Rechargeable` có chứa một phương thức `get_battery_level()` cho cả hai struct này?

**Answer:** 
en: First, define the trait: `trait Rechargeable { fn get_battery_level(&self) -> u8; }`. Then, implement it for `Laptop` using the `impl` block: `impl Rechargeable for Laptop { fn get_battery_level(&self) -> u8 { self.battery_percentage } }`. Next, do the same for `Smartphone`: `impl Rechargeable for Smartphone { fn get_battery_level(&self) -> u8 { self.current_battery } }`.
vi: Đầu tiên, định nghĩa trait: `trait Rechargeable { fn get_battery_level(&self) -> u8; }`. Sau đó, triển khai (implement) nó cho `Laptop` bằng cách sử dụng khối lệnh `impl`: `impl Rechargeable for Laptop { fn get_battery_level(&self) -> u8 { self.battery_percentage } }`. Tiếp theo, làm tương tự cho `Smartphone`: `impl Rechargeable for Smartphone { fn get_battery_level(&self) -> u8 { self.current_battery } }`.

```rust
struct Laptop {
    battery_percentage: u8,
}

struct Smartphone {
    current_battery: u8,
}

trait Rechargeable {
    fn get_battery_level(&self) -> u8;
}

impl Rechargeable for Laptop {
    fn get_battery_level(&self) -> u8 {
        self.battery_percentage
    }
}

impl Rechargeable for Smartphone {
    fn get_battery_level(&self) -> u8 {
        self.current_battery
    }
}
```

---
**Question 2:**
en: Given a struct `Book` and two traits, `Summary` and `GetInfo`, write the code to implement both traits for `Book`.
vi: Cho một struct `Book` và hai trait là `Summary` và `GetInfo`, hãy viết mã để triển khai cả hai trait này cho `Book`.

**Answer:** 
en: You must use separate `impl` blocks for each trait that you want to implement for a given type.
vi: Bạn phải sử dụng các khối `impl` tách biệt nhau cho mỗi trait mà bạn muốn triển khai trên một kiểu dữ liệu.

```rust
pub struct Book {
    title: String,
    author: String,
    content: String,
}

pub trait Summary {
    fn summarize(&self);
}

pub trait GetInfo {
    fn get_title(&self) -> &str;
}

// Implement first trait
impl Summary for Book {
    fn summarize(&self) {
        println!("{} by {}", self.title, self.author);
    }
}

// Implement second trait
impl GetInfo for Book {
    fn get_title(&self) -> &str {
        &self.title
    }
}
```

### Level 4: Analyzing
en: Compare and contrast "Trait Bounds" with dynamic dispatch using "Trait Objects" (`dyn Trait`). When is it more appropriate to use one over the the other?
vi: So sánh và đối chiếu "Trait Bounds" với dynamic dispatch sử dụng "Trait Objects" (`dyn Trait`). Khi nào thì việc sử dụng cái này phù hợp hơn cái kia?

**Answer:** 
en: Trait bounds (e.g., `fn print_it<T: Display>(item: &T)`) use static dispatch. The compiler generates specialized copies of the function for each concrete type used at compile time (monomorphization), resulting in faster execution but larger binary sizes. Trait objects (e.g., `fn print_it(item: &dyn Display)`) use dynamic dispatch via vtables at runtime. This allows for heterogeneous collections (like a `Vec<Box<dyn Display>>`) and smaller binary sizes, but introduces a slight runtime performance penalty due to pointer indirection. Use trait bounds for performance-critical generic code, and trait objects when you need flexibility, such as storing different types in the same collection.
vi: Trait bounds (ví dụ: `fn print_it<T: Display>(item: &T)`) sử dụng static dispatch (điều phối tĩnh). Trình biên dịch tạo ra các bản sao chuyên biệt của hàm cho mỗi kiểu dữ liệu cụ thể được sử dụng tại thời điểm biên dịch (monomorphization), dẫn đến quá trình thực thi nhanh hơn nhưng kích thước tệp nhị phân lớn hơn. Trait objects (ví dụ: `fn print_it(item: &dyn Display)`) sử dụng dynamic dispatch (điều phối động) thông qua vtables (bảng phương thức ảo) tại thời điểm chạy (runtime). Điều này cho phép tạo các bộ sưu tập không đồng nhất (ví dụ: `Vec<Box<dyn Display>>`) và tệp nhị phân nhỏ hơn, nhưng gây ra một chút độ trễ hiệu suất nhỏ do xử lý gián tiếp qua con trỏ. Sử dụng trait bounds cho code generic ưu tiên hiệu suất, và trait objects khi bạn cần sự linh hoạt, chẳng hạn như lưu trữ các kiểu dữ liệu khác nhau trong cùng một collection.

```rust
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
    let text = "Hello";
    
    // Static dispatch
    print_static(&number);
    print_static(&text);
    
    // Dynamic dispatch - heterogeneous collection
    let mixed_collection: Vec<Box<dyn Display>> = vec![
        Box::new(number),
        Box::new(text),
    ];
    
    for item in mixed_collection {
        print_dynamic(&*item);
    }
}
```

---
**Question 2:**
en: Analyze the code below containing a `Book` struct and a `Summary` trait. Is it valid Rust code? If not, explain the compilation error and how to fix it.
vi: Hãy phân tích đoạn mã dưới đây chứa struct `Book` và trait `Summary`. Đoạn mã này có hợp lệ trong Rust không? Nếu không, hãy giải thích lỗi biên dịch và làm thế nào để khắc phục.

```rust
pub trait Summary {
    fn summarize(&self) {}
}

pub struct Book { pub title: String }

impl Summary for Book {
    fn summarize(&self) { println!("{}", self.title); }
    fn summarize_details(&self) { println!("Details: {}", self.title); }
}
```

**Answer:** 
en: The code is invalid and will not compile. In Rust, you cannot define new methods (like `summarize_details`) inside an `impl Trait for Type` block if they are not explicitly declared in the trait's definition (`Summary`). To fix it, `summarize_details` must be placed in a separate, inherent implementation block just for the struct itself.
vi: Đoạn mã này không hợp lệ và sẽ không biên dịch được. Trong Rust, bạn không thể định nghĩa các phương thức mới (như `summarize_details`) bên trong khối lệnh `impl Trait for Type` nếu chúng không được khai báo một cách công khai trong phần định nghĩa của trait đó (`Summary`). Để sửa lỗi, `summarize_details` bắt buộc phải được đặt vào một khối triển khai nội tại (inherent block) riêng rẽ chỉ dành cho struct đó.

```rust
// Correct way to fix the error:
impl Summary for Book {
    fn summarize(&self) { println!("{}", self.title); }
}

// Inherent implementation block for non-trait methods
impl Book {
    fn summarize_details(&self) { 
        self.summarize();
        println!("Details: {}", self.title); 
    }
}
```

### Level 5: Evaluating
en: Evaluate the safety and architectural implications of the "Orphan Rule" in Rust. Do you think restricting the implementation of external traits on external types is a necessary limitation, or is it overly restrictive? Defend your answer.
vi: Đánh giá những ý nghĩa về tính an toàn và mặt kiến trúc của "Orphan Rule" (Quy tắc mồ côi) trong Rust. Bạn nghĩ việc hạn chế việc triển khai các external trait (trait bên ngoài) trên các external type (kiểu dữ liệu bên ngoài) là một sự giới hạn cần thiết, hay nó quá khắt khe? Hãy biện luận cho câu trả lời của bạn.

**Answer:** 
en: The Orphan Rule states that you can only implement a trait for a type if either the trait or the type are defined within your local crate. This is a crucial foundation for Rust's safety and stability. If anyone could implement any trait for any external type (e.g., someone writing `impl Display for String` in their own library), it would cause severe compilation conflicts if two separate crates provided different implementations for the exact same trait/type combination. While it can sometimes feel overly restrictive (forcing developers to use the Newtype pattern to wrap external types), it absolutely prevents "dependency hell" and ensures reasoning about code behavior remains local and predictable. It trades a small amount of convenience for massive, system-wide architectural stability.
vi: "Orphan Rule" quy định rằng bạn chỉ có thể triển khai một trait cho một kiểu dữ liệu nếu trait đó hoặc kiểu dữ liệu đó được định nghĩa bên trong crate nội bộ (local crate) của bạn. Đây là nền tảng vô cùng quan trọng cho sự an toàn và ổn định của Rust. Nếu ai cũng có thể triển khai bất kỳ trait nào cho bất kỳ kiểu dữ liệu bên ngoài nào (ví dụ: ai đó tự viết `impl Display for String` trong thư viện của họ), nó có thể gây ra xung đột biên dịch nghiêm trọng nếu hai crate riêng biệt cùng cung cấp hai cách triển khai riêng cho cùng một cặp trait/kiểu dữ liệu. Mặc dù đôi khi nó có vẻ quá khắt khe (buộc các nhà phát triển phải sử dụng pattern "Newtype" để dể bao bọc (wrap) kiểu dữ liệu bên ngoài), nhưng điều này hoàn toàn ngăn chặn được tình trạng "địa ngục dependency" và đảm bảo rằng việc suy luận về hành vi của code luôn rõ ràng và dễ dự đoán trong phạm vi cục bộ. Nó trade-off (đánh đổi) một chút sự tiện lợi để đổi lấy sự ổn định khổng lồ cho kiến trúc trên toàn hệ thống.

---
**Question 2:**
en: Evaluate the design pattern of defining many small traits (like `Summary` and `GetInfo`) versus one large, monolithic trait (e.g., `BookOperations`) for a type like `Book`. Which approach is preferred in Rust and why?
vi: Đánh giá mẫu thiết kế của việc định nghĩa nhiều trait nhỏ (như `Summary` và `GetInfo`) so với một trait khổng lồ duy nhất (chẳng hạn như `BookOperations`) cho một kiểu như `Book`. Hướng tiếp cận nào được ưu tiên hơn trong Rust và tại sao?

**Answer:** 
en: Rust strongly prefers many small, specific traits, aligning with the Interface Segregation Principle from SOLID. It allows types to implement only necessary behaviors and enables granular trait bounds in generic functions (e.g., demanding `T: Summary` without caring if it also has `GetInfo`). A large, monolithic trait leads to rigid systems where implementers are forced to write dummy methods or panic handlers for functionalities they don't actually need.
vi: Rust rất ưu tiên việc thiết kế nhiều trait nhỏ gọn và cụ thể, rất phù hợp với Nguyên tắc Phân tách Giao diện (Interface Segregation Principle) trong SOLID. Điều này cho phép các kiểu dữ liệu chỉ cần triển khai những hành vi thực sự cần thiết, đồng thời giúp tạo ra các ràng buộc trait (trait bounds) chi tiết hơn trong generic function (ví dụ: chỉ yêu cầu `T: Summary` mà không cần biết đến `GetInfo`). Một trait khổng lồ sẽ dẫn đến một hệ thống cứng nhắc, nơi các implementer bị buộc phải viết các hàm rỗng hoặc xử lý lỗi vớ vẩn cho những phương thức mà họ thực sự không cần đến.
