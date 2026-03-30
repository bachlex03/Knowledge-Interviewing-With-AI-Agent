# Reference and Borrowing in Rust - Bloom's Taxonomy

## Level 1: Remembering

**Question 1:**
en: What is borrowing in Rust, and what symbol is used to create a reference?
vi: Việc mượn (borrowing) trong Rust là gì, và ký hiệu nào được sử dụng để tạo một tham chiếu?

**Answer 1:**
en: Borrowing is the mechanism of accessing data without taking ownership of it. A reference is created using the ampersand symbol (`&`).
vi: Việc mượn là cơ chế truy cập dữ liệu mà không lấy đi quyền sở hữu của nó. Một tham chiếu được tạo ra bằng cách sử dụng ký hiệu dấu và (`&`).

**Question 2:**
en: State the two main rules of references in Rust.
vi: Nêu hai quy tắc chính của tham chiếu trong Rust.

**Answer 2:**
en: 
1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid (no dangling references).
vi: 
1. Tại bất kỳ thời điểm nào, bạn có thể có một tham chiếu có thể thay đổi (mutable) hoặc vô số tham chiếu không thể thay đổi (immutable).
2. Các tham chiếu phải luôn hợp lệ (không có tham chiếu lơ lửng - dangling references).

---

## Level 2: Understanding

**Question 1:**
en: Why does Rust allow multiple immutable references but restricts mutable references to exactly one at a time?
vi: Tại sao Rust cho phép nhiều tham chiếu không thể thay đổi nhưng lại giới hạn số lượng tham chiếu có thể thay đổi chính xác ở mức một tại một thời điểm?

**Answer 1:**
en: Rust allows multiple immutable references because reading data concurrently is safe and won't cause data races. It restricts mutable references to exactly one to prevent data races at compile time, ensuring that no two pointers access the same data at the same time if at least one of them is writing to it.
vi: Rust cho phép nhiều tham chiếu không thể thay đổi vì việc đọc dữ liệu song song là an toàn và sẽ không gây ra data racing. Nó giới hạn tham chiếu có thể hạy đổi ở mức một để ngăn chặn data racing tại thời điểm biên dịch, đảm bảo rằng không có hai con trỏ nào truy cập vào cùng một dữ liệu cùng lúc nếu ít nhất một trong số chúng đang ghi vào đó.

**Question 2:**
en: Explain the concept of a dangling reference and how Rust's compiler handles it.
vi: Giải thích khái niệm tham chiếu lơ lửng (dangling reference) và cách trình biên dịch tĩnh của Rust xử lý nó.

**Answer 2:**
en: A dangling reference is a pointer that references a location in memory that may have been given to someone else or freed. Rust's compiler, using the borrow checker, guarantees that references will never be dangling by ensuring that the data will not go out of scope before the reference to the data does.
vi: Một tham chiếu lơ lửng là một con trỏ tham chiếu đến một vị trí trong bộ nhớ mà có thể đã được giao cho người khác hoặc đã bị giải phóng. Trình biên dịch của Rust, sử dụng borrow checker, đảm bảo rằng các tham chiếu sẽ không bao giờ lơ lửng bằng cách chắc chắn rằng dữ liệu sẽ không ra khỏi phạm vi trước khi tham chiếu đến dữ liệu đó kết thúc.

---

## Level 3: Applying

**Question 1:**
en: Write a function `change_string` that takes a mutable reference to a `String` and appends the word " world" to it. Show how to call this function.
vi: Viết một hàm `change_string` nhận vào một tham chiếu có thể thay đổi tới một `String` và thêm từ " world" vào nó. Thể hiện cách gọi hàm này.

**Answer 1:**
en: We define the function parameter as `&mut String` and use `.push_str()` to alter it. We must ensure the caller's string is mutable as well.
vi: Chúng ta định nghĩa tham số hàm là `&mut String` và sử dụng `.push_str()` để thay đổi nó. Chúng ta phải đảm bảo chuỗi của hàm gọi cũng là chuỗi có thể thay đổi được (mutable variable).

```rust
fn change_string(s: &mut String) {
    s.push_str(" world");
}

fn main() {
    let mut my_string = String::from("hello");
    change_string(&mut my_string);
    println!("{}", my_string); // Prints "hello world"
}
```

**Question 2:**
en: Given the following snippet, add a block scope `{}` to fix the borrow checker error so that both mutable references are valid independently.
vi: Cho đoạn mã sau, hãy thêm một phạm vi khối lệnh `{}` để sửa lỗi của borrow checker sao cho cả hai tham chiếu có thể thay đổi đều độc lập và hợp lệ.

**Answer 2:**
en: The first mutable borrow must go out of scope before the second one is created. We can fix this by wrapping the first borrow in a narrower scope.
vi: Lượt mượn mutable đầu tiên phải ra khỏi phạm vi trước khi lượt mượn thứ hai được tạo. Chúng ta có thể sửa vấn đề này bằng cách đưa lượt mượn đầu tiên vào một block scope hẹp hơn.

```rust
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(" world");
    } // r1 goes out of scope here, so we can make a new reference without problems.

    let r2 = &mut s;
    r2.push_str("!");
    println!("{}", r2);
}
```

---

## Level 4: Analyzing

**Question 1:**
en: Analyze the following code. Will it compile? Explain why or why not based on the rules of borrowing.
vi: Phân tích đoạn mã sau. Nó có biên dịch được không? Giải thích lý do tại sao hoặc tại sao không dựa trên các quy tắc vay mượn.
Code to analyze:
```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s; 
    let r2 = &s; 
    let r3 = &mut s; 
    println!("{}, {}, and {}", r1, r2, r3);
}
```

**Answer 1:**
en: No, the code will not compile. It violates the rule that you cannot have a mutable reference (`r3`) while you have immutable references (`r1` and `r2`) to the same value in the same scope. The immutable borrows' lifetimes end after their last usage in the `println!`, meaning `r3`'s creation overlaps with the active immutable borrows.
vi: Không, đoạn mã sẽ không thể biên dịch. Nó vi phạm quy tắc rằng bạn không thể có một tham chiếu có thể thay đổi (`r3`) trong khi bạn đang có các tham chiếu không thể thay đổi (`r1` và `r2`) cho cùng một giá trị trong cùng một phạm vi. Vòng đời của các tham chiếu không thể thay đổi được kết thúc sau lần sử dụng cuối cùng của chúng tại `println!`, nghĩa là thời điểm khởi tạo của `r3` bị đè đè lên vùng mượn không thể thay đổi đang kích hoạt.

**Question 2:**
en: Compare passing large structs by value versus passing them by reference. What are the performance and architectural implications?
vi: So sánh việc truyền tham số chứa cấu trúc dữ liệu lớn (structs lớn) bằng giá trị so với việc truyền bằng tham chiếu. Ý nghĩa về hiệu suất và kiến trúc là gì?

**Answer 2:**
en: 
- **By Value (Move/Copy):** Moving implies a potential memcpy if the type implements `Copy` or just ownership transfer if it doesn't. Moving large objects limits reuse since the original variable becomes invalid. If it's a `Clone`, making an explicit copy is computationally expensive (memory allocation and duplicating data).
- **By Reference (Borrowing):** Passing `&T` simply passes a pointer. It's an O(1) operation regardless of the struct's size, preserving memory and CPU cycles. Architecturally, borrows enforce that the caller retains ownership and responsibility for the memory lifetime, while the function just "looks" at the data.
vi: 
- **Theo Giá Trị (Move/Copy):** Di chuyển ám chỉ một phép `memcpy` tiềm năng nếu kiểu thực hiện `Copy` hoặc chỉ là chức năng chuyển vùng sở hữu nếu không. Di chuyển các object lớn làm hạn chế khả năng tái sử dụng vì biến ban đầu trở nên mất tác dụng. Nếu đó là một `Clone`, việc tạo một bản copy rõ ràng là tốn kém về điện toán (cấp phát bộ nhớ và nhân bản dữ liệu).
- **Theo Tham Chiếu (Vay/Mượn):** Truyền tham chiếu `&T` đơn giản chỉ là truyền một con trỏ. Nó là thao tác kích thước O(1) bất kể kích thước của con trỏ là gì, tiết kiệm memory và chu kỳ CPU. Về mặt kiến trúc, hệ thống Borrow bắt buộc người gọi chịu chi phí duy trì quyền sở hữu và trách nhiệm đối với vòng đời bộ nhớ, trong khi function chỉ đang "nhìn" vào data.

---

## Level 5: Evaluating

**Question 1:**
en: Evaluate the Borrow Checker's role in Rust. Many newcomers complain that the borrow checker makes development too slow and inflexible. Defend the strictness of the borrow checker in building highly concurrent backend systems.
vi: Đánh giá vai trò của Borrow Checker trong Rust. Nhiều người mới học phàn nàn rằng borrow checker làm chậm quá trình phát triển và thiếu linh hoạt. Hãy bảo vệ mức độ nghiêm ngặt của borrow checker đối với việc thiết kế hệ thống backend concurrency cường độ cao.

**Answer 1:**
en: While the borrow checker steepens the learning curve and occasionally slows initial prototyping, it acts as a proactive defense mechanism. In highly concurrent backend systems, data races represent some of the most difficult and catastrophic bugs to trace. The borrow checker ensures at compile-time that shared mutable state doesn't exist unsafely. By enforcing these strict rules ahead of execution, Rust eliminates entirely multi-threaded crash categories, making code refactoring and scaling predictably secure. In production systems, the compile-time cost is immensely cheaper than debugging runtime deadlocks and race conditions.
vi: Mặc dù borrow checker tạo ra một đường cong học hỏi khó khăn và đôi lúc làm chậm việc xây dựng nguyên mẫu ban đầu, nó hoạt động như một cơ chế phòng thủ chủ động. Trong các hệ thống backend đồng thời mức cao, data race đại diện cho những lỗi khó truy tìm và tệ hại nhất. Borrow checker đảm bảo tại compile-time rằng trạng thái biến đổi được chia sẻ không thể tồn tại một cách thiếu an toàn. Bằng cách cưỡng chế những quy định này trước thời gian thực thi, Rust loại bỏ hoàn toàn một loạt các danh mục crash multi-thread, khiến cho việc refactoring mã và scale code trở nên vô cùng an toàn có thể dự báo được. Trong production system, cái giá của compile-time mỏng hơn rất nhiều việc debug vòng đời deadlock và điều kiện chạy đua (race condition) tại runtime.

**Question 2:**
en: Some programming languages use implicit references for any object assignment (e.g., Java, Python). Rust requires explicit references (`&`). Judge the benefits and drawbacks of Rust's explicit syntax approach.
vi: Một số ngôn ngữ lập trình sử dụng tham chiếu ngầm định cho bất kỳ phép gán đối tượng nào (VD: Java, Python). Rust yêu cầu các tham chiếu hiện tường (`&`, `mut`). Hãy nhận định những lợi ích và nhược điểm của cách tiếp cận cú pháp rõ ràng này trong Rust.

**Answer 2:**
en: 
- **Benefits:** Explicit syntax clearly communicates intent. By looking at a function signature (e.g., `fn process(x: &mut Foo)`), a developer immediately knows the function will modify the original object. It forces engineers to think about data aliasing and mutability upfront, preventing unexpected side effects often found in implicit reference languages where mutating a passed object accidentally changes it globally.
- **Drawbacks:** It adds syntactic noise and requires developers to frequently deal with dereferencing (`*`) and referencing (`&`) semantics. This overhead hinders developer velocity when writing simple logical algorithms that would otherwise take less effort in implicit languages.
vi: 
- **Lợi ích:** Cú pháp lộ khai minh rõ ràng ý tưởng. Bằng việc nhìn vào hàm (vd. `fn process(x: &mut Foo)`), người phát triển biết ngay lập tức chức năng sẽ sửa đổi đối tượng gốc. Nó bắt người kỹ sư nghĩ đến aliasing dữ liệu và tính khả biến ngay lúc đầu, phòng ngừa được những tình huống gây tác dụng phụ mà ở đó sự đa hình được truyền vô tình sửa đổi toàn bộ thông qua các ngôn ngữ tham chiếu ngầm.
- **Nhược điểm:** Nó cấu thành nên noise (tiếng động nhiễu lập trình) về cú pháp và yêu cầu lập trình viên phải thường xuyên xử lý tính chất ngữ nghĩa giải tham chiếu (`*`) và tham chiếu (`&`). Chi phí đầu tư này kiềm chế tốc độ phát triển cho việc triển khai code các logic mà thật sự nếu xài hệ ngôn ngữ tham chiếu ngầm sẽ ít tốn chi phí hơn.
