# Ownership in Rust - Bloom's Taxonomy

## Level 1: Remembering

**Question 1:**
en: What are the three main rules of ownership in Rust?
vi: Ba quy tắc cốt lõi của quyền sở hữu (ownership) trong Rust là gì?

**Answer 1:**
en: 
1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
vi: 
1. Mỗi giá trị trong Rust có một biến được gọi là chủ sở hữu (owner) của nó.
2. Chỉ có thể có một chủ sở hữu tại một thời điểm.
3. Khi chủ sở hữu ra khỏi phạm vi (scope), giá trị sẽ bị hủy (dropped).


**Question 2:**
en: What is the `Drop` trait, and how does it relate to ownership?
vi: Trait `Drop` là gì, và nó liên quan như thế nào đến quyền sở hữu (ownership)?

**Answer 2:**
en: The `Drop` trait allows you to customize what happens when a value is about to go out of scope. It is automatically called by Rust when the owner of the value goes out of scope to clean up resources, such as freeing memory on the heap.
vi: Trait `Drop` cho phép bạn tùy chỉnh những gì xảy ra khi một giá trị chuẩn bị ra khỏi phạm vi. Nó được Rust tự động gọi khi chủ sở hữu của giá trị ra khỏi phạm vi để dọn dẹp tài nguyên, chẳng hạn như giải phóng bộ nhớ trên heap.

---

## Level 2: Understanding

**Question 1:**
en: Explain the difference between "Move" and "Copy" semantics in Rust.
vi: Giải thích sự khác biệt giữa ngữ nghĩa "Move" (Di chuyển) và "Copy" (Sao chép) trong Rust.

**Answer 1:**
en: 
- **Move**: For complex types (like `String` allocated on the heap), assigning a variable to another transfers ownership. The original variable becomes invalid to prevent double-free errors.
- **Copy**: For simple types with known sizes at compile time (like integers `i32`), assigning one variable to another creates a bitwise copy of the value, leaving the original variable valid. These types implement the `Copy` trait.
vi: 
- **Move**: Đối với các kiểu phức tạp (như `String` được cấp phát trên heap), việc gán một biến cho một biến khác sẽ chuyển quyền sở hữu. Biến gốc trở nên không hợp lệ để ngăn chặn lỗi giải phóng kép (double-free).
- **Copy**: Đối với các kiểu đơn giản có kích thước xác định tại thời điểm biên dịch (như số nguyên `i32`), việc gán biến này cho biến khác tạo ra một bản sao bitwise của giá trị, và biến gốc vẫn hợp lệ. Các kiểu này implement trait `Copy`.


**Question 2:**
en: How does Rust's ownership model prevent standard memory bugs like dangling pointers without using a garbage collector?
vi: Mô hình ownership của Rust ngăn chặn các lỗi bộ nhớ tiêu chuẩn như các con trỏ lơ lửng (dangling pointers) mà không cần sử dụng bộ thu gom rác (garbage collector) như thế nào?

**Answer 2:**
en: Rust enforces memory safety at compile time. Since every value has exactly one owner, and its lifetime is tied to that owner's scope, the memory is freed precisely when the scope ends. For borrowing, the compiler's borrow checker ensures that references never outlive the data they point to, thereby preventing dangling pointers outright at compile time.
vi: Rust đảm bảo an toàn bộ nhớ tại thời điểm biên dịch. Vì mỗi giá trị có chính xác một chủ sở hữu, và vòng đời của nó gắn liền với phạm vi của chủ sở hữu đó, bộ nhớ được giải phóng chính xác khi phạm vi kết thúc. Đối với việc mượn (borrowing), trình kiểm tra mượn (borrow checker) của trình biên dịch đảm bảo rằng các tham chiếu không bao giờ tồn tại lâu hơn dữ liệu mà chúng trỏ tới, do đó ngăn chặn hoàn toàn các con trỏ lơ lửng tại thời điểm biên dịch.

---

## Level 3: Applying

**Question 1:**
en: Write a code example demonstrating how to transfer ownership of a `String` to a function, and another function that borrows the `String` without taking ownership.
vi: Viết một ví dụ mã lệnh minh họa cách chuyển quyền sở hữu của một `String` cho một hàm, và một hàm khác chỉ mượn (borrow) `String` mà không lấy quyền sở hữu.

**Answer 1:**
en: When passing a `String` to a function directly, ownership is moved. To avoid this, we can pass a reference `&String` to borrow it. Calling `takes_ownership` invalidates `s1`, while `borrows` allows reusing `s2`.
vi: Khi truyền trực tiếp một `String` vào một hàm, quyền sở hữu sẽ bị di chuyển. Để tránh điều này, chúng ta có thể truyền một tham chiếu `&String` để mượn nó. Việc gọi `takes_ownership` làm cho `s1` mất hiệu lực, trong khi `borrows` cho phép tái sử dụng `s2`.

```rust
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and `drop` is called. Memory is freed.

fn borrows(some_string: &String) {
    println!("{}", some_string);
} // some_string goes out of scope here. But because it doesn't own what it refers to, nothing is dropped.

fn main() {
    let s1 = String::from("hello");
    takes_ownership(s1);
    // println!("{}", s1); // This would cause a compile error!

    let s2 = String::from("world");
    borrows(&s2);
    println!("s2 is still valid: {}", s2); // Works fine!
}
```


**Question 2:**
en: Apply the concept of partial mapping and shadowing. Given a tuple `(String, i32)`, how can you destructure and move only the `String` part, leaving the `i32` still accessible via the original tuple?
vi: Áp dụng khái niệm phân rã một phần (partial mapping) và che khuất biến (shadowing). Cho một tuple `(String, i32)`, làm thế nào bạn có thể phân rã và chỉ di chuyển (move) phần `String`, trong khi phần `i32` vẫn có thể truy cập được thông qua tuple ban đầu?

**Answer 2:**
en: You can destructure the tuple by binding only specific elements. Because `String` does not implement `Copy`, attempting to bind it moves the value out of the tuple, making the `String` part of the original tuple inaccessible. However, since the `i32` portion implements `Copy` and was not moved (or was only copied), you can still access the `i32` part using the tuple's dot notation.
vi: Bạn có thể phân rã tuple bằng cách chỉ gán các phần tử cụ thể. Vì `String` không implement `Copy`, việc gán nó sẽ di chuyển giá trị ra khỏi tuple, khiến phần `String` của tuple ban đầu không thể truy cập được nữa. Tuy nhiên, vì phần `i32` implement `Copy` và không bị di chuyển (hoặc chỉ bị copy), bạn vẫn có thể truy cập phần `i32` bằng cách sử dụng ký pháp chấm (dot notation) của tuple.

```rust
fn main() {
    let t = (String::from("rust"), 2026);
    let (s, _) = t; // 's' takes ownership of the String
    // println!("{:?}", t); // Error: t is partially moved
    println!("String is: {}", s);
    println!("Integer is still valid: {}", t.1); // This works!
}
```

---

## Level 4: Analyzing

**Question 1:**
en: Analyze the following compilation error. Explain why it occurs based on ownership rules, and show how to fix it by returning ownership.
vi: Phân tích lỗi biên dịch sau. Giải thích lý do tại sao nó xảy ra dựa trên các quy tắc quyền sở hữu, và đưa ra cách sửa lỗi bằng cách trả lại quyền sở hữu.
Code to analyze / Nguồn mã để phân tích:
```rust
fn create_and_use_string() {
    let s = String::from("hello");
    let length = calculate_length(s);
    println!("The length of '{}' is {}.", s, length); // Error!
}
fn calculate_length(s: String) -> usize {
    s.len()
}
```

**Answer 1:**
en: The error occurs because `calculate_length` takes a `String` by value, moving ownership of `s` into the function. When the function ends, `s` is dropped. The subsequent `println!` attempts to use `s` after it has been moved (use of moved value error). We can fix this by having the function return the `String` along with the length, thus transferring ownership back to the caller.
vi: Lỗi xảy ra vì `calculate_length` nhận vào một `String` theo giá trị, làm di chuyển quyền sở hữu của `s` vào hàm. Khi hàm kết thúc, `s` bị hủy. Lệnh `println!` ngay sau đó cố gắng sử dụng `s` sau khi nó đã bị di chuyển (lỗi sử dụng giá trị đã di chuyển). Chúng ta có thể sửa vấn đề này bằng cách làm cho hàm trả về `String` cùng với độ dài, qua đó chuyển lại quyền sở hữu cho hàm gọi.

```rust
fn create_and_use_string() {
    let s1 = String::from("hello");
    // Receive the string back along with its length
    let (s2, length) = calculate_length_fixed(s1);
    println!("The length of '{}' is {}.", s2, length); // Now this works
}

fn calculate_length_fixed(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // Return ownership back
}
```


**Question 2:**
en: Contrast the memory layout of `String` and string literals (`&str`), analyzing how ownership conceptually differs between the two.
vi: So sánh sự khác nhau về bố cục bộ nhớ của `String` và các hằng chuỗi (string literals - `&str`), phân tích sự khác nhau về mặt khái niệm quyền sở hữu giữa cấu trúc cả hai.

**Answer 2:**
en: 
- A `String` is composed of 3 parts stored on the stack (pointer to heap, length, and capacity), while the actual text is allocated on the heap. Ownership applies here: the variable binding owns this heap memory, and when it is dropped, the heap memory is freed.
- A string literal (`&str` static reference) is a slice pointing to text hardcoded directly into the program's binary (read-only memory). Because it's fundamentally a reference to pre-existing data, there is no "owner" responsible for freeing the memory at runtime. Thus, `&str` represents borrowing a view into a string rather than owning it.
vi: 
- Một `String` bao gồm 3 phần được lưu trữ trên stack (con trỏ trỏ đến heap, độ dài, và dung lượng), trong khi văn bản thực tế được cấp phát trên heap. Quyền sở hữu được áp dụng ở đây: biến gán sở hữu vùng nhớ heap này, và khi nó bị hủy, vùng nhớ heap sẽ được giải phóng.
- Một hằng chuỗi (tham chiếu tĩnh `&str`) là một slice trỏ đến văn bản được mã hóa cứng trực tiếp vào tập tin nhị phân của chương trình (bộ nhớ chỉ đọc). Bởi vì về cơ bản nó là một tham chiếu đến dữ liệu có sẵn, không có "chủ sở hữu" nào chịu trách nhiệm giải phóng bộ nhớ tại thời điểm chạy. Do đó, `&str` đại diện cho việc mượn (borrowing) một góc nhìn vào chuỗi thay vì sở hữu nó.

---

## Level 5: Evaluating

**Question 1:**
en: Evaluate ownership vs. garbage collection (GC). Defend Rust's choice to use the ownership model instead of a GC for systems programming. What are the key performance tradeoffs?
vi: Đánh giá quyền sở hữu (ownership) đối nghịch với thu gom rác (garbage collection - GC). Bảo vệ sự lựa chọn về việc sử dụng mô hình quyền sở hữu thay vì GC cho lập trình hệ thống (systems programming) của Rust. Đâu là những đánh đổi chính về hiệu năng?

**Answer 1:**
en: Rust's ownership model provides deterministic memory management, meaning developers know exactly when and where memory is freed. This guarantees low latency and consistent performance, without the unpredictable "stop-the-world" pauses inherent to garbage collectors. For systems programming (like OS kernels or game engines), predictable performance and fine-grained memory control are critical. The tradeoff is a steeper learning curve and slower initial development time, as the developer must rigidly appease the borrow checker construct, whereas GC languages allow rapid prototyping without worrying about memory lifetimes.
vi: Mô hình ownership của Rust cung cấp khả năng quản lý bộ nhớ mang tính xác định, nghĩa là lập trình viên biết chính xác khi nào và ở đâu bộ nhớ được giải phóng. Điều này đảm bảo độ trễ thấp và hiệu suất đồng nhất, không có những khoảng dừng "stop-the-world" không thể đoán trước vốn có ở các bộ thu gom rác. Đối với lập trình hệ thống (như nhân hệ điều hành hay engine trò chơi), hiệu năng có thể dự đoán được và quyền kiểm soát phân mảng bộ nhớ tinh chỉnh là rất quan trọng. Sự đánh đổi là quá trình học khó khăn hơn và thời gian phát triển ban đầu chậm hơn, do lập trình viên phải nghiêm ngặt làm hài lòng cơ chế borrow checker, trong khi các ngôn ngữ dùng GC cho phép tạo mẫu nhanh mà không cần lo lắng về vòng đời của bộ nhớ.


**Question 2:**
en: Judge the assertion that "Ownership in Rust eliminates the need for reference counting." Do you agree with this statement? Justify your answer by discussing types like `Rc<T>` and `Arc<T>`.
vi: Đánh giá nhận định rằng "Ownership trong Rust loại bỏ nhu cầu sử dụng cơ chế đếm tham chiếu (reference counting)". Bạn có đồng ý với nhận định này không? Chứng minh câu trả lời của bạn bằng cách thảo luận về các kiểu dữ liệu như `Rc<T>` và `Arc<T>`.

**Answer 2:**
en: This assertion is false. While standard ownership solves most memory safety problems, it strictly enforces a "single owner" rule. In real-world applications, such as representing graph data structures or sharing state amongst multiple threads, multiple parts of a program may legitimately need equal ownership of the same data. Rust acknowledges this by providing reference counting smart pointers like `Rc<T>` (for single-threaded) and `Arc<T>` (for multi-threaded scenarios). These types bypass the single owner rule by keeping a dynamically updated count of owners at runtime, dropping the inner value only when the counter reaches zero.
vi: Nhận định này là sai. Dù mô hình ownership tiêu chuẩn giải quyết hầu hết các vấn đề về an toàn bộ nhớ, nó thực thi nghiêm ngặt bộ quy tắc "chủ sở hữu duy nhất". Trong các ứng dụng thực tế, chẳng hạn như biểu diễn cấu trúc dữ liệu đồ thị hoặc chia sẻ trạng thái giữa nhiều luồng (thread), nhiều bộ phận của chương trình có thể thật sự cần quyền sở hữu ngang nhau đối với cùng một dữ liệu. Rust công nhận điều này nên đã cung cấp các con trỏ thông minh đếm tham chiếu như `Rc<T>` (cho đơn luồng) và `Arc<T>` (cho kịch bản đa luồng). Các kiểu dữ liệu này bỏ qua quy tắc một chủ sở hữu bằng cách giữ một bộ đếm số lượng các chủ sở hữu theo thời gian thực tại thời điểm chạy (runtime), chỉ giải phóng giá trị bên trong khi bộ đếm rơi xuống bằng 0.
