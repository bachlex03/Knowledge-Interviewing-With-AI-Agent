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
