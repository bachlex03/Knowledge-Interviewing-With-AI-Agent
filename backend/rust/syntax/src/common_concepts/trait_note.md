# Rust Trait Notes

## 1. Display Implementation

### Why Use the `Display` Trait?

The `Display` trait is used for user-facing output. Implementing it allows you to use the `{}` placeholder in macros like `println!`, `format!`, and `write!`.

Trait `Display` được dùng cho đầu ra dành cho người dùng cuối. Việc triển khai trait này cho phép bạn sử dụng ký tự giữ chỗ `{}` trong các macro như `println!`, `format!`, và `write!`.

#### Anatomy of the Implementation

```rust
impl Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}
```

- **`impl Display for Article`**: EN: Provides behavior for `Article`. VI: Cung cấp hành vi cho `Article`.
- **`f: &mut Formatter<'_>`**: EN: Mutable output buffer. VI: Bộ đệm đầu ra có thể thay đổi.
- **`'_`**: EN: Anonymous Lifetime. VI: Thời gian sống ẩn danh.
- **`write!`**: EN: Macro to write data; result is returned without semicolon. VI: Macro để ghi dữ liệu; kết quả được trả về trực tiếp (không dấu chấm phẩy).

#### Comparison: Display vs Debug

| Feature | `Display` (`{}`) | `Debug` (`{:?}`) |
| --- | --- | --- |
| **Purpose** | User-facing, clean output | Developer-facing, diagnostic output |
| **Implementation** | Manual (usually) | Automatic (via `#[derive(Debug)]`) |

---

## 2. Comparison Traits: PartialEq & PartialOrd

These traits enable the use of comparison operators like `==`, `!=`, `<`, `>`, etc.

Các trait này cho phép sử dụng các toán tử so sánh như `==`, `!=`, `<`, `>`, v.v.

### PartialEq (Partial Equality)

- **Purpose (EN)**: Enables equality operators: `==` and `!=`. When derived, it compares all fields of the struct.
- **Mục đích (VI)**: Cho phép các toán tử so sánh bằng: `==` và `!=`. Khi dùng `derive`, Rust sẽ so sánh tất cả các trường dữ liệu của struct.
- **Why "Partial"?**: Some types (like floating-point `f64`) have values like `NaN` which are not equal to themselves. Thus, it is "Partial" rather than total equality (`Eq`).
- **Tại sao là "Partial"?**: Một số kiểu (như số thực `f64`) có các giá trị như `NaN` (không bằng chính nó). Do đó, nó là sự bằng nhau "một phần" thay vì hoàn toàn (`Eq`).

### PartialOrd (Partial Order)

- **Purpose (EN)**: Enables relational operators: `<`, `>`, `<=`, and `>=`. It depends on `PartialEq`.
- **Mục đích (VI)**: Cho phép các toán tử so sánh thứ tự: `<`, `>`, `<=`, và `>=`. Trait này phụ thuộc vào `PartialEq`.
- **How it works**: Performs lexicographical comparison. For simple structs, it compares the inner values in order.
- **Cách hoạt động**: Thực hiện so sánh theo thứ tự từ điển (lexicographical). Với struct đơn giản, nó so sánh các giá trị bên trong theo thứ tự.

---

## 3. Operator Overloading: `std::ops` & Associated Types

Operator overloading allows you to define how built-in operators (like `+`, `-`, `*`) behave for your custom types.

Nạp chồng toán tử cho phép bạn định nghĩa cách các toán tử có sẵn (như `+`, `-`, `*`) hoạt động trên các kiểu dữ liệu tùy chỉnh.

### `std::ops` Module

- **Purpose (EN)**: This module contains traits that correspond to built-in operators. For example, `std::ops::Add` handles the `+` operator.
- **Mục đích (VI)**: Module này chứa các trait tương ứng với các toán tử có sẵn. Ví dụ, `std::ops::Add` xử lý toán tử `+`.
- **Mechanism**: Use of an operator is "syntactic sugar" for calling a trait method (e.g., `a + b` becomes `a.add(b)`).
- **Cơ chế**: Việc sử dụng toán tử là "cú pháp ngọt" (syntactic sugar) cho việc gọi một phương thức của trait (ví dụ: `a + b` trở thành `a.add(b)`).

### Associated Types (`type Output = ...`)

```rust
impl ops::Add<Bar> for Foo {
    type Output = FooBar; // Associated Type

    fn add(self, _rhs: Bar) -> FooBar { ... }
}
```

- **Definition (EN)**: An **Associated Type** is a placeholder used within a trait that the implementation must define. 
- **Định nghĩa (VI)**: Một **Associated Type** (kiểu liên kết) là một ký tự giữ chỗ được sử dụng trong trait mà phần triển khai bắt buộc phải xác định cụ thể.
- **Why it matters**: It allows the result of an operation to be a different type than the inputs. By setting `type Output = FooBar;`, you guarantee that adding `Foo` and `Bar` always results in a `FooBar`.
- **Tại sao quan trọng**: Nó cho phép kết quả của một phép tính mang một kiểu dữ liệu khác với các toán hạng đầu vào. Việc thiết lập `type Output = FooBar;` đảm bảo rằng cộng `Foo` và `Bar` sẽ luôn trả về một `FooBar`.

---

## 4. Iterator Trait: `Option<Self::Item>`

The `Iterator` trait is used to define how to step through a sequence of values.

Trait `Iterator` được dùng để định nghĩa cách duyệt qua một chuỗi các giá trị.

### Anatomy of `next` Signature

```rust
impl Iterator for Fibonacci {
    type Item = u32; // Associated Type

    fn next(&mut self) -> Option<Self::Item> { ... }
}
```

- **`Option<...>` (EN)**: The `next` method returns `Some(value)` if there is another element, or `None` if the iteration is finished.
- **`Option<...>` (VI)**: Phương thức `next` trả về `Some(value)` nếu còn phần tử tiếp theo, hoặc `None` nếu quá trình duyệt đã kết thúc.
- **`Self` (EN)**: Refers to the current implementation (the `Fibonacci` iterator).
- **`Self` (VI)**: Tham chiếu đến phần triển khai hiện tại (iterator `Fibonacci`).
- **`::Item` (EN)**: Refers to the associated type `Item` defined in the implementation (`u32` in this case).
- **`::Item` (VI)**: Tham chiếu đến associated type `Item` được định nghĩa trong phần triển khai (trong trường hợp này là `u32`).

### Why use `Self::Item`?

- **EN**: Using `Self::Item` instead of writing the concrete type (like `u32`) ensures consistency with the trait definition and makes the code easier to maintain if the item type changes.
- **VI**: Sử dụng `Self::Item` thay vì viết kiểu cụ thể (như `u32`) đảm bảo tính nhất quán với định nghĩa của trait và giúp mã nguồn dễ bảo trì hơn nếu kiểu dữ liệu của phần tử thay đổi.

---

## 5. Supertraits: `trait SubTrait: SuperTrait`

Supertraits allow you to define a trait that requires another trait to be implemented first.

Supertraits cho phép bạn định nghĩa một trait yêu cầu một trait khác phải được triển khai trước đó.

### Syntax & Relationship

```rust
trait Person {
    fn name(&self) -> String;
}

// Student is a subtrait; Person is a supertrait
trait Student: Person {
    fn university(&self) -> String;
}
```

- **Requirement (EN)**: Any type that implements `Student` **must** also implement `Person`.
- **Ràng buộc (VI)**: Bất kỳ kiểu dữ liệu nào triển khai `Student` **bắt buộc** cũng phải triển khai `Person`.
- **Hierarchy (EN)**: It represents specialization. A `Student` "is a" `Person` with extra capabilities (`university()`).
- **Phân cấp (VI)**: Nó đại diện cho sự chuyên biệt hóa. Một `Student` "là một" `Person` với các khả năng bổ sung (`university()`).

### Core Benefits

- **Access (EN)**: Methods in `Student` can call methods from `Person` because the compiler guarantees they exist.
- **Truy cập (VI)**: Các phương thức trong `Student` có thể gọi các phương thức từ `Person` vì trình biên dịch đảm bảo chúng luôn tồn tại.
- **Organization (EN)**: Helps build complex interfaces from simpler, reusable components (similar to inheritance in other languages).
- **Tổ chức (VI)**: Giúp xây dựng các giao diện phức tạp từ các thành phần đơn giản và có thể tái sử dụng (tương tự như kế thừa trong các ngôn ngữ khác).

---

---

## 6. Implementation Methods & Destructuring

This section covers the syntax for defining methods and using pattern matching to extract data from structs (especially tuple structs) within those methods.

Phần này bao gồm cú pháp định nghĩa phương thức và sử dụng khớp mẫu để trích xuất dữ liệu từ struct (đặc biệt là tuple struct) bên trong các phương thức đó.

### Anatomy of a Conversion Method

```rust
impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(value) = self;
        Centimeters(value * 2.54)
    }
}
```

- **`impl Inches` (EN)**: Declares an implementation block for the `Inches` type.
- **`impl Inches` (VI)**: Khai báo một khối triển khai cho kiểu dữ liệu `Inches`.
- **`&self` (EN)**: Immutably borrows the current instance; allows access to its data without taking ownership.
- **`&self` (VI)**: Mượn thực thể hiện tại ở dạng bất biến; cho phép truy cập dữ liệu mà không chiếm quyền sở hữu.
- **`let &Inches(value) = self;` (EN)**: Uses **Pattern Matching** to destructure the struct. The `&` matches the reference `self`, and `Inches(value)` extracts the inner value.
- **`let &Inches(value) = self;` (VI)**: Sử dụng **Khớp mẫu (Pattern Matching)** để giải cấu trúc struct. Ký tự `&` khớp với tham chiếu `self`, và `Inches(value)` trích xuất giá trị bên trong.
- **Implicit Return (EN)**: The expression `Centimeters(...)` lacks a semicolon, so its value is automatically returned.
- **Trả về ngầm định (VI)**: Biểu thức `Centimeters(...)` không có dấu chấm phẩy, vì vậy giá trị của nó tự động được trả về.

---

## 7. Fully Qualified Syntax (Disambiguation)

Fully qualified syntax is used to call a specific trait's method when multiple traits share the same method name for a type.

Cú pháp định danh đầy đủ được dùng để gọi phương thức của một trait cụ thể khi có nhiều trait cùng chia sẻ một tên phương thức cho cùng một kiểu dữ liệu.

### Syntax & Usage

```rust
let username = <Form as UsernameWidget>::get(&form);
```

- **`<Type as Trait>` (EN)**: Tells the compiler to treat the concrete type as a specific trait implementer.
- **`<Type as Trait>` (VI)**: Báo cho trình biên dịch coi kiểu dữ liệu cụ thể như là một thực thể triển khai của một trait nhất định.
- **`::method(...)` (EN)**: Calls the specific method. Because this is a static-style call, you must manually pass the instance (`&form`) as the first argument.
- **`::method(...)` (VI)**: Gọi phương thức cụ thể. Vì đây là lời gọi theo kiểu tĩnh, bạn phải truyền thực thể (`&form`) vào làm tham số đầu tiên theo cách thủ công.

### When to use it?

- **EN**: Use it when `form.get()` is ambiguous because both `UsernameWidget` and `AgeWidget` define a `get` method for the `Form` struct.
- **VI**: Sử dụng khi `form.get()` gây ra sự nhập nhằng vì cả `UsernameWidget` và `AgeWidget` đều định nghĩa phương thức `get` cho struct `Form`.

---

## Summary

- **Implementing `Display`** defines how your data is shown to the public. It requires handling a mutable formatter and returning a `fmt::Result`.
- **Deriving `PartialEq` and `PartialOrd`** gives your custom types instant comparison powers, essential for sorting or conditional logic (e.g., `if a < b`).
- **Overloading Operators** via `std::ops` lets you use standard math/logic syntax for your types. **Associated Types** (like `type Output`) define what type the operation results in.
- **Implementing `Iterator`** requires defining an `Item` type and a `next` method that returns an `Option<Self::Item>`, signaling the presence or end of data.
- **Defining Supertraits** (`trait A: B`) creates a dependency where behavior 'A' requires behavior 'B'. This allows for specialized, hierarchical trait design.
- **Implementation Blocks** (`impl`) allow you to define methods where you can use **Pattern Matching** (e.g., `let &Inches(val) = self`) to concisely extract and process internal data.
- **Fully Qualified Syntax** (`<Type as Trait>::method`) resolves ambiguity when multiple traits implement methods with the same name for the same type.

- **Triển khai `Display`** định nghĩa cách dữ liệu hiển thị cho người dùng. Nó đòi hỏi xử lý buffer và trả về `fmt::Result`.
- **Derive `PartialEq` và `PartialOrd`** cung cấp cho các kiểu dữ liệu tùy chỉnh khả năng so sánh tức thì, cần thiết cho việc sắp xếp hoặc logic điều kiện (ví dụ: `if a < b`).
- **Nạp chồng toán tử** thông qua `std::ops` cho phép bạn sử dụng cú pháp toán học/logic tiêu chuẩn cho các kiểu dữ liệu của mình. **Associated Types** (như `type Output`) định nghĩa kiểu kết quả của phép tính đó.
- **Triển khai `Iterator`** đòi hỏi định nghĩa kiểu `Item` và phương thức `next` trả về `Option<Self::Item>`, báo hiệu sự tồn tại hoặc kết thúc của dữ liệu.
- **Định nghĩa Supertraits** (`trait A: B`) tạo ra một sự phụ thuộc nơi hành vi 'A' yêu cầu phải có hành vi 'B'. Điều này cho phép thiết kế các trait theo thứ bậc và chuyên biệt hóa.
- **Khối triển khai** (`impl`) cho phép định nghĩa các phương thức mà bạn có thể dùng **Khớp mẫu** (ví dụ: `let &Inches(val) = self`) để trích xuất và xử lý dữ liệu nội bộ một cách ngắn gọn.
- **Fully Qualified Syntax** (`<Type as Trait>::method`) giải quyết sự nhập nhằng khi có nhiều trait cùng triển khai các phương thức trùng tên cho cùng một kiểu dữ liệu.
