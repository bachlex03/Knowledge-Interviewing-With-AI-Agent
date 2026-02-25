# Array Notes / Ghi chú về Mảng

## 1. Modifying Elements with Mutable References / Sửa đổi phần tử bằng Tham chiếu có thể thay đổi

**En:**
To modify elements of an array during iteration, follow these steps:

- **`&mut data`**: Iterate using mutable references. The loop variable (e.g., `item`) becomes type `&mut T`.
- **`*` (Dereference Operator)**: You must use the `*` symbol to "access" the actual value behind the reference.
- **Example**:
  ```rust
  for item in &mut data {
      *item *= 2; // Follow the pointer to modify the real value
  }
  ```

**Vi:**
Để sửa đổi các phần tử của một mảng trong quá trình lặp, hãy làm theo các bước sau:

- **`&mut data`**: Lặp bằng các tham chiếu có thể thay đổi. Biến vòng lặp (ví dụ: `item`) sẽ có kiểu `&mut T`.
- **`*` (Toán tử giải tham chiếu)**: Bạn phải sử dụng ký hiệu `*` để "truy cập" vào giá trị thực tế đằng sau tham chiếu đó.
- **Ví dụ**:
  ```rust
  for item in &mut data {
      *item *= 2; // Đi theo con trỏ để sửa đổi giá trị thực
  }
  ```

---

## 2. The `.contains()` Method / Phương thức `.contains()`

**En:**
- **Syntax**: `array.contains(&value)`
- **Why `&`**: The method requires a reference (`&T`) to avoid taking ownership or unnecessarily copying large objects. Even for small types like `i32`, the signature mandates a reference.
- **Return Value**: Returns a `bool` (`true` if found, `false` otherwise).

**Vi:**
- **Cú pháp**: `array.contains(&value)`
- **Tại sao dùng `&`**: Phương thức yêu cầu một tham chiếu (`&T`) để tránh việc chiếm quyền sở hữu hoặc sao chép các đối tượng lớn không cần thiết. Ngay cả với các kiểu dữ liệu nhỏ như `i32`, chữ ký của hàm vẫn yêu cầu một tham chiếu.
- **Giá trị trả về**: Trả về kiểu `bool` (`true` nếu tìm thấy, `false` nếu không).

---

## 3. The `.map()` Method / Phương thức `.map()`

**En:**
- **Syntax**: `array.map(|x| x * x)`
- **Behavior**: Transforms every element using a **closure** (`|x| ...`) and returns a **new array** of the same length.
- **Key Point**: It **does not modify** the original array (unlike a mutable `for` loop). The original remains valid and unchanged.

**Vi:**
- **Cú pháp**: `array.map(|x| x * x)`
- **Hành vi**: Biến đổi mọi phần tử bằng cách sử dụng một **closure** (`|x| ...`) và trả về một **mảng mới** có cùng độ dài.
- **Điểm mấu chốt**: Nó **không sửa đổi** mảng gốc (khác với vòng lặp `for` có thể thay đổi). Mảng gốc vẫn hợp lệ và không bị thay đổi.

---

## 4. Array vs. Slice / So sánh Mảng và Lát cắt (Slice)

**En:**
While they look similar, they have different memory representations and behaviors:

- **Array (`[T; N]`)**: 
  - **Ownership**: Owns its data.
  - **Size**: Fixed and known at compile time (part of the type).
  - **Location**: Stored directly on the stack.
- **Slice (`&[T]`)**: 
  - **Ownership**: Borrows data (does not own it).
  - **Size**: Known at runtime.
  - **Representation**: A "Fat Pointer" consisting of a pointer to the data and a length.
  - **Flexibility**: Can point to portions of arrays or vectors.

| Feature | Array | Slice |
| :--- | :--- | :--- |
| **Type** | `[i32; 10]` | `&[i32]` |
| **Ownership** | Owns data | Borrows data |
| **Size** | Known at compile time | Known at runtime |

**Vi:**
Mặc dù trông có vẻ giống nhau, chúng có cách biểu diễn bộ nhớ và hành vi khác nhau:

- **Mảng (`[T; N]`)**: 
  - **Quyền sở hữu**: Sở hữu dữ liệu của chính nó.
  - **Kích thước**: Cố định và được biết khi biên dịch (là một phần của kiểu dữ liệu).
  - **Vị trí**: Lưu trữ trực tiếp trên stack.
- **Lát cắt - Slice (`&[T]`)**: 
  - **Quyền sở hữu**: Vay mượn (borrow) dữ liệu (không sở hữu nó).
  - **Kích thước**: Được biết khi chạy (runtime).
  - **Biểu diễn**: Một "Con trỏ béo" (Fat Pointer) gồm một con trỏ đến dữ liệu và một độ dài.
  - **Tính linh hoạt**: Có thể trỏ đến các phần của mảng hoặc vector.

| Đặc điểm | Mảng (Array) | Lát cắt (Slice) |
| :--- | :--- | :--- |
| **Kiểu dữ liệu** | `[i32; 10]` | `&[i32]` |
| **Quyền sở hữu** | Sở hữu dữ liệu | Vay mượn dữ liệu |
| **Kích thước** | Biết khi biên dịch | Biết khi chạy (runtime) |



