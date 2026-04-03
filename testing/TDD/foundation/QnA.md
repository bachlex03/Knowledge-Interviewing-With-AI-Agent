# TDD Foundation Q&A

### Level 1: Remembering
#### Q1: What are the three steps of the TDD cycle?
**Question:**
en: What are the three steps of the TDD cycle?
vi: Ba bước của chu kỳ TDD là gì?

**Answer:**
en: 1. **Red**: Write a failing test. 2. **Green**: Write the minimum code to make the test pass. 3. **Refactor**: Clean up the code while ensuring tests still pass.
vi: 1. **Red**: Viết một kiểm thử thất bại. 2. **Green**: Viết mã nguồn tối thiểu để làm cho kiểm thử vượt qua. 3. **Refactor**: Dọn dẹp mã nguồn trong khi đảm bảo các kiểm thử vẫn vượt qua.

#### Q2: What does the "Red" state in TDD signify?
**Question:**
en: What does the "Red" state in TDD signify?
vi: Trạng thái "Đỏ" (Red) trong TDD có ý nghĩa gì?

**Answer:**
en: It signifies that a test has been written for a new feature or bug fix, but it fails because the implementation code does not yet exist or is incomplete.
vi: Nó biểu thị rằng một kiểm thử đã được viết cho một tính năng mới hoặc sửa lỗi, nhưng nó thất bại vì mã nguồn thực thi chưa tồn tại hoặc chưa hoàn thiện.

#### Q3: List three common testing frameworks used for TDD.
**Question:**
en: List three common testing frameworks used for TDD across different languages.
vi: Liệt kê ba khung kiểm thử (testing frameworks) phổ biến được sử dụng cho TDD trong các ngôn ngữ khác nhau.

**Answer:**
en: 1. **JUnit** (Java), 2. **pytest** (Python), 3. **Jest** (JavaScript/TypeScript).
vi: 1. **JUnit** (Java), 2. **pytest** (Python), 3. **Jest** (JavaScript/TypeScript).

#### Q4: Define "Unit Testing" in the context of TDD.
**Question:**
en: Define "Unit Testing" in the context of TDD.
vi: Định nghĩa "Kiểm thử đơn vị" (Unit Testing) trong ngữ cảnh của TDD.

**Answer:**
en: Unit testing is the practice of testing the smallest possible parts of an application (units), such as functions or methods, in isolation to ensure they work correctly.
vi: Kiểm thử đơn vị là việc kiểm thử các phần nhỏ nhất có thể của một ứng dụng (các đơn vị), chẳng hạn như các hàm hoặc phương thức, trong sự cô lập để đảm bảo chúng hoạt động chính xác.

#### Q5: Who is credited with rediscovering/developing TDD in the modern era?
**Question:**
en: Who is credited with rediscovering/developing TDD in the modern era?
vi: Ai là người được ghi nhận là đã tái phát hiện/phát triển TDD trong kỷ nguyên hiện đại?

**Answer:**
en: Kent Beck.
vi: Kent Beck.

---

### Level 2: Understanding
#### Q1: Explain the primary goal of the "Refactor" step in TDD.
**Question:**
en: Explain the primary goal of the "Refactor" step in TDD.
vi: Giải thích mục tiêu chính của bước "Refactor" trong TDD.

**Answer:**
en: The goal is to improve code structure and readability without changing external behavior, reducing technical debt from the "Green" phase.
vi: Mục tiêu là cải thiện cấu trúc và tính dễ đọc của mã nguồn mà không làm thay đổi hành vi bên ngoài, giúp giảm nợ kỹ thuật phát sinh từ giai đoạn "Green".

#### Q2: What is the benefit of writing the test *before* the code?
**Question:**
en: What is the benefit of writing the test *before* the code?
vi: Lợi ích của việc viết kiểm thử *trước* khi viết mã nguồn là gì?

**Answer:**
en: It forces clear requirement definition, ensures the code is design-for-testability, and prevents writing unnecessary functionality (YAGNI).
vi: Nó bắt buộc phải định nghĩa rõ ràng các yêu cầu, đảm bảo mã nguồn được thiết kế để có thể kiểm thử được, và ngăn chặn việc viết các chức năng không cần thiết (YAGNI).

#### Q3: Contrast "Outside-In" vs "Inside-Out" TDD.
**Question:**
en: Briefly contrast "Outside-In" vs "Inside-Out" TDD approaches.
vi: Phân biệt ngắn gọn cách tiếp cận TDD "Từ ngoài vào" (Outside-In) và "Từ trong ra" (Inside-Out).

**Answer:**
en: **Outside-In** starts from high-level requirements (UI/APIs) and works down using mocks. **Inside-Out** (Classic TDD) starts from domain logic/units and builds up.
vi: **Outside-In** bắt đầu từ các yêu cầu cấp cao (Giao diện/API) và đi xuống dưới bằng cách sử dụng các đối tượng giả (mocks). **Inside-Out** (TDD cổ điển) bắt đầu từ logic nghiệp vụ/đơn vị cơ bản và xây dựng dần lên.

#### Q4: Why is TDD sometimes referred to as "Test-Driven Design"?
**Question:**
en: Why is TDD sometimes referred to as "Test-Driven Design"?
vi: Tại sao TDD đôi khi được gọi là "Thiết kế hướng kiểm thử" (Test-Driven Design)?

**Answer:**
en: Because the process of making code testable inherently leads to better design patterns, like low coupling and high cohesion.
vi: Bởi vì quá trình làm cho mã nguồn có thể kiểm thử được về bản chất sẽ dẫn đến các mẫu thiết kế tốt hơn, chẳng hạn như liên kết thấp (low coupling) và độ gắn kết cao (high cohesion).

#### Q5: Explain the importance of "Minimum Viable Code" in the Green phase.
**Question:**
en: Explain the importance of "Minimum Viable Code" in the Green phase.
vi: Giải thích tầm quan trọng của "Mã nguồn tối thiểu" (Minimum Viable Code) trong giai đoạn Green.

**Answer:**
en: Writing only the code necessary to pass the test prevents over-engineering and keeps the implementation strictly bound to requirements.
vi: Việc chỉ viết mã nguồn cần thiết để vượt qua kiểm thử giúp ngăn ngừa kỹ thuật quá mức (over-engineering) và giữ cho việc triển khai bám sát các yêu cầu.

---

### Level 3: Applying
#### Q1: Given a requirement to create a function that reverses a string, describe the first test case you would write following TDD.
**Question:**
en: Given a requirement to create a function that reverses a string, describe the first test case you would write following TDD.
vi: Với yêu cầu tạo một hàm đảo ngược chuỗi, hãy mô tả trường hợp kiểm thử (test case) đầu tiên bạn sẽ viết theo TDD.

**Answer:**
en: I would write a test that asserts an empty string returns an empty string.
vi: Tôi sẽ viết một kiểm thử khẳng định rằng một chuỗi rỗng trả về một chuỗi rỗng.

```rust
#[test]
fn test_reverse_empty() {
    assert_eq!(reverse(""), "");
}
```

#### Q2: Demonstrate a TDD test for a function that detects palindromes.
**Question:**
en: Demonstrate a TDD test for a function that detects palindromes.
vi: Minh họa một kiểm thử TDD cho một hàm phát hiện chuỗi đối xứng (palindromes).

**Answer:**
en: Write a test for a positive case like "radar".
vi: Viết một kiểm thử cho trường hợp đúng như "radar".

```javascript
test('detects simple palindrome', () => {
  expect(isPalindrome("radar")).toBe(true);
});
```

#### Q3: Use TDD to handle an edge case for a division function.
**Question:**
en: Use TDD to handle an edge case for a division function (e.g., division by zero).
vi: Sử dụng TDD để xử lý một trường hợp biên cho hàm chia (ví dụ: chia cho số không).

**Answer:**
en: Write a test that expects an error/exception when dividing by zero before implementing the check.
vi: Viết một kiểm thử mong đợi một lỗi/ngoại lệ khi chia cho số không trước khi thực hiện việc kiểm tra.

```python
def test_divide_by_zero():
    with pytest.raises(ZeroDivisionError):
        divide(10, 0)
```

#### Q4: Apply TDD to implement a `Stack`'s `push` method.
**Question:**
en: Apply TDD to implement a `Stack`'s `push` method.
vi: Áp dụng TDD để thực hiện phương thức `push` của một `Stack`.

**Answer:**
en: Test that the stack size increases and the top element matches after pushing.
vi: Kiểm thử xem kích thước ngăn xếp có tăng lên và phần tử trên cùng có khớp sau khi push hay không.

```java
@Test
public void testPushIncrementsSize() {
    Stack stack = new Stack();
    stack.push(1);
    assertEquals(1, stack.size());
}
```

#### Q5: Demonstrate how to mock a dependency in TDD.
**Question:**
en: Demonstrate how to mock a dependency (e.g., an Email Service) in a TDD test.
vi: Minh họa cách giả lập (mock) một phụ thuộc (ví dụ: Dịch vụ Email) trong một kiểm thử TDD.

**Answer:**
en: Use a mock object to verify that the `send` method was called without actually sending an email.
vi: Sử dụng một đối tượng giả để xác minh rằng phương thức `send` đã được gọi mà không thực sự gửi email.

```typescript
it('notifies user', () => {
  const mockEmail = { send: jest.fn() };
  const controller = new UserController(mockEmail);
  controller.signup("test@example.com");
  expect(mockEmail.send).toHaveBeenCalled();
});
```
