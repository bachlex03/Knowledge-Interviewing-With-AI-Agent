# TDD Foundation Q&A

### Level 1: Remembering

#### Q1: What is Test-Driven Development (TDD)?

**Question:**
en: What is Test-Driven Development (TDD)?
vi: Phát triển hướng kiểm thử (TDD) là gì?

**Answer:**
en: Test-Driven Development (TDD) is a software development process where you write a failing test first, then write the minimum code to pass it, and finally refactor.
vi: Phát triển hướng kiểm thử (TDD) là một quy trình phát triển phần mềm trong đó chúng ta sẽ viết test trước khi viết code cụ thể hơn là viết một kiểm thử thất bại trước tiên, sau đó viết code tối thiểu để pass những cái test đó, và cuối cùng là chúng ta sẽ refactor.

#### Q2: What are the primary benefits of using TDD?

**Question:**
en: What are the primary benefits of using TDD?
vi: Những lợi ích chính của việc sử dụng TDD là gì?

**Answer:**
en:
vi: Việc viết unit test trước viết code giúp cho các lập trình viên hiểu rõ những gì cần phải làm, giống như mình cần phải dừng lại 1 nhịp để suy nghĩ về những gì mình cần phải làm, suy nghĩ về cách triển khai và cách thức code hoạt động cũng như là ngăn ngừa viết những đoạn code không cần thiết (YAGNI), thay vì cứ nhận requirements rồi lao đầu vào code ngay. Cụ thể sẽ có những lợi ích như là:
**Giảm thiểu lỗi**: Vì mình đang phân tách code hoặc chia nhỏ vấn đề ra thành những unit test nhỏ, nên mình có thể dễ dàng phát hiện những trường hợp edge cases gây ra bug. Ngoài ra, việc phát hiện sớm giúp tiết kiệm chi phí và effort sửa lỗi cho sau này khi mà bug đó được phát hiện ở giai đoạn production.
**Tài liệu kĩ thuật**: Mỗi unit test đóng vai trò như là một tài liệu kĩ thuật, giúp cho người đọc hay những người mới tham gia vào dự án hiểu rõ hơn về cách thức hoạt động của chức năng.
**Refactor code mà không sợ gây lỗi cho hệ thống**: Tự tin refactor hoặc tối ưa code mà không sợ ảnh hưởng đến hệ thống vì có bộ test tự động để đảm bảo rằng các thay đổi không ảnh hưởng đến các chức năng khác của hệ thống.
**Tích hợp CI/CD**: Những unit tests này sẽ được chạy tự động trong pipeline CI/CD, giúp đảm bảo rằng code mới không gây ra lỗi cho hệ thống.

#### Q3: What are the three steps of the TDD cycle?

**Question:**
en: What are the three steps of the TDD cycle?
vi: Ba bước của chu kỳ TDD là gì?

**Answer:**
en:
vi:

1. **Red**: Viết test cho một chức năng cụ thể.
2. **Green**: Viết code tối thiểu để pass cái test đó.
3. **Refactor**: Refactor code để để để tối ưu hoặc sạch và vẫn đảm bảo pass test.

#### Q4: What does the "Red" state in TDD signify?

**Question:**
en: What does the "Red" state in TDD signify?
vi: Trạng thái "Đỏ" (Red) trong TDD có ý nghĩa gì?

**Answer:**
en: It signifies that a test has been written for a new feature or bug fix, but it fails because the implementation code does not yet exist or is incomplete.
vi: Nó biểu thị rằng một kiểm thử đã được viết cho một tính năng mới hoặc sửa lỗi, nhưng nó thất bại vì mã nguồn thực thi chưa tồn tại hoặc chưa hoàn thiện.

#### Q6: Define "Unit Testing" in the context of TDD.

**Question:**
en: Define "Unit Testing" in the context of TDD.
vi: Định nghĩa "Kiểm thử đơn vị" (Unit Testing) trong ngữ cảnh của TDD.

**Answer:**
en: Unit testing is the practice of testing the smallest possible parts of an application (units), such as functions or methods, in isolation to ensure they work correctly.
vi: Unit tests là việc kiểm thử các phần nhỏ nhất có thể của một ứng dụng (các đơn vị), chẳng hạn như các hàm hoặc phương thức hoặc 1 class, trong sự cô lập để đảm bảo chúng hoạt động chính xác.

---

### Level 2: Understanding

#### Q1: Explain the primary goal of the "Refactor" step in TDD.

**Question:**
en: Explain the primary goal of the "Refactor" step in TDD.
vi: Giải thích mục tiêu chính của bước "Refactor" trong TDD.

**Answer:**
en: The goal is to improve code structure and readability without changing external behavior, reducing technical debt from the "Green" phase.
vi: Mục tiêu là cải thiện chất lượng code hoặc cải thiện để dễ đọc hơn mà không làm thay đổi hành vi bên ngoài, giúp giảm nợ kỹ thuật phát sinh từ giai đoạn "Green".

#### Q2:

**Question:**
en:
vi: Lý do TDD là một phương pháp tuyệt vời?

**Answer:**
en:
vi:
**Thay đổi tư duy**: Ở phía backend khi viết code em tư duy như là developer và khi viết unit test em sẽ tư duy như là một tester, nghĩ là em sẽ phải hình dung ra những trường hợp có thể xảy ra, những trường hợp edge cases gây ra bug. Ngoài ra, em thường đặt mình là "người sử dụng chức năng đó" thay vì "người viết ra nó", khi mà em chuyển công việc thì người khác thay thể em thì người ta có thể dễ dàng maintain và hiểu được nghiệp vụ của chức năng đó tại vì unit tests cũng là tài liệu kỹ thuật.
**Ngăn chặn "Over-engineering (YAGNI - You Aren't Gonna Need It)"**: TDD ép chúng ta phải viết lượng code vừa đủ để pass test hiện tại, khi đó chúng ta sẽ không mất thời gian để viết ra những logic thừa thãi.
**Đảm bảo 100% Code có thể Test được (Testable)**: Khi chúng ta viết code trước thì sẽ dễ gây ra tình trạng code xong xuôi rồi mới nhận ra hàm này dính chặt với database, không thể Mock/Stub để test được. Với TDD, chúng ta viết test trước nên đoạn code sẽ 100% test được.

#### Q3: Contrast "Outside-In" vs "Inside-Out" TDD.

**Question:**
en: Briefly contrast "Outside-In" vs "Inside-Out" TDD approaches.
vi: Phân biệt ngắn gọn cách tiếp cận TDD "Từ ngoài vào" (Outside-In) và "Từ trong ra" (Inside-Out).

**Answer:**
en: **Outside-In** starts from high-level requirements (UI/APIs) and works down using mocks. **Inside-Out** (Classic TDD) starts from domain logic/units and builds up.
vi: **Outside-In** bắt đầu từ các yêu cầu cấp cao (Giao diện/API) và đi xuống dưới bằng cách sử dụng các đối tượng giả (mocks). **Inside-Out** (TDD cổ điển) bắt đầu từ logic nghiệp vụ/đơn vị cơ bản và xây dựng dần lên.

#### Q5: Explain the importance of "Minimum Viable Code" in the Green phase.

**Question:**
en: Explain the importance of "Minimum Viable Code" in the Green phase.
vi: Giải thích tầm quan trọng của "Mã nguồn tối thiểu" (Minimum Viable Code) trong giai đoạn Green.

**Answer:**
en: Writing only the code necessary to pass the test prevents over-engineering and keeps the implementation strictly bound to requirements.
vi: Việc chỉ viết code tối thiểu cần thiết để vượt qua unit test giúp ngăn ngừa viết những đoạn code thừa thãi hay còn gọi là over-engineering điều này giúp giữ cho việc triển khai bám sát các yêu cầu.

#### Q6:

**Question:**
en:
vi: Như thế nào là một unit test tốt.

**Answer:**
en:
vi: Khi mà em viết unit test thì em tuân thủ theo nguyên tắc FIRST và SOLID (S - Single Responsibility và O - Open/Closed và D - Dependency Inversion):

Cụ thể hơn với nguyên tắc SOLID:
**S - Single Responsibility**: asdasda.
**O - Open/Closed**: asdasda.
**D - Dependency Inversion**: asdasda.

Còn đối với nguyên tắc FIRST:
**F - Fast**: asdasda.
**I - Independent**: asdasda.
**R - Repeatable**: asdasda.
**S - Self-Validating**: asdasda.
**T - Timely**: asdasda.

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
test("detects simple palindrome", () => {
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
it("notifies user", () => {
  const mockEmail = { send: jest.fn() };
  const controller = new UserController(mockEmail);
  controller.signup("test@example.com");
  expect(mockEmail.send).toHaveBeenCalled();
});
```
