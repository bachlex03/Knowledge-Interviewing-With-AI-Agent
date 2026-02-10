# Integer Types in Rust (Notes)

- Rust infernce i32 by default

## OS related

It’s a classic "off-by-one" logic puzzle! The reason it stops at **255** is that we have a limited number of "patterns" available, and we use one of those patterns to represent **0**.

Think of it like a car's odometer with only three digits. If you can only show numbers from `000` to `999`, you can show **1,000** distinct values total—but the highest number you can see is **999**.

---

### The Math of Bits

Each bit is a switch that can be either **0** or **1**. With 8 bits, the total number of unique combinations is calculated as:


If we started counting at 1, we could reach 256. However, computers are designed to start counting at **0**.

### Let's visualize the count:

| Sequence | Binary | Decimal Value |
| --- | --- | --- |
| 1st combination | `00000000` | **0** |
| 2nd combination | `00000001` | **1** |
| ... | ... | ... |
| 255th combination | `11111110` | **254** |
| 256th combination | `11111111` | **255** |

If you tried to represent **256**, you would need a **9th bit**: `1 00000000`. Since a byte only has 8 "slots," that 9th bit has nowhere to go. This is what programmers call an **integer overflow**.

---

### The "Odometer" Visual

Imagine a physical counter:

1. You start at `00000000`.
2. You keep adding 1 until you hit `11111111` (which is 255).
3. If you add **one more**, the whole thing flips back to `00000000` because there is no room for the carry-over digit.

> **Fun Fact:** This is why original Pokémon games (like Red/Blue) often had items cap at 255, or why certain glitches happened at level 256—the memory literally couldn't hold a higher number in that single byte!

Would you like to see what happens in Rust code when you try to force a variable to go from 255 to 256?

---

## Handling Overflow with `Option`

When using methods like `checked_add`, Rust returns an `Option<T>`. You must handle the `None` case (overflow) to extract the value.

### 1. `unwrap()` / `expect()`
en: Use when you want to panic on overflow.
vi: Sử dụng khi bạn muốn chương trình dừng lại (panic) nếu bị tràn số.
```rust
let x = 255u8.checked_add(1).unwrap();
let x = 255u8.checked_add(1).expect("Overflow!");
```

### 2. `unwrap_or()`
en: Provide a default value on overflow.
vi: Cung cấp một giá trị mặc định nếu bị tràn số.
```rust
let x = 255u8.checked_add(1).unwrap_or(0);
```

### 3. `match` Expression
en: Explicitly handle both cases.
vi: Xử lý tường minh cả hai trường hợp.
```rust
let x = match 255u8.checked_add(1) {
    Some(val) => val,
    None => 0,
};
```

### 4. `if let`
en: Execute code only if no overflow occurs.
vi: Chỉ thực thi mã nếu không xảy ra tràn số.
```rust
if let Some(val) = 255u8.checked_add(1) {
    println!("Result: {}", val);
}
```


# Rust Integer Method & Formatting Notes / Ghi chú về Phương thức Số nguyên & Định dạng trong Rust

## 1. Error E0689: Ambiguous Numeric Type / Lỗi kiểu số mơ hồ

**Error / Chi tiết lỗi:**
`error[E0689]: can't call method checked_add on ambiguous numeric type {integer}`

**En:**
- **Cause**: Rust needs the exact type (like `i32`, `u64`) to know which version of a method to call. If the type is just inferred as a generic "integer", methods like `checked_add` cannot be called.
- **Solution**: Explicitly annotate the type of the variable.

**Vi:**
- **Nguyên nhân**: Rust cần biết chính xác kiểu dữ liệu (như `i32`, `u64`) để biết nên gọi phiên bản nào của phương thức. Nếu kiểu chỉ được suy luận là "số nguyên" chung chung, các phương thức như `checked_add` sẽ không thể được gọi.
- **Giải pháp**: Ghi chú kiểu dữ liệu (type annotation) cho biến một cách tường minh.

```rust
// Wrong / Sai
let x = 5;
let result = x.checked_add(1);

// Correct / Đúng
let x: i32 = 5;
let result = x.checked_add(1);
```

---

## 2. Option Return Type & Type Mismatch / Kiểu trả về Option & Sai lệch kiểu

**En:**
- **Cause**: Methods like `checked_add` return `Option<T>` (e.g., `Option<i32>`) to handle potential overflows safely. You cannot assign this directly to a variable of type `i32`.
- **Solution**: Use `Option<i32>` or handle the value using `.unwrap()`, `.unwrap_or()`, or pattern matching.

**Vi:**
- **Nguyên nhân**: Các phương thức như `checked_add` trả về kiểu `Option<T>` (ví dụ: `Option<i32>`) để xử lý an toàn trường hợp tràn số. Bạn không thể gán trực tiếp kết quả này vào một biến kiểu `i32`.
- **Giải pháp**: Sử dụng kiểu `Option<i32>` hoặc xử lý giá trị bằng `.unwrap()`, `.unwrap_or()`, hoặc pattern matching.

```rust
// Error / Lỗi
let x: i32 = 5;
let y: i32 = x.checked_add(1); // Mismatch: Option<i32> vs i32

// Correct / Đúng
let y = x.checked_add(1); // y is Option<i32>
```

---

## 3. Error E0277: Display vs Debug Formatting / Lỗi định dạng Display và Debug

**Error / Chi tiết lỗi:**
`error[E0277]: Option<i32> doesn't implement std::fmt::Display`

**En:**
- **Cause**: The `{}` placeholder uses the `Display` trait, which `Option` does not implement.
- **Solution**: Use `{:?}` (Standard Debug) or `{:#?}` (Pretty-print Debug).

**Vi:**
- **Nguyên nhân**: Ký hiệu `{}` sử dụng trait `Display`, nhưng `Option` không thực thi trait này.
- **Giải pháp**: Sử dụng `{:?}` (Debug tiêu chuẩn) hoặc `{:#?}` (Debug căn chỉnh đẹp).

```rust
let x = Some(6);

// Error / Lỗi
println!("{}", x);

// Correct / Đúng
println!("{:?}", x);   // Output: Some(6)
println!("{:#?}", x);  // Output: Some(6) with indentation
```

---

## 4. Debug vs Pretty-print Debug / So sánh Debug tiêu chuẩn và Debug căn chỉnh

**En:**
- `{:?}`: Compact, single-line output.
- `{:#?}`: Pretty-print, multi-line output with indentation (easier to read for complex data).

**Vi:**
- `{:?}`: Gọn gàng, in trên một dòng duy nhất.
- `{:#?}`: In đẹp, nhiều dòng kèm thụt lề (dễ đọc hơn đối với dữ liệu phức tạp).

---

## 5. Deep Dive into `.unwrap()` / Tìm hiểu sâu về `.unwrap()`

**En:**
In Rust, `unwrap()` is a common method used to extract the value contained inside an `Option<T>` or a `Result<T, E>`.

### What it does:
1.  **Successful Path**: If the `Option` is `Some(value)` or the `Result` is `Ok(value)`, `unwrap()` returns the inner `value`.
2.  **Failure Path**: If the `Option` is `None` or the `Result` is `Err(e)`, `unwrap()` will cause the program to **panic** (crash immediately) with an error message.

### Example in code:
```rust
let x = x.checked_add(i32::MAX).unwrap();
```
The `checked_add` method returns an `Option<i32>`. By calling `.unwrap()`, you are explicitly telling Rust: **"I am certain this addition will not overflow. If it does, crash the program."**

**Vi:**
Trong Rust, `unwrap()` là một phương thức phổ biến được sử dụng để lấy giá trị bên trong một `Option<T>` hoặc `Result<T, E>`.

### Nó làm gì:
1.  **Trường hợp thành công**: Nếu `Option` là `Some(value)` hoặc `Result` là `Ok(value)`, `unwrap()` sẽ trả về giá trị `value` bên trong.
2.  **Trường hợp thất bại**: Nếu `Option` là `None` hoặc `Result` là `Err(e)`, `unwrap()` sẽ khiến chương trình bị **panic** (sập ngay lập tức) cùng với một thông báo lỗi.

### Ví dụ trong mã:
```rust
let x = x.checked_add(i32::MAX).unwrap();
```
Phương thức `checked_add` trả về một `Option<i32>`. Việc gọi `.unwrap()` ở đây có nghĩa là bạn đang nói với Rust một cách rõ ràng: **"Tôi chắc chắn phép cộng này sẽ không bị tràn số. Nếu nó bị tràn, hãy làm sập chương trình."**

### Best Practices for Handling Potential Failures / Thực hành tốt nhất khi xử lý lỗi tiềm ẩn

**En:**
Using `unwrap()` is generally discouraged in production code because it makes your application fragile. Better alternatives include:

-   **`.expect("Custom message")`**: Similar to `unwrap()` but provides a more helpful error message upon crashing. Use this when failure is theoretically impossible but you want to document the assumption.
-   **`match` or `if let`**: Allows you to handle the `None` case gracefully (e.g., logging an error or returning a default value).
-   **`.unwrap_or(default_value)`**: Returns the success value or a provided fallback value if it fails.
-   **The `?` operator**: Propagates the error to the caller, allowing for centralized error handling.

**Vi:**
Việc sử dụng `unwrap()` thường không được khuyến khích trong mã nguồn thực tế (production) vì nó làm ứng dụng của bạn dễ bị sập đột ngột. Các giải pháp thay thế tốt hơn bao gồm:

-   **`.expect("Thông báo tùy chỉnh")`**: Tương tự như `unwrap()` nhưng cung cấp thông báo lỗi hữu ích hơn khi sập. Sử dụng cái này khi về mặt lý thuyết lỗi không thể xảy ra nhưng bạn muốn ghi lại giả định đó.
-   **`match` hoặc `if let`**: Cho phép bạn xử lý trường hợp `None` một cách an toàn (ví dụ: ghi log lỗi hoặc trả về một giá trị mặc định).
-   **`.unwrap_or(giá trị_mặc_định)`**: Trả về giá trị thành công hoặc một giá trị dự phòng nếu thất bại.
-   **Toán tử `?`**: Đưa lỗi lên cấp cao hơn (caller), cho phép xử lý lỗi tập trung.

---

## 7. Best Practices for specialized overflow handling methods / Thực hành tốt nhất cho các phương thức xử lý tràn số chuyên biệt

**En:**
Choosing the right method depends on how your application should behave when it hits the limits of a data type:

-   **`checked_*` (The Safest Choice)**: Use when overflow is an **error condition** that you must handle (e.g., financial transactions, inventory).
-   **`saturating_*` (The "Cap" Choice)**: Use when you want the value to stay at its **min/max** limit instead of wrapping (e.g., Game HP, UI Brightness, Volume).
-   **`wrapping_*` (The "Modulus" Choice)**: Use only when the logic **mathematically requires** wrapping (e.g., Cryptography, Hash functions, Circular buffers).
-   **`overflowing_*` (The "Diagnostic" Choice)**: Use when you need the wrapped result **and** a boolean flag to know if a wrap occurred (e.g., Custom numeric types, Specialized math libraries).

**Vi:**
Việc chọn đúng phương thức phụ thuộc vào cách ứng dụng của bạn nên hành xử khi chạm đến giới hạn của kiểu dữ liệu:

-   **`checked_*` (Lựa chọn an toàn nhất)**: Dùng khi việc tràn số được coi là một **trạng thái lỗi** mà bạn phải xử lý (ví dụ: giao dịch tài chính, tồn kho).
-   **`saturating_*` (Lựa chọn "Chạm trần/Sàn")**: Dùng khi bạn muốn giá trị dừng lại ở **giới hạn tối thiểu/tối đa** thay vì quay vòng (ví dụ: Máu/HP trong game, Độ sáng UI, Âm lượng).
-   **`wrapping_*` (Lựa chọn "Quay vòng")**: Chỉ dùng khi logic **yêu cầu về mặt toán học** hành vi quay vòng (ví dụ: Mã hóa, Hàm băm, Bộ đệm vòng).
-   **`overflowing_*` (Lựa chọn "Chẩn đoán")**: Dùng khi bạn cần kết quả đã quay vòng **và** một cờ boolean để biết liệu việc quay vòng có xảy ra hay không (ví dụ: Kiểu số tùy chỉnh, Thư viện toán học chuyên biệt).