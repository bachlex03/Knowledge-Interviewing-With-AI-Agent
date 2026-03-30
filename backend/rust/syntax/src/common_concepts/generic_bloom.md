# Generics in Rust - Bloom's Taxonomy

## Level 1: Remembering

**Question 1:**
en: What syntax is used to define generic types in a Rust function or struct signature?
vi: Cú pháp nào được sử dụng để định nghĩa các kiểu tổng quát (generic types) trong chữ ký của một hàm hoặc struct trong Rust?

**Answer 1:**
en: Generics are defined using angle brackets, typically wrapping a single uppercase letter like `<T>`, placed right after the function or struct name.
vi: Generics được định nghĩa bằng cách sử dụng các dấu ngoặc nhọn, thường bao bọc một chữ cái viết hoa đơn như `<T>`, đặt ngay sau tên hàm hoặc tên struct.

**Question 2:**
en: Name two standard library constructs in Rust that inherently rely on generics.
vi: Hãy kể tên hai cấu trúc trong thư viện tiêu chuẩn của Rust mà vốn dĩ dựa nhiều vào generics.

**Answer 2:**
en: The `Option<T>` enum and the `Result<T, E>` enum, as well as vectors like `Vec<T>`.
vi: Enum `Option<T>` và enum `Result<T, E>`, cũng như các vector như `Vec<T>`.

---

## Level 2: Understanding

**Question 1:**
en: Explain how the Rust compiler handles generic code to avoid a runtime performance hit. What is the process called?
vi: Giải thích cách trình biên dịch Rust xử lý mã generic để tránh ảnh hưởng đến hiệu suất tại thời điểm chạy. Quá trình này được gọi là gì?

**Answer 1:**
en: Rust uses a process called **Monomorphization**. During compilation, the compiler analyzes what concrete types are actually used with the generic functions/structs. It then generates specific, non-generic versions of the code for each of those concrete types. Because the generics are expanded at compile time, there is no runtime overhead compared to writing concrete types manually.
vi: Rust sử dụng một quy trình gọi là **Monomorphization** (đơn hình hóa). Trong quá trình biên dịch, trình biên dịch phân tích xem những kiểu dữ liệu cụ thể nào thực sự được sử dụng với các hàm/struct generic. Sau đó, nó sinh ra các phiên bản mã cụ thể, không-generic cho từng kiểu cụ thể đó. Bởi vì các generic được mở rông tại thời điểm biên dịch, nên không có chi phí phát sinh nào ở thời điểm chạy so với việc tự tay viết các kiểu cụ thể.

**Question 2:**
en: How do generic lifetimes (`<'a>`) relate to generic types (`<T>`) in Rust?
vi: Vòng đời tổng quát (`<'a>`) liên quan như thế nào đến các kiểu tổng quát (`<T>`) trong Rust?

**Answer 2:**
en: Lifetimes are essentially just a specialized kind of generic parameter. While `<T>` specifies that a type can be anything matching given constraints, `<'a>` specifies that the generic lifetime parameter can be any lifetime matching the compiler's borrow constraints. Both are declared inside the same angle brackets, e.g., `<'a, T>`.
vi: Lifetimes về cơ bản chỉ là một dạng đặc biệt của tham số generic. Trong khi `<T>` chỉ định rằng một kiểu có thể là bất kỳ thứ gì khớp với các ràng buộc đã cho, `<'a>` chỉ định rằng tham số lifetime generic có thể là bất kỳ lifetime nào khớp với ràng buộc mượn của trình biên dịch. Cả hai đều được khai báo trong cùng một cặp dấu ngoặc nhọn, ví dụ: `<'a, T>`.

---

## Level 3: Applying

**Question 1:**
en: Write a generic struct named `Point` that can hold x and y coordinates of any identical type. Show how to instantiate it with integers and floats.
vi: Viết một struct generic có tên `Point` có thể chứa tọa độ x và y thuộc bất kỳ kiểu dữ liệu giống hệt nhau. Thể hiện cách khởi tạo nó với số nguyên (integers) và số thực (floats).

**Answer 1:**
en: We define `Point<T>` where both `x` and `y` are of type `T`.
vi: Chúng ta định nghĩa `Point<T>` trong đó cả `x` và `y` đều có kiểu `T`.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.2, y: 4.5 };
    
    // let mixed_point = Point { x: 5, y: 4.5 }; // This would fail because T must be the same for both.
}
```

**Question 2:**
en: Given the `Point<T>` struct, write an implementation block (`impl`) that provides an `x()` method to return a reference to the `x` field.
vi: Với struct `Point<T>`, hãy viết một khối triển khai (`impl`) cung cấp phương thức `x()` để trả về một tham chiếu trỏ đến trường `x`.

**Answer 2:**
en: We must declare the generic parameter `<T>` immediately after the `impl` keyword so the compiler knows that `T` is a generic type, not an actual concrete type named `T`.
vi: Chúng ta phải khai báo tham số generic `<T>` ngay sau từ khóa `impl` để trình biên dịch biết rằng `T` là một kiểu generic, chứ không phải là một kiểu cụ thể có tên là `T`.

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

---

## Level 4: Analyzing

**Question 1:**
en: Let's say you have a generic function `fn compare<T>(a: T, b: T) -> bool`, and inside it you write `a > b`. Why will this code fail to compile, and how can you fix it using Trait Bounds?
vi: Giả sử bạn có một hàm generic `fn compare<T>(a: T, b: T) -> bool`, và bên trong nó bạn viết `a > b`. Tại sao đoạn mã này sẽ không biên dịch được, và bạn có thể sửa nó bằng cách sử dụng Trait Bounds (ràng buộc trait) như thế nào?

**Answer 1:**
en: The code fails because the compiler doesn't know if the generic type `T` actually supports the greater-than (`>`) operation. By default, `T` can be literally any type, including structs that aren't comparable. To fix this, you must restrict `T` with a trait bound, specifically `std::cmp::PartialOrd`, to guarantee that `T` has comparison capabilities.
vi: Mã biên dịch lỗi vì trình biên dịch không biết liệu kiểu generic `T` có thực sự hỗ trợ thao tác lớn hơn (`>`) hay không. Mặc định, `T` có thể là bất kỳ kiểu nào, kể cả các struct không thể so sánh được. Để khắc phục, bạn phải dùng trait bound (chuỗi ràng buộc), một cách chi tiết là `std::cmp::PartialOrd`, đối chiếu rằng `T` bao hàm chức năng compare trên.

```rust
// Fixed version overriding with Trait Bounds
fn compare<T: PartialOrd>(a: T, b: T) -> bool {
    a > b
}
```

**Question 2:**
en: Contrast the use of a single generic `<T>` versus multiple generics `<T, U>` in a struct definition. Provide a scenario where `<T, U>` is necessary.
vi: Phân tích sự đối nghịch trong việc xài `<T>` đơn và multiple generics `<T, U>` trong khai báo kiến trúc trúc (struct). Đề xuất tình tiết sử dụng lúc nào `<T, U>` hiệu quả.

**Answer 2:**
en: Using `<T>` forces all fields marked with `T` to be of the exact identical type. Multiple generics `<T, U>` allow fields to be of independent types. For instance, in an HTTP `Response<T, U>` struct where `T` is the body type (like JSON or Text) and `U` represents custom header metadata tags, the body and headers rarely share the exact identical data type, making `<T, U>` essential for flexibility.
vi: Xài `<T>` cưỡng chế lập trình phải dùng toàn bộ khối field là loại giá trị hoàn hảo giống nhau. Ở hệ đa parameter `<T, U>`, các field có thể tự gán chức năng khác biệt riêng rẽ mà không bị ép buộc lặp type. Đơn cử ở struct điều phối HTTP `Response<T, U>` ở đó `T` đại diện thân text body (JSON/Image) và `U` nhắm đến loại siêu dữ liệu (metadata/header tags), thông thường thân bài sẽ khác header nên `<T, U>` là thiết yếu cho giải pháp hoàn hảo đó.

---

## Level 5: Evaluating

**Question 1:**
en: Evaluate the tradeoff of Monomorphization in Rust. What is the primary negative impact on the development lifecycle when building large generic-heavy projects?
vi: Đánh giá sự đánh đổi (tradeoff) của quy trình Đơn hình hóa (Monomorphization) trong Rust. Đoạn phát triển dự án lập trình với siêu generics gặp ảnh hưởng tiêu cực chủ yếu gì theo vòng đời sống?

**Answer 1:**
en: 
While Monomorphization guarantees zero-cost abstraction at runtime, the primary negative impact is **significantly increased compile times** and **larger binary sizes** (binary bloat). Because the compiler duplicates the code for every concrete type used via the generic interface, compiling a heavy framework parsing dozens of types forces massive AST duplication. This can make the edit-compile-run loop slower for the developer.
vi: 
Trong khi Monomorphization đảm bảo những lớp trừu tượng tại runtime sẽ tốn phí "zero-cost" , thì cú tát tác động tiêu cực chính là **sự phình to dung lượng binary** và **gian nan tăng compile time** cực độ. Tại vì trình biên dịch phải sao chụp mã logic cho toàn bộ concrete type, compile các bộ API / Framework đồ sộ cho chục type khác nhau đều phải nhân bản dạng cây cú pháp (AST) tương ứng. Đều đó kiềm hãm vòng tròn lặp Edit - Compile - Run vòng chậm lại.

**Question 2:**
en: Defend the decision of combining generic traits (like `T: Display + Debug`) using the `where` clause format as opposed to inline formatting `<T: Display + Debug>`. When does styling become crucial?
vi: Hãy bao biện cho phương án hòa nhập bộ trait bounds (vd `T: Display + Debug`) thông qua cú pháp mệnh đề `where` so sánh với ghi nội dòng `<T: Display + Debug>`. Lúc nào style thiết kế này mang chức năng sống còn?

**Answer 2:**
en: Inline trait bounding works beautifully for short signatures but becomes unreadable in advanced abstractions. The `where` clause creates architectural clarity by decoupling the generic declarations from the generic constraints. This styling becomes crucial when a function returns complex iterators or requires multiple generic parameters each with multi-trait bounds (e.g., `<T, K, V>` where each needs `Clone`, `Hash`, `Serialize`). The `where` clause pushes all this visual clutter to the bottom of the signature, keeping the function parameters and return type immediately visible and legible to the observer.
vi: Binding trực tiếp tại signature tuy ngắn gọn với module đơn nhưng đánh mất thẩm mỹ đọc ở tầng đa abstraction. Mệnh đề `where` tạo ra tính kiến trúc trong sáng bằng cách tháo gỡ khai báo tham số chung tách rời khỏi hệ ràng buộc (constraints). Chìa khoá sinh tồn này hiệu lực khi một func trả lại hàng đống parameters `<T, K, V>` mang tính phức tạp như HashMap Iterator mà mỗi cái đều phải bao chứa cả đống trait (`Clone`, `Hash`, `Serialize`). Mệnh đề `where` nhả khối text đó rơi thẳng xuống sát mép logic thân hầm dưới khiến function tên gọi thân bài dễ tương tác và định lượng cho người quan sát đọc qua.
