# Rust Mastery Roadmap: Problems & Concepts

This document summarizes the core challenges and advanced concepts that a Rust developer needs to master to reach a Senior/Architect level.

---

## 1. Memory Safety & Ownership (An toàn Bộ nhớ & Quyền sở hữu)
en: Mastering the borrow checker to write safe and efficient code without a Garbage Collector.
vi: Làm chủ borrow checker để viết mã an toàn và hiệu quả mà không cần bộ thu gom rác (Garbage Collector).

- **Ownership & Move Semantics**: Tracking value transfer and invalidation.
- **Borrowing (Shared vs Mutable)**: Rules of references and aliasing.
- **Lifetimes**: Ensuring references never outlive the data they point to.
- **The Stack vs The Heap**: Understanding memory allocation with `Box`, `Vec`, and `String`.

---

## 2. Type System & Traits (Hệ thống Kiểu & Traits)
en: Using Rust's powerful type system to enforce business logic and polymorphism.
vi: Sử dụng hệ thống kiểu mạnh mẽ của Rust để thực thi logic nghiệp vụ và tính đa hình.

- **Generics vs Trait Objects**: Static dispatch (`impl Trait`) vs Dynamic dispatch (`dyn Trait`).
- **Orphan Rule & Blanket Impls**: Rules for implementing traits on external types.
- **Associated Types vs Generic Parameters**: Designing flexible and readable traits.
- **Derive Macros**: Automating common trait implementations.

---

## 3. Concurrency & Parallelism (Xử lý Đồng thời & Song song)
en: Writing thread-safe code using the "Fearless Concurrency" paradigm.
vi: Viết mã an toàn đa luồng bằng mô hình "Fearless Concurrency".

- **Send & Sync Traits**: How Rust proves thread safety at compile time.
- **Arc & Mutex/RwLock**: Shared ownership and synchronized mutation across threads.
- **Channels (mpsc)**: Message passing between threads for decoupled logic.
- **Data Races**: Why they are impossible in Safe Rust.

---

## 4. Async Rust (Lập trình Bất đồng bộ)
en: Building high-performance network services with non-blocking I/O.
vi: Xây dựng các dịch vụ mạng hiệu năng cao với I/O không chặn.

- **The Future Trait**: How async state machines work under the hood.
- **Executors & Runtimes (Tokio/async-std)**: How tasks are scheduled.
- **Pinning**: Handling self-referential structures in async blocks.
- **Select! & Join!**: Coordinating multiple concurrent tasks.

---

## 5. Error Handling & Robustness (Xử lý Lỗi & Tính bền bỉ)
en: Designing reliable systems that handle every possible failure case.
vi: Thiết kế các hệ thống tin cậy có khả năng xử lý mọi trường hợp thất bại.

- **Result & Option Flow**: Idiomatic ways to handle success, failure, and absence.
- **Error Propagation (`?` operator)**: Simplifying error handling logic.
- **Custom Error Types (thiserror/anyhow)**: Balancing library precision vs. application convenience.
- **Panic vs. Recoverable Errors**: When to crash and when to handle.

---

## 6. Performance & Unsafe Rust (Hiệu năng & Unsafe Rust)
en: Pushing Rust to the limit while maintaining safety boundaries.
vi: Đưa Rust tới giới hạn hiệu năng trong khi vẫn duy trì ranh giới an toàn.

- **Iterators & Zero-Cost Abstractions**: Writing high-level code that compiles to efficient machine code.
- **Smart Pointers (Rc, Raw Pointers, Cell/RefCell)**: Breaking standard rules safely.
- **FFI (Foreign Function Interface)**: Interacting with C/C++ libraries.
- **Unsafe blocks**: When and how to manually bypass compiler checks.

---

# Deep Dive: Problems & Solutions (Phân tích Chuyên sâu)

## 1. Ownership & Move Semantics
en: **Problem**: In Rust, assigning a value to another variable "moves" it, making the original variable invalid. This prevents double-free bugs but can be confusing for beginners.
vi: **Vấn đề**: Trong Rust, việc gán một giá trị cho một biến khác sẽ "di chuyển" (move) nó, làm cho biến ban đầu không còn hợp lệ. Điều này ngăn chặn lỗi double-free nhưng có thể gây khó hiểu cho người mới.

**Solution:**

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // en: s1 is MOVED here / vi: s1 bị MOVE tại đây

    // println!("{}", s1); // en: Error! s1 is invalid / vi: Lỗi! s1 không còn hợp lệ
    
    // en: Solution: Use .clone() if you need both / vi: Giải pháp: Dùng .clone() nếu cần cả hai
    let s3 = s2.clone(); 
    println!("s2: {}, s3: {}", s2, s3);
}
```

---

## 2. Borrowing Rules (Shared vs Mutable)
en: **Problem**: You cannot have a mutable reference (`&mut T`) while other references (shared or mutable) exist in the same scope. This prevents data races at compile time.
vi: **Vấn đề**: Bạn không thể có một tham chiếu có thể thay đổi (`&mut T`) trong khi các tham chiếu khác (chia sẻ hoặc thay đổi) đang tồn tại trong cùng phạm vi. Điều này ngăn chặn data race ngay khi biên dịch.

**Solution:**

```rust
fn main() {
    let mut data = vec![1, 2, 3];

    let r1 = &data; // en: shared borrow / vi: tham chiếu chia sẻ
    let r2 = &data; // en: OK / vi: OK
    
    // let r3 = &mut data; // en: ERROR! Cannot borrow as mutable while shared exists
    // vi: LỖI! Không thể mượn kiểu mut khi đang có tham chiếu chia sẻ

    println!("{:?}, {:?}", r1, r2);
    
    // en: r1, r2 out of scope here / vi: r1, r2 hết phạm vi tại đây
    let r3 = &mut data; // en: Now it's OK / vi: Bây giờ thì OK
    r3.push(4);
}
```

---

## 3. Lifetimes & Dangling References
en: **Problem**: Returning a reference to local data that will be dropped when the function returns.
vi: **Vấn đề**: Trả về một tham chiếu tới dữ liệu cục bộ - dữ liệu sẽ bị hủy khi hàm kết thúc.

**Solution (Using valid lifetimes):**

```rust
// en: ERROR: returns reference to local variable
// vi: LỖI: trả về tham chiếu tới biến cục bộ
// fn get_str() -> &str { let s = String::from("hi"); &s } 

// en: SOLUTION 1: Return the owned type / vi: GIẢI PHÁP 1: Trả về kiểu có quyền sở hữu
fn get_string() -> String { String::from("hi") }

// en: SOLUTION 2: Pass the data in, then return reference / vi: GIẢI PHÁP 2: Truyền dữ liệu vào, trả về tham chiếu
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

## 4. Multi-threading with Arc & Mutex
en: **Problem**: Sharing and mutating data across multiple threads safely.
vi: **Vấn đề**: Chia sẻ và thay đổi dữ liệu giữa các luồng một cách an toàn.

**Solution:**

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // en: Arc = Atomic Reference Count (Shared ownership)
    // en: Mutex = Mutual Exclusion (Thread-safe mutation)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles { handle.join().unwrap(); }
    println!("Result: {}", *counter.lock().unwrap());
}
```

---

## 5. Result vs Option (Error Flow)
en: **Problem**: Handling complex operations that can fail or return nothing without crashing.
vi: **Vấn đề**: Xử lý các thao tác phức tạp có thể thất bại hoặc không trả về gì mà không làm sập ứng dụng.

**Solution:**

```rust
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(res) => println!("Success: {}", res),
        Err(e)  => println!("Error: {}", e),
    }

    // en: Using the ? operator for easy propagation
    // vi: Sử dụng toán tử ? để truyền lỗi nhanh chóng
}
```

---

## 6. Trait Objects vs Static Dispatch
en: **Problem**: Designing functions that accept different types that share a common behavior.
vi: **Vấn đề**: Thiết kế các hàm chấp nhận các kiểu dữ liệu khác nhau nhưng có chung một hành vi (behavior).

**Solution:**

```rust
trait Speak { fn say(&self); }
struct Dog;
impl Speak for Dog { fn say(&self) { println!("Woof!"); } }

// en: Static Dispatch (Fast, specialized code)
// vi: Dispatch tĩnh (Nhanh, tạo mã chuyên biệt)
fn static_speak<T: Speak>(item: T) { item.say(); }

// en: Dynamic Dispatch (Flexible, smaller binary size)
// vi: Dispatch động (Linh hoạt, kích thước binary nhỏ hơn)
fn dynamic_speak(item: &dyn Speak) { item.say(); }
```

---

## 7. Async/Await & Pinning
en: **Problem**: Future values that contain references to themselves cannot be moved in memory, otherwise their internal pointers become invalid.
vi: **Vấn đề**: Các giá trị Future chứa tham chiếu tới chính nó không thể bị di chuyển trong bộ nhớ, nếu không các con trỏ nội bộ của chúng sẽ mất hiệu lực.

**Solution (The concept of Pin):**

```rust
// en: Pin<P> ensures the data P points to stays fixed in memory.
// en: This is essential for async/await because the state machine 
//     generated by the compiler often contains internal references.
// vi: Pin<P> đảm bảo dữ liệu mà P trỏ tới được cố định trong bộ nhớ.
//     Điều này thiết yếu cho async/await vì máy trạng thái do 
//     trình biên dịch tạo ra thường chứa các tham chiếu nội bộ.
```

---

## 8. Recursive Types & Box
en: **Problem**: Types whose size cannot be known at compile time (like a recursive List).
vi: **Vấn đề**: Các kiểu dữ liệu có kích thước không thể xác định khi biên dịch (như một danh sách đệ quy).

**Solution (Indirect allocation on the Heap):**

```rust
// en: ERROR: "recursive type has infinite size"
// enum List { Cons(i32, List), Nil }

// en: SOLUTION: Use Box to put the next element on the Heap
// vi: GIẢI PHÁP: Dùng Box để đưa phần tử tiếp theo lên Heap
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

---

## 9. Closures & Variable Capturing
en: **Problem**: A closure needs to take ownership of a variable from its environment to use it inside a thread or store it.
vi: **Vấn đề**: Một closure cần lấy quyền sở hữu của một biến từ môi trường của nó để sử dụng bên trong một luồng hoặc lưu trữ nó.

**Solution (The move keyword):**

```rust
fn main() {
    let x = vec![1, 2, 3];
    
    // en: 'move' forces the closure to take ownership of its environment
    // vi: 'move' bắt buộc closure lấy quyền sở hữu của môi trường xung quanh nó
    let handle = std::thread::spawn(move || {
        println!("Vector from thread: {:?}", x);
    });

    handle.join().unwrap();
}
```

---

## Summary Checklist / Danh sách kiểm tra tóm tắt
| Category | Priority | Difficulty |
| :--- | :--- | :--- |
| **Memory Safety** | High | Medium |
| **Concurrency** | High | High |
| **Async Rust** | High | High |
| **Type System** | Medium | Medium |
| **Unsafe/FFI** | Low | High |
