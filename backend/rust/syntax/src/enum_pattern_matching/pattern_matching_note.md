# Patterns and Matching Notes / Ghi chú về Patterns và Matching

## 1. The `@` Binding Sigil / Ký hiệu `@` để gán biến khi khớp mẫu

**En:**
The `@` operator allows you to test a value against a pattern while simultaneously binding that value to a variable.

- **Why use it**: It lets you capture the value that matched a specific range or condition so you can use it within the match arm's block.
- **Example**:
  ```rust
  match msg {
      Message::Hello { id: id @ 3..=7 } => {
          println!("Found an id in range: {}", id);
      }
      _ => (),
  }
  ```
- **Comparison**:
  - `id @ 3..=7`: Matches the range AND saves the value into `id`.
  - `3..=7`: Matches the range but DOES NOT save the value for use.
  - `id`: Saves the value into `id` but DOES NOT check any specific range.

**Vi:**
Toán tử `@` cho phép bạn vừa kiểm tra một giá trị với một khuôn mẫu (pattern), vừa đồng thời gán giá trị đó vào một biến.

- **Tại sao sử dụng**: Nó giúp bạn "bắt" (capture) lấy giá trị đã khớp với một khoảng hoặc điều kiện cụ thể để bạn có thể sử dụng nó bên trong khối lệnh của nhánh match.
- **Ví dụ**:
  ```rust
  match msg {
      Message::Hello { id: id @ 3..=7 } => {
          println!("Tìm thấy id trong khoảng: {}", id);
      }
      _ => (),
  }
  ```
- **So sánh**:
  - `id @ 3..=7`: Khớp với khoảng giá trị VÀ lưu giá trị vào biến `id`.
  - `3..=7`: Khớp với khoảng giá trị nhưng KHÔNG lưu lại giá trị để sử dụng.
  - `id`: Lưu giá trị vào biến `id` nhưng KHÔNG kiểm tra khoảng giá trị cụ thể nào.
