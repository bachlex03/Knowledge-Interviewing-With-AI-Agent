# TDD Foundation Q&A

### Level 1: Remembering

#### Q1: What is Test-Driven Development (TDD)?

**Question:**
en: What is Test-Driven Development (TDD)?
vi: Phát triển hướng kiểm thử (TDD) là gì?

**Answer:**
en: Test-Driven Development (TDD) is a software development process where you write a failing test first, then write the minimum code to pass it, and finally refactor.
vi: Phát triển hướng kiểm thử (TDD) là một quy trình phát triển phần mềm trong đó chúng ta sẽ viết test trước khi viết code cụ thể hơn là viết một kiểm thử thất bại trước tiên, sau đó viết code tối thiểu để pass những cái test đó, và cuối cùng là chúng ta sẽ refactor.

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

#### Q3:

**Question:**
en:
vi: Những nhược điểm chính của việc sử dụng TDD là gì?

**Answer:**
en:
vi:
**Tốn thời gian ban đầu**

- **Viết nhiều code hơn:** Điều dễ nhìn thấy nhất chắc chắn là mình sẽ phải viết code nhiều hơn. Thay vì chỉ viết 100 dòng code cho tính năng, mình phải viết thêm hơn 100 dòng code để test cái tính năng đó.
- **Thời gian thiết lập (Setup & Boilerplate):** Khi mà có chức năng mới, quá trình chuẩn bị (arrange) tạo ra các Mock/Stub (làm giả Database, làm giả API, làm giả thời gian...) thường khá tốn thời gian.
- **Xung đột với thời gian gắt gao Agile/Scum**: Trong mấy dự án làm theo kiểu "chạy nước rút" (Agile/Scrum với deadline gắt gao), các sếp hoặc khách hàng chỉ muốn sản phẩm chạy càng nhanh càng tốt. Những lúc này việc viết test là không cần thiết ban đầu.

**Dễ bị thay đổi nếu code logic/nghiệp vụ thay đổi**: Trong ba bước chu kỳ của TDD có bước refactor, người ta hay nói "có unit test thì sẽ dễ refactor" điều này đúng khi mà yêu cầu nghiệp vụ/business logic KHÔNG thay đổi, nếu nghiệp vụ thay đổi, bộ test cũng sẽ thay đổi theo.

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
**S - Single Responsibility**: Mỗi unit test chỉ làm 1 việc duy nhất, kiểm thử 1 logic duy nhất.
**O - Open/Closed**: Một class nên "mở" để mở rộng (thêm tính năng), nhưng "đóng" với sự sửa đổi (không sửa code cũ). Với unit test, mỗi khi có test case mới, chúng ta sẽ viết thêm 1 test case mới thay vì sửa code cũ.
**D - Dependency Inversion**: Với unit test, chúng ta sẽ sử dụng các đối tượng giả (mocks) để thay thế các dependency, giúp cô lập unit test và đảm bảo tính độc lập. Cụ thể hơn, là không phụ thuộc vào trực tiếp yếu tố biến động như Datetime.Now, database, file system, network, ... thay vào đó là phụ thuộc vào các abstraction (interface, abstract class, ...).

=> "testing/TDD/foundation/Q6": details

Còn đối với nguyên tắc FIRST:
**F - Fast**: Một unit test phải chạy nhanh, thường < 50ms. Vì một dự án có thể có từ vài ngàn đến hàng chục ngàn unit test. Nếu một bài test tốn 1s để chạy thì 10.000 sẽ tốn tận 3 tiếng để chạy xong. Khi test chạy quá chậm developer sẽ sinh ra tâm lý lười chạy test và thế là những con bug sẽ dễ dàng lọt qua.
**I - Isolated**: Mỗi unit test không phụ thuộc vào **môi trường bên ngoài** và không phụ thuộc vào các **bài test khác**.
**R - Repeatable**: Nói về sự nhất quán tuyệt đối. Dù chạy ở môi trường nào, thời điểm nào cũng ra đúng một kết quả (Pass hoặc Fail). Loaị bỏ các yếu tố có tính biến động như DateTime.Now, Random, hoặc tình trạng mạng Internet, database, file system, network, ...
**S - Self-Validating**: Các bài unit test phải tự đưa ra kết luận Pass hoặc Fail mà không cần con người phải kiểm tra thủ công. Vì trước đây nhiều người hay có thói quen in kết quả ra màn hình để kiếm chứng bằng mắt xem có đúng như kỳ vọng không. Việc này làm mất hoàn toàn ý nghĩa tự động hóa của Unit Test.
**T - Timely**: Thời điểm viết Unit Test rất quan trọng. Nó phải được viết một cách kịp thời. Trong triết lý TDD (Test-Driven Development), "kịp thời" có nghĩa là **viết Test TRƯỚC khi viết Code thực**. Nếu mà mình viết code xong xuôi hết rồi vài tháng sau mới quay lại viết test hay còn gọi là nợ kỹ thuật, lúc này sẽ xảy ra nhiều trường hợp: đầu tiên là code dính chặt với nhau và cực kỳ khó test, thứ hai là mình có thể sẽ quên mất logic nghiệp vụ lúc đó nếu viết test ban đầu thì nếu mình có quên thì sẽ có bài test nó sẽ nhắc mình. Nếu không viết test ban đầu thì lúc này việc viết test sẽ trở nên cực hình, gượng ép giống như là 1 gánh nặng.

#### Q7: ...

**Question:**
en:
vi: Những lưu ý khi viết unit test.

**Answer:**
en:
vi:

- **Chỉ test hành vi - Testing Behavior, không test chi tiết triển khai - Testing Implementation:** Mình sẽ xem hình hàm như hộp đen (black box) mình chỉ quan tâm đến input và output của hàm, không quan tâm đến cách thức triển khai bên trong. Điều này thể hiện rõ bước Refactor trong TDD và cũng đảm bảo Repeatable trong nguyên tắc FIRST.
- **Hạn chế nhiều assert:** Mỗi test case chỉ nên có 1 assert vì nhiều assert thì có test fail mình sẽ không biết lỗi nằm ở đâu trong test đó.
- **Trách logic trong test, viết đơn giản:** Không dùng if/else, try/catch, for, while, ... trong test case. Chỉ dùng assert.
- **Đảm bảo nguyên tắc S (Single Responsibility):** Mỗi unit test chỉ làm 1 việc duy nhất, kiểm thử 1 logic duy nhất. **Ví dụ** viết hàm áp dụng giảm giá cho ngày thứ ba và áp dụng giảm giá cho ngày thứ sáu là 2 test case khác nhau.
- **Đảm bảo nguyên tắc O (Open/Closed):** Giúp dễ dàng viết test hơn khi có thêm yêu cầu mới. **Ví dụ** trong bài toán "GetDiscountedPrice" ở trên, nếu không tuân thủ OCP thì khi có thêm yêu cầu mới, mình sẽ phải sửa lại class `PriceCalculator` và viết thêm một dòng `else if (Thứ Sáu)` nào cả. Điều này vi phạm nguyên tắc OCP.
- **Đảm bảo nguyên tắc D (Dependency Inversion):** Giúp cô lập unit test và đảm bảo tính độc lập. Cụ thể hơn, là không phụ thuộc vào trực tiếp yếu tố biến động như Datetime.Now, database, file system, network, ... thay vào đó là phụ thuộc vào các abstraction (interface, abstract class, ...) khi đó mình có thể Mock/Stub các dependency đó.
- **Đảm bảo nguyên tắc FIRST:**

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
