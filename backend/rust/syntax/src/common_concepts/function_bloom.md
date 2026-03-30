# Functions in Rust - Bloom's Taxonomy

## Level 1: Remembering

**Question 1:**
en: What is the keyword used to declare a function in Rust?
vi: Từ khóa nào được sử dụng để khai báo một hàm trong Rust?

**Answer 1:**
en: The `fn` keyword is used to declare a function in Rust.
vi: Từ khóa `fn` được sử dụng để khai báo một hàm trong Rust.

**Question 2:**
en: How do you specify the return type of a function in Rust?
vi: Bạn chỉ định kiểu trả về của một hàm trong Rust như thế nào?

**Answer 2:**
en: The return type is specified after an arrow (`->`) following the function's parameter list.
vi: Kiểu trả về được chỉ định sau một mũi tên (`->`) theo sau danh sách tham số của hàm.

---

## Level 2: Understanding

**Question 1:**
en: Explain the difference between statements and expressions in the context of Rust functions.
vi: Giải thích sự khác biệt giữa câu lệnh (statements) và biểu thức (expressions) trong bối cảnh các hàm của Rust.

**Answer 1:**
en: In Rust, statements are instructions that perform some action and do not return a value (e.g., `let y = 6;`). Expressions evaluate to a resultant value (e.g., `5 + 6`). Function bodies are made up of a series of statements optionally ending in an expression. If a block ends in an expression (without a semicolon), that value is returned.
vi: Trong Rust, câu lệnh (statements) là các chỉ thị thực hiện một hành động nào đó và không trả về giá trị (ví dụ: `let y = 6;`). Biểu thức (expressions) đánh giá và tạo ra một giá trị kết quả (ví dụ: `5 + 6`). Thân hàm bao gồm một chuỗi các câu lệnh và có thể kết thúc bằng một biểu thức. Nếu một khối lệnh kết thúc bằng một biểu thức (không có dấu chấm phẩy), giá trị đó sẽ được trả về.

**Question 2:**
en: Why must you explicitly declare the type of each parameter in a Rust function signature?
vi: Tại sao bạn phải khai báo rõ ràng kiểu của từng tham số trong chữ ký hàm (function signature) của Rust?

**Answer 2:**
en: Rust requires explicit type annotations for function parameters because it ensures that the compiler has enough information to reason about the code without needing to infer types globally. This makes compile times faster and error messages much more precise and localized to the function boundary.
vi: Rust yêu cầu chú thích kiểu rõ ràng cho các tham số ranh giới hàm vì nó đảm bảo rằng trình biên dịch có đủ thông tin để suy luận về đoạn mã mà không cần phải suy luận kiểu cục bộ toàn cục. Điều này làm cho thời gian biên dịch nhanh hơn và các thông báo lỗi chính xác và khu trú hơn ở phạm vi của hàm.

---

## Level 3: Applying

**Question 1:**
en: Write a function named `calculate_area` that takes two `f64` parameters (width and height) and returns their product (the area) implicitly as an `f64`.
vi: Viết một hàm tên là `calculate_area` nhận vào hai tham số `f64` (chủ yếu là chiều rộng và chiều cao) và trả về tích của chúng (diện tích) một cách ngầm định dưới dạng `f64`.

**Answer 1:**
en: Implicit return is done by ending the function with an expression containing no semicolon.
vi: Việc trả về ngầm định được thực hiện bằng cách kết thúc hàm với một biểu thức không có dấu chấm phẩy.

```rust
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height // No semicolon here causes an implicit return
}

fn main() {
    let area = calculate_area(5.0, 10.0);
    println!("The area is: {}", area);
}
```

**Question 2:**
en: Create a function that uses the `return` keyword to explicitly return early if a given `i32` number is negative, otherwise, it should return the number multiplied by 10 implicitly.
vi: Tạo một hàm sử dụng từ khóa `return` để trả về sớm (ngầm định) nếu một số `i32` cho trước là số âm, nếu không thì nó sẽ trả về số đó nhân với 10 (ngầm định không dùng `return`).

**Answer 2:**
en: The `return` keyword is necessary to exit the control flow before reading the last expression of the function.
vi: Từ khóa `return` là cần thiết để thoát khỏi luồng điều khiển trước khi đọc vào biểu thức cuối cùng của hàm.

```rust
fn process_number(n: i32) -> i32 {
    if n < 0 {
        return 0; // Early explicit return
    }
    n * 10 // Implicit return at the end of the block
}
```

---

## Level 4: Analyzing

**Question 1:**
en: Analyze the following function. What compiler error will occur and why?
vi: Phân tích hàm sau. Lỗi biên dịch nào sẽ xảy ra và tại sao?
Code / Mã lệnh:
```rust
fn get_user_id() -> i32 {
    let id = 10;
    id;
}
```

**Answer 1:**
en: The compiler will throw a "mismatched types" error because the function signature declares it returns an `i32`, but the body ends with a statement `id;` (due to the semicolon). A statement evaluates to the unit type `()`, not an `i32`.
vi: Trình biên dịch sẽ báo lỗi "mismatched types" (kiểu không khớp) vì chữ ký hàm khai báo trả về một `i32`, nhưng phần thân lại kết thúc bằng một câu lệnh `id;` (do có dấu chấm phẩy). Một câu lệnh đánh giá ra kiểu unit `()`, chứ không phải `i32`.

**Question 2:**
en: Categorize the difference between calling standard functions and calling standard macros (like `println!`).
vi: Phân loại sự khác biệt giữa việc gọi các hàm tiêu chuẩn và việc gọi các macro tiêu chuẩn (như `println!`).

**Answer 2:**
en: 
- **Functions (`fn`)**: They take a fixed number of arguments with predefined types. They are compiled into machine code as a single callable routine.
- **Macros (`!`)**: Denoted by the exclamation mark, macros are metaprogramming tools that generate code at compile time. They can accept a variable number of arguments and even different types (like variadic arguments in `println!`), which normal Rust functions cannot natively do in the same way.
vi: 
- **Hàm (`fn`)**: Chúng nhận một số lượng tham số cố định với các kiểu được xác định trước. Chúng được biên dịch thành mã máy như một quy trình có thể triệu gọi duy nhất.
- **Macro (`!`)**: Được biểu thị bằng dấu chấm than, macro là các công cụ meta-programming sinh ra mã nguồn tại thời điểm biên dịch. Chúng có thể chấp nhận số lượng đối số lượng biến đổi và thậm chí là các kiểu dữ liệu dị biến (như các đối số biến đổi trong `println!`), điều mà hàm Rust thông thường không thể thực hiện một cách tự nhiên.

---

## Level 5: Evaluating

**Question 1:**
en: Evaluate Rust's decision to use implicit returns (leaving off the semicolon on the last expression). What are the arguments for and against this syntactical pattern compared to strictly requiring the `return` keyword as in C or Java?
vi: Đánh giá quyết định của Rust về việc sử dụng implicit returns (bỏ dấu chấm phẩy ở biểu thức cuối cùng). Các lập luận ủng hộ và phản đối pattern cú pháp này so với việc yêu cầu nghiêm ngặt dùng từ khóa `return` như trong C hoặc Java là gì?

**Answer 1:**
en: 
- **Pros:** It embraces an expression-oriented programming style characteristic of functional languages. It reduces boilerplate, making closures and short map/filter functions significantly cleaner and easier to read.
- **Cons:** It can confuse beginners transitioning from C-family statements-oriented languages, where a missing semicolon is traditionally just a syntax error. It requires careful visual tracking of semicolons to discern whether a value is being returned or suppressed.
vi: 
- **Ưu điểm:** Nó theo sát định dạng phong cách lập trình thiên hướng biểu thức rất đặc trưng ở các ngôn ngữ lập trình hàm. Nó giúp loại bỏ boilerplate dư thừa, biến các closure (hàm đóng) và những function map/filter ngắn trở nên mượt mà rõ ràng hơn. 
- **Nhược điểm:** Nó có thể làm nhiễu người cơ bản đi sâu từ dòng họ ngôn ngữ C hướng thủ tục vốn bị lỗi syntax liên tục với dấu `;`. Lập trình viên phải để mắt sát sao để dõi theo xem biến đó đã return hay chỉ là bị loại bỏ (như unit data type `()`). 

**Question 2:**
en: Judge the utility of "Unit type `()`" as the default return type of Rust functions when no arrow `->` is provided. Does this simplify or complicate the language type system?
vi: Hãy nhận xét lợi ích của việc sử dụng loại rỗng (Unit type `()`) làm return type mặc định khi không khai báo `->` trong Rust. Liệu điều này đơn giản hóa hay làm phức tạp hệ thống loại dữ liệu của ngôn ngữ?

**Answer 2:**
en: The Unit type `()` simplifies the type system structurally. Unlike `void` in C/C++, which literally means "no type" and forces the compiler to create special cases for void-returning functions versus value-returning functions, `()` is an actual concrete type with exactly one value (an empty tuple). This consistency means trait bounds, generics, and compiler internals can handle everything as returning a value, avoiding edge cases internally. It simplifies generic implementations because you don't need a specific overload for generic parameters that represent "nothing".
vi: Kiểu `()` làm cho type system Rust về chức năng hoàn thiện hơn. Không như `void` trong C/C++ vốn hoàn toàn đồng nghĩa với "không có dữ liệu" và lại buộc phải điều chỉnh đặc vụ kiểm duyệt void-function riêng biệt so với non-void, Unit `()` lại là cấu trúc data rỗng với duy nhất một thông số duy nhất: tuple tập rỗng. Nó thiết lập consistency cho generic/trait binding tránh phải xử lý sự phức tạp ngầm khi xử lý. Điều đó tối ưu hóa generic implementations vì bạn không cần viết quá tải overload cho bất kì tham gia vô hiệu nào cả.
