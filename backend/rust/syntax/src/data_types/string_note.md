# String & Ownership Notes / Ghi chú về String & Quyền sở hữu

## 1. String Concatenation: `s1 + &s2` / Phép nối chuỗi: `s1 + &s2`

**En:**
In the expression `let s3 = s1 + &s2;`, the behavior of each variable is different based on how they are passed to the `add` method:

- **`s1` (Moved)**: Taken **by value**. Ownership of the heap data is transferred to the new string `s3`. `s1` is no longer valid.
- **`s2` (Borrowed)**: Taken **by reference** (`&s2`). `s2` remains valid and usable because only a temporary permission to "read" its content was given.
- **Memory Efficiency**: Rust reuses the memory buffer from `s1`, appends `s2`'s content to it, and returns the result as `s3`. This avoids allocating a brand-new space for "Hello".

**Vi:**
Trong biểu thức `let s3 = s1 + &s2;`, hành vi của mỗi biến là khác nhau dựa trên cách chúng được truyền vào phương thức `add`:

- **`s1` (Bị di chuyển)**: Được truyền theo **giá trị (by value)**. Quyền sở hữu dữ liệu trên heap được chuyển sang chuỗi mới `s3`. `s1` không còn hợp lệ để sử dụng.
- **`s2` (Được cho mượn)**: Được truyền theo **tham chiếu (by reference - `&s2`)**. `s2` vẫn hợp lệ và có thể sử dụng được vì chỉ có quyền "đọc" nội dung tạm thời được cấp đi.
- **Hiệu quả bộ nhớ**: Rust tái sử dụng vùng nhớ của `s1`, nối thêm nội dung của `s2` vào đó, và trả về kết quả dưới tên `s3`. Điều này giúp tránh việc phải cấp phát một vùng nhớ hoàn toàn mới cho chuỗi ban đầu.

---

## 2. Understanding "Taken by Value" / Hiểu về "Truyền theo giá trị"

**En:**
A `String` on the stack consists of three parts: **Pointer, Length, and Capacity**. When a string is "taken by value":

1. **Stack Copy**: Rust copies those three pieces of data from the source to the destination.
2. **Ownership Transfer**: Since the destination now has the pointer, it becomes the new owner of the heap memory.
3. **Invalidation**: The original variable is marked as invalid. This prevents a "double-free" error (two variables trying to delete the same memory).

**Vi:**
Một `String` trên stack gồm ba phần: **Con trỏ (Pointer), Độ dài (Length), và Dung lượng (Capacity)**. Khi một chuỗi được "truyền theo giá trị":

1. **Sao chép trên Stack**: Rust sao chép ba mẩu dữ liệu đó từ nguồn sang đích.
2. **Chuyển quyền sở hữu**: Vì bên đích hiện đã có con trỏ, nó trở thành chủ sở hữu mới của vùng nhớ trên heap.
3. **Vô hiệu hóa**: Biến ban đầu được đánh dấu là không hợp lệ. Điều này giúp ngăn chặn lỗi "double-free" (hai biến cùng cố gắng xóa một vùng nhớ).

---

## 3. `format!` Macro vs. `+` Operator / Macro `format!` so với Toán tử `+`

**En:**
The `format!` macro and `+` operator differ significantly in ownership and performance:

- **Ownership**: 
  - `+` **moves** at least one string (making it unusable).
  - `format!` **borrows** its arguments, keeping the original variables valid.
- **Performance**: 
  - `+` is often faster as it **reuses** the memory of the first string.
  - `format!` is slightly slower as it always allocates a **brand new String** and copies everything into it.
- **Best Practice**: Use `+` for simple, high-performance joining; use `format!` for complexity or when you need to keep your variables.

**Vi:**
Macro `format!` và toán tử `+` khác nhau đáng kể về quyền sở hữu và hiệu suất:

- **Quyền sở hữu**:
  - `+` **di chuyển (move)** ít nhất một chuỗi (làm nó không thể sử dụng sau đó).
  - `format!` **mượn (borrow)** các đối số, giữ cho các biến ban đầu vẫn hợp lệ.
- **Hiệu suất**:
  - `+` thường nhanh hơn vì nó **tái sử dụng** bộ nhớ của chuỗi đầu tiên.
  - `format!` chậm hơn một chút vì nó luôn cấp phát một **String hoàn toàn mới** và sao chép tất cả nội dung vào đó.
- **Lời khuyên**: Sử dụng `+` cho các phép nối đơn giản, cần hiệu suất cao; sử dụng `format!` cho các trường hợp phức tạp hoặc khi bạn cần giữ lại các biến của mình.

---

## 4. The `.collect()` Method / Phương thức `.collect()`

**En:**
In Rust, `.collect()` is a versatile method that transforms an **iterator** into a concrete **collection** (like `Vec`, `HashMap`, or `String`).

- **Why it's needed**: Many methods like `.split()` are "lazy"—they return an iterator but don't perform the work immediately. Calling `.collect()` forces the iterator to process and store the results.
- **Type Annotation**: Since `.collect()` can create many types of collections, you must specify the target type (e.g., `let parts: Vec<&str> = ...`) so the compiler knows what to build.

**Vi:**
Trong Rust, `.collect()` là một phương thức linh hoạt giúp biến đổi một **trình lặp (iterator)** thành một **bộ sưu tập (collection)** cụ thể (như `Vec`, `HashMap`, hoặc `String`).

- **Tại sao cần thiết**: Nhiều phương thức như `.split()` có tính chất "lười biếng" (lazy)—chúng trả về một trình lặp nhưng không thực hiện công việc ngay lập tức. Gọi `.collect()` buộc trình lặp phải xử lý và lưu trữ kết quả.
- **Chú thích kiểu dữ liệu**: Vì `.collect()` có thể tạo ra nhiều loại bộ sưu tập khác nhau, bạn phải chỉ định kiểu dữ liệu đích (ví dụ: `let parts: Vec<&str> = ...`) để trình biên dịch biết cần tạo ra cái gì.

---

## 5. Lazy Evaluation & Iterators / Đánh giá lười biếng & Trình lặp

**En:**
"Lazy" means a calculation is not performed until it is absolutely necessary.

- **How it works**: Methods like `.split()`, `.map()`, or `.filter()` just create a "plan" (Iterator) without processing data. The work only happens when the iterator is "consumed" (e.g., via `.collect()` or a `for` loop).
- **Benefits**:
  - **Efficiency**: Stops processing as soon as requirements are met (e.g., finding the first 5 of a million items).
  - **Chaining**: Allows stacking operations without creating intermediate temporary lists.

**Vi:**
"Lười biếng" (Lazy) có nghĩa là một phép tính sẽ không được thực hiện cho đến khi nó thực sự cần thiết.

- **Cách hoạt động**: Các phương thức như `.split()`, `.map()`, hoặc `.filter()` chỉ tạo ra một "kế hoạch" (Iterator) mà không thực sự xử lý dữ liệu ngay lúc đó. Công việc chỉ thực sự diễn ra khi trình lặp bị "tiêu thụ" (ví dụ: thông qua `.collect()` hoặc vòng lặp `for`).
- **Lợi ích**:
  - **Hiệu suất**: Dừng xử lý ngay khi đạt được yêu cầu (ví dụ: tìm 5 mục đầu tiên trong số một triệu mục).
  - **Kết nối chuỗi**: Cho phép xếp chồng các thao tác mà không cần tạo ra các danh sách tạm thời trung gian.

---

## 6. String vs. String Slice (&str) / String so với Lát cắt chuỗi (&str)

**En:**
The primary difference is **ownership**:
- **`String`**: Owned, heap-allocated, growable.
- **`&str`**: A borrowed reference (fat pointer) to a sequence of UTF-8 bytes.

| Feature | `String` | `&str` |
| :--- | :--- | :--- |
| **Ownership** | Owner | Borrowed (Reference) |
| **Allocation** | Heap-allocated | Points to existing memory |
| **Mutability** | Growable / Modifiable | Immutable (usually) |

**Vi:**
Sự khác biệt chính là **quyền sở hữu**:
- **`String`**: Được sở hữu, cấp phát trên heap, có thể mở rộng.
- **`&str`**: Một tham chiếu đi mượn (fat pointer) đến một chuỗi các byte UTF-8.

| Đặc điểm | `String` | `&str` |
| :--- | :--- | :--- |
| **Quyền sở hữu** | Chủ sở hữu | Đi mượn (Tham chiếu) |
| **Cấp phát** | Cấp phát trên Heap | Trỏ đến vùng nhớ có sẵn |
| **Khả năng thay đổi** | Có thể mở rộng | Bất biến (thông thường) |

---

## 7. Memory Allocation: Heap vs. Stack / Cấp phát bộ nhớ: Heap và Stack

**En:**
- **`String`**: Its metadata (pointer, length, capacity) is on the **stack**, but the actual character data is on the **heap**.
- **`array` ([T; N])**: Allocated entirely on the **stack** by default. Its size must be known at compile time.
- **Manual Heap Allocation**: You can move stack data (like an array) to the heap using `Box<T>`.

**Vi:**
- **`String`**: Siêu dữ liệu (con trỏ, độ dài, dung lượng) nằm trên **stack**, nhưng dữ liệu ký tự thực tế nằm trên **heap**.
- **Mảng (`array` - [T; N])**: Theo mặc định được cấp phát hoàn toàn trên **stack**. Kích thước của nó phải được biết tại thời điểm biên dịch.
- **Cấp phát Heap thủ công**: Bạn có thể di chuyển dữ liệu từ stack (như một mảng) lên heap bằng cách sử dụng `Box<T>`.

---

## 8. String Literals / Chuỗi Literals

**En:**
- **Type**: `&'static str`.
- **Storage**: Hardcoded directly into the **compiled binary** (Read-Only Data segment).
- **Lifetime**: `'static` (exists for the entire duration of the program).
- **Nature**: They are immutable and not allocated on the stack or heap at runtime.

**Vi:**
- **Kiểu dữ liệu**: `&'static str`.
- **Lưu trữ**: Được nhúng trực tiếp vào **tệp nhị phân đã biên dịch** (phân đoạn dữ liệu chỉ đọc).
- **Vòng đời**: `'static` (tồn tại trong suốt thời gian chương trình chạy).
- **Bản chất**: Chúng là bất biến và không được cấp phát trên stack hay heap khi chương trình đang chạy.

---

## 9. Length vs. Capacity / Độ dài so với Dung lượng

**En:**
Both are measured in **bytes**.
- **Length (`len`)**: The number of bytes currently used by the string.
- **Capacity (`capacity`)**: The total bytes allocated on the heap.
- **Reallocation**: When `len == capacity`, adding data triggers a reallocation (usually doubling the capacity) to maintain performance.

**Vi:**
Cả hai đều được tính bằng **byte**.
- **Độ dài (`len`)**: Số lượng byte hiện đang được chuỗi sử dụng.
- **Dung lượng (`capacity`)**: Tổng số byte đã được cấp phát trên heap.
- **Cấp phát lại**: Khi `độ dài == dung lượng`, việc thêm dữ liệu sẽ kích hoạt cấp phát lại (thường là gấp đôi dung lượng) để duy trì hiệu suất.

---

## 10. Deep Dive: Capacity Optimization / Đi sâu vào: Tối ưu hóa dung lượng

**En:**
- **`String::with_capacity(n)`**: Used to pre-allocate memory. Faster because it avoids multiple reallocations.
- **Growth Strategy**: Rust uses an exponential growth strategy (amortized O(1)) to minimize expensive calls to the OS.
- **`shrink_to_fit()`**: `String` never automatically releases memory. You must call this method to decrease capacity to match the current length.

**Vi:**
- **`String::with_capacity(n)`**: Được sử dụng để cấp phát bộ nhớ trước. Nhanh hơn vì tránh được nhiều lần cấp phát lại.
- **Chiến lược tăng trưởng**: Rust sử dụng chiến lược tăng trưởng theo hàm mũ (amortized O(1)) để giảm thiểu các lần gọi tốn kém đến hệ điều hành.
- **`shrink_to_fit()`**: `String` không bao giờ tự động giải phóng bộ nhớ. Bạn phải gọi phương thức này để giảm dung lượng khớp với độ dài hiện tại.

---

## 11. Slice as a Type (DST) / Slice như một Kiểu dữ liệu (DST)

**En:**
A **Slice** is a special kind of type known as a **DST (Dynamically Sized Type)**:
- **The Unsized Type (`[T]` or `str`)**: Rust doesn't know its size at compile time, so you cannot create a variable of this type directly.
- **The Slice Reference (`&[T]` or `&str`)**: This is what we use in code. It's a **Fat Pointer** containing the memory address AND the length.
- **Flexibility**: Unlike arrays where size is part of the type (e.g., `[i32; 5]`), a slice reference can point to any number of elements, making it ideal for function parameters.

**Vi:**
**Slice (Lát cắt)** là một loại kiểu đặc biệt được gọi là **DST (Dynamically Sized Type - Kiểu có kích thước động)**:
- **Kiểu không định cỡ (`[T]` hoặc `str`)**: Rust không biết kích thước của chúng tại thời điểm biên dịch, vì vậy bạn không thể tạo trực tiếp một biến thuộc kiểu này.
- **Tham chiếu lát cắt (`&[T]` hoặc `&str`)**: Đây là những gì chúng ta sử dụng trong mã nguồn. Nó là một **Fat Pointer (Con trỏ béo)** chứa địa chỉ bộ nhớ VÀ độ dài.
- **Sự linh hoạt**: Không giống như mảng có kích thước là một phần của kiểu dữ liệu (ví dụ: `[i32; 5]`), một tham chiếu lát cắt có thể trỏ đến bất kỳ số lượng phần tử nào, giúp nó trở nên lý tưởng cho các tham số của hàm.


