# SOLID Foundation Q&A

### Level 1: Remembering
#### Q1: What does each letter in SOLID stand for?
**Question:**
en: What does each letter in the SOLID acronym stand for?
vi: Mỗi chữ cái trong từ viết tắt SOLID đại diện cho điều gì?

**Answer:**
en: **S**: Single Responsibility Principle, **O**: Open/Closed Principle, **L**: Liskov Substitution Principle, **I**: Interface Segregation Principle, **D**: Dependency Inversion Principle.
vi: **S**: Nguyên tắc đơn trách nhiệm, **O**: Nguyên tắc Đóng/Mở, **L**: Nguyên tắc thay thế Liskov, **I**: Nguyên tắc phân tách giao diện, **D**: Nguyên tắc đảo ngược phụ thuộc.

#### Q2: What is the definition of the Single Responsibility Principle (SRP)?
**Question:**
en: What is the primary definition of the Single Responsibility Principle (SRP) as stated by Robert C. Martin?
vi: Định nghĩa chính của Nguyên tắc đơn trách nhiệm (SRP) theo Robert C. Martin là gì?

**Answer:**
en: "A class should have one, and only one, reason to change."
vi: "Một lớp chỉ nên có một, và chỉ một, lý do để thay đổi."

#### Q3: Define the Open/Closed Principle (OCP).
**Question:**
en: Define the Open/Closed Principle (OCP).
vi: Định nghĩa Nguyên tắc Đóng/Mở (OCP).

**Answer:**
en: Software entities (classes, modules, functions, etc.) should be open for extension, but closed for modification.
vi: Các thực thể phần mềm (lớp, mô-đun, hàm, v.v.) nên được mở để mở rộng, nhưng đóng đối với việc sửa đổi.

#### Q4: What is the core rule of the Liskov Substitution Principle (LSP)?
**Question:**
en: What is the core rule of the Liskov Substitution Principle (LSP)?
vi: Quy tắc cốt lõi của Nguyên tắc thay thế Liskov (LSP) là gì?

**Answer:**
en: Objects of a superclass should be replaceable with objects of its subclasses without breaking the application.
vi: Các đối tượng của lớp cha phải có khả năng được thay thế bằng các đối tượng của lớp con mà không làm hỏng ứng dụng.

#### Q5: State the purpose of the Interface Segregation Principle (ISP).
**Question:**
en: State the primary purpose of the Interface Segregation Principle (ISP).
vi: Nêu mục đích chính của Nguyên tắc phân tách giao diện (ISP).

**Answer:**
en: Clients should not be forced to depend upon interfaces that they do not use.
vi: Các đối tượng sử dụng (clients) không nên bị buộc phải phụ thuộc vào các giao diện mà họ không sử dụng.

#### Q6: Identify the origin of the SOLID acronym.
**Question:**
en: Who introduced the SOLID acronym and in which book were these principles popularized?
vi: Ai là người đã giới thiệu từ viết tắt SOLID và những nguyên tắc này đã được phổ biến trong cuốn sách nào?

**Answer:**
en: The acronym was introduced by Michael Feathers around 2004, but the principles were established by Robert C. Martin (Uncle Bob) in his books like "Agile Software Development, Principles, Patterns, and Practices."
vi: Từ viết tắt được Michael Feathers giới thiệu vào khoảng năm 2004, nhưng các nguyên tắc này được khởi xướng bởi Robert C. Martin (Uncle Bob) trong các cuốn sách như "Agile Software Development, Principles, Patterns, and Practices."

#### Q7: List two signs of "Bad Design" that SOLID aims to fix.
**Question:**
en: List two of the four characteristics of "Bad Design" (Rigidity, Fragility, Immobility, Viscosity) that SOLID aims to address.
vi: Liệt kê hai trong bốn đặc điểm của "Thiết kế tồi" (Cứng nhắc, Dễ vỡ, Bất động, Nhớt) mà SOLID hướng tới giải quyết.

**Answer:**
en: 1. **Rigidity**: The design is hard to change. 2. **Fragility**: The design is easy to break.
vi: 1. **Tính cứng nhắc (Rigidity)**: Thiết kế khó thay đổi. 2. **Tính dễ vỡ (Fragility)**: Thiết kế dễ bị hỏng (lỗi).

#### Q8: Name the principle that prevents "Fragility" caused by cascading changes.
**Question:**
en: Which SOLID principle primarily prevents "Fragility" caused by modification that breaks unrelated parts of the system?
vi: Nguyên tắc SOLID nào chủ yếu ngăn chặn "Tính dễ vỡ" do việc sửa đổi làm hỏng các phần không liên quan của hệ thống?

**Answer:**
en: The Open/Closed Principle (OCP) and Single Responsibility Principle (SRP).
vi: Nguyên tắc Đóng/Mở (OCP) và Nguyên tắc đơn trách nhiệm (SRP).

#### Q9: Define a "client" in the context of Interface Segregation.
**Question:**
en: In the context of the Interface Segregation Principle (ISP), what does the term "client" refer to?
vi: Trong ngữ cảnh của Nguyên tắc phân tách giao diện (ISP), thuật ngữ "client" ám chỉ điều gì?

**Answer:**
en: A "client" is any piece of code (class or module) that uses or depends on a particular interface.
vi: "Client" là bất kỳ đoạn mã nào (lớp hoặc mô-đun) sử dụng hoặc phụ thuộc vào một giao diện cụ thể.

#### Q10: Identify the difference between high-level and low-level modules in DIP.
**Question:**
en: According to the Dependency Inversion Principle (DIP), what is a high-level module?
vi: Theo Nguyên tắc đảo ngược phụ thuộc (DIP), mô-đun cấp cao là gì?

**Answer:**
en: High-level modules contain the core business logic and policy of the application, while low-level modules handle detailed operations (like database or network access).
vi: Các mô-đun cấp cao chứa logic nghiệp vụ cốt lõi và các chính sách của ứng dụng, trong khi các mô-đun cấp thấp xử lý các hoạt động chi tiết (như truy cập cơ sở dữ liệu hoặc mạng).

---

### Level 2: Understanding
#### Q1: Explain why SRP helps in reducing code coupling.
**Question:**
en: Explain why the Single Responsibility Principle (SRP) helps in reducing code coupling.
vi: Giải thích tại sao Nguyên tắc đơn trách nhiệm (SRP) giúp giảm thiểu sự liên kết (coupling) trong mã nguồn.

**Answer:**
en: By limiting a class to one responsibility, changes to one feature won't affect unrelated functions, making the system more modular and less prone to side effects.
vi: Bằng cách giới hạn một lớp chỉ có một trách nhiệm, các thay đổi đối với một tính năng sẽ không ảnh hưởng đến các chức năng không liên quan, làm cho hệ thống trở nên mô-đun hơn và ít bị ảnh hưởng bởi các tác dụng phụ.

#### Q2: Describe how OCP facilitates adding new features.
**Question:**
en: Describe how the Open/Closed Principle (OCP) facilitates adding new features to a system.
vi: Mô tả cách Nguyên tắc Đóng/Mở (OCP) tạo điều kiện thuận lợi cho việc thêm các tính năng mới vào một hệ thống.

**Answer:**
en: Instead of editing existing tested code (which might introduce bugs), you add new functionality by creating new classes that inherit from or implement common abstractions.
vi: Thay vì chỉnh sửa mã nguồn đã được kiểm thử (điều có thể gây ra lỗi), bạn thêm chức năng mới bằng cách tạo các lớp mới kế thừa hoặc thực thi các trừu tượng chung.

#### Q3: Discuss the importance of LSP for polymorphism.
**Question:**
en: Discuss the importance of the Liskov Substitution Principle (LSP) for ensuring reliable polymorphism.
vi: Thảo luận về tầm quan trọng của Nguyên tắc thay thế Liskov (LSP) trong việc đảm bảo tính đa hình đáng tin cậy.

**Answer:**
en: It ensures that any code written to work with a base class will work correctly with any subclass, maintaining predictable behavior across an inheritance hierarchy.
vi: Nó đảm bảo rằng bất kỳ mã nguồn nào được viết để hoạt động với lớp cha sẽ hoạt động chính xác với bất kỳ lớp con nào, duy trì hành vi có thể dự đoán được trong suốt hệ thống kế thừa.

#### Q4: Explain the problem that Interface Segregation (ISP) aims to solve.
**Question:**
en: Explain the problem of "Fat Interfaces" that Interface Segregation (ISP) aims to solve.
vi: Giải thích vấn đề "Giao diện béo" (Fat Interfaces) mà Nguyên tắc phân tách giao diện (ISP) hướng tới giải quyết.

**Answer:**
en: Fat interfaces contain too many methods, forcing subclasses to implement methods they don't need, leading to empty method bodies or runtime errors.
vi: Các giao diện béo chứa quá nhiều phương thức, buộc các lớp con phải thực thi các phương thức mà chúng không cần, dẫn đến các thân phương thức rỗng hoặc lỗi thời gian chạy.

#### Q5: Describe the essence of Dependency Inversion (DIP).
**Question:**
en: Describe the essence of the Dependency Inversion Principle (DIP).
vi: Mô tả bản chất của Nguyên tắc đảo ngược phụ thuộc (DIP).

**Answer:**
en: High-level modules should not depend on low-level modules; both should depend on abstractions. Abstractions should not depend on details; details should depend on abstractions.
vi: Các mô-đun cấp cao không nên phụ thuộc vào các mô-đun cấp thấp; cả hai nên phụ thuộc vào các trừu tượng. Các trừu tượng không nên phụ thuộc vào các chi tiết; các chi tiết nên phụ thuộc vào các trừu tượng.

#### Q6: Explain "Interface Pollution" in the context of ISP.
**Question:**
en: Explain the concept of "Interface Pollution" in the context of violating the Interface Segregation Principle (ISP).
vi: Giải thích khái niệm "Ô nhiễm giao diện" (Interface Pollution) trong ngữ cảnh vi phạm Nguyên tắc phân tách giao diện (ISP).

**Answer:**
en: Interface pollution occurs when an interface is cluttered with methods that aren't needed by all its implementations, forcing them to implement "stub" methods.
vi: Ô nhiễm giao diện xảy ra khi một giao diện bị lấp đầy bởi các phương thức không cần thiết cho tất cả các bản thực thi của nó, buộc chúng phải thực thi các phương thức "stub" (rỗng).

#### Q7: Describe how LSP prevents "runtime type checking."
**Question:**
en: Describe how following the Liskov Substitution Principle (LSP) helps prevent excessive "runtime type checking" (like `instanceof` or `isType`).
vi: Mô tả cách việc tuân theo Nguyên tắc thay thế Liskov (LSP) giúp ngăn chặn việc "kiểm tra kiểu lúc chạy" quá mức (như `instanceof` hoặc `isType`).

**Answer:**
en: If subclasses consistently follow base class behavior, the client code doesn't need to ask "what specific type is this?" before calling a method, resulting in cleaner, polymorphic code.
vi: Nếu các lớp con tuân thủ nhất quán hành vi của lớp cha, mã nguồn client không cần hỏi "đây là kiểu cụ thể nào?" trước khi gọi một phương thức, dẫn đến mã nguồn đa hình sạch sẽ hơn.

#### Q8: Discuss the relationship between Abstraction and OCP.
**Question:**
en: Discuss why "Abstraction" is the key to effectively applying the Open/Closed Principle (OCP).
vi: Thảo luận tại sao "Trừu tượng hóa" (Abstraction) là chìa khóa để áp dụng hiệu quả Nguyên tắc Đóng/Mở (OCP).

**Answer:**
en: Abstraction creates a stable interface that is "closed" for changes, while allowing different "open" implementations to be plugged in to change the application's behavior.
vi: Trừu tượng hóa tạo ra một giao diện ổn định "đóng" đối với các thay đổi, đồng thời cho phép các bản thực thi "mở" khác nhau được cắm vào để thay đổi hành vi của ứng dụng.

#### Q9: Explain why DIP encourages the use of Interfaces/Abstract classes.
**Question:**
en: Explain why the Dependency Inversion Principle (DIP) encourages depending on Interfaces or Abstract classes rather than concrete implementations.
vi: Giải thích tại sao Nguyên tắc đảo ngược phụ thuộc (DIP) khuyến khích việc phụ thuộc vào Giao diện hoặc lớp Trừu tượng thay vì các bản thực thi cụ thể.

**Answer:**
en: Interfaces are more stable than concrete classes. Depending on an interface allows you to swap low-level implementations without changing the high-level policy code.
vi: Giao diện ổn định hơn các lớp cụ thể. Phụ thuộc vào một giao diện cho phép bạn hoán đổi các bản thực thi cấp thấp mà không cần thay đổi mã chính sách cấp cao.

#### Q10: Summarize how SRP affects the size of individual classes.
**Question:**
en: Summarize how the Single Responsibility Principle (SRP) generally affects the physical size and complexity of individual classes in a system.
vi: Tóm tắt cách Nguyên tắc đơn trách nhiệm (SRP) thường ảnh hưởng đến kích thước vật lý và độ phức tạp của từng lớp riêng lẻ trong hệ thống.

**Answer:**
en: SRP leads to smaller, more focused classes. While it increases the total number of classes, each class is simpler and easier to understand, test, and debug in isolation.
vi: SRP dẫn đến các lớp nhỏ hơn và tập trung hơn. Mặc dù nó làm tăng tổng số lượng các lớp, nhưng mỗi lớp lại đơn giản hơn và dễ hiểu, dễ kiểm thử và gỡ lỗi hơn trong sự cô lập.

---

### Level 3: Applying
#### Q1: Demonstrate SRP by refactoring a "Service" class.
**Question:**
en: Demonstrate SRP by refactoring a class that both processes payments and logs to a file.
vi: Minh họa SRP bằng cách tái cấu trúc một lớp vừa xử lý thanh toán vừa ghi log vào tệp.

**Answer:**
en: Split the payment logic into its own class and use a separate logger class.
vi: Chia logic thanh toán thành lớp riêng và sử dụng một lớp ghi log riêng biệt.

```csharp
// Before violation
public class PaymentService {
    public void Process() { /* ... */ }
    public void LogToFile(string msg) { /* ... */ }
}

// After SRP
public class PaymentProcessor {
    public void Process() { /* ... */ }
}
public class FileLogger {
    public void Log(string msg) { /* ... */ }
}
```

#### Q2: Apply OCP to a discount calculation system.
**Question:**
en: Apply the Open/Closed Principle (OCP) to a discount calculation system that needs to support new discount types easily.
vi: Áp dụng Nguyên tắc Đóng/Mở (OCP) cho một hệ thống tính chiết khấu cần hỗ trợ các loại chiết khấu mới một cách dễ dàng.

**Answer:**
en: Use an interface `DiscountStrategy` and implement specific discount classes instead of using `if/else` or `switch` statements in a central class.
vi: Sử dụng một giao diện `DiscountStrategy` và thực thi các lớp chiết khấu cụ thể thay vì sử dụng các câu lệnh `if/else` hoặc `switch` trong một lớp trung tâm.

```csharp
public interface IDiscount {
    double Apply(double price);
}

public class FlatDiscount : IDiscount {
    public double Apply(double price) => price - 10.0;
}
```

#### Q3: Show an LSP violation and its fix.
**Question:**
en: Provide a code example of a Liskov Substitution Principle (LSP) violation (e.g., Square-Rectangle) and how to fix it.
vi: Cung cấp một ví dụ mã nguồn về vi phạm Nguyên tắc thay thế Liskov (LSP) (ví dụ: Hình vuông - Hình chữ nhật) và cách khắc phục.

**Answer:**
en: A Square should not inherit from Rectangle if it cannot satisfy the requirement that Width and Height are independent. Better to have them both implement a `Shape` interface.
vi: Một Hình vuông không nên kế thừa từ Hình chữ nhật nếu nó không thể đáp ứng yêu cầu rằng Chiều rộng và Chiều cao độc lập nhau. Tốt hơn là để cả hai thực thi một giao diện `Shape`.

```csharp
// Violation: Square overrides Width to also set Height
// Fix: Use IShape interface with GetArea()
public interface IShape { double GetArea(); }
```

#### Q4: Refactor a "Fat Interface" into smaller ones (ISP).
**Question:**
en: Refactor an interface `Worker` that has `work()` and `eat()` for use by both Humans and Robots.
vi: Tái cấu trúc một giao diện `Worker` có `work()` và `eat()` để sử dụng cho cả Con người và Robot (áp dụng ISP).

**Answer:**
en: Split `Worker` into `Workable` and `Eatable`. Robots only implement `Workable`.
vi: Chia `Worker` thành `Workable` và `Eatable`. Robot chỉ thực thi `Workable`.

```csharp
public interface IWorkable { void Work(); }
public interface IEatable { void Eat(); }

public class Robot : IWorkable {
    public void Work() { /* working */ }
}
```

#### Q5: Apply DIP to decouple a high-level `App` from a `Database`.
**Question:**
en: Apply the Dependency Inversion Principle (DIP) to decouple a high-level `Application` from a concrete `PostgreSQLDatabase`.
vi: Áp dụng Nguyên tắc đảo ngược phụ thuộc (DIP) để tách biệt lớp `Application` cao cấp khỏi lớp `PostgreSQLDatabase` cụ thể.

**Answer:**
en: The `Application` should depend on a `DatabaseRepository` interface, which `PostgreSQLDatabase` then implements.
vi: Lớp `Application` nên phụ thuộc vào một giao diện `DatabaseRepository`, sau đó `PostgreSQLDatabase` sẽ thực thi giao diện này.

```csharp
public interface IDatabaseRepository {
    void Save(object data);
}

public class Application {
    private readonly IDatabaseRepository _db;
    public Application(IDatabaseRepository db) {
        _db = db;
    }
}
```

#### Q6: Refactor a class that violates ISP (Multi-function Printer).
**Question:**
en: Refactor an interface `SmartDevice` that has `print()`, `scan()`, and `fax()` to follow the Interface Segregation Principle (ISP).
vi: Tái cấu trúc một giao diện `SmartDevice` có `print()`, `scan()`, và `fax()` để tuân theo Nguyên tắc phân tách giao diện (ISP).

**Answer:**
en: Break the interface into `Printer`, `Scanner`, and `Fax`.
vi: Chia giao diện thành `Printer`, `Scanner`, và `Fax`.

```csharp
public interface IPrinter { void Print(); }
public interface IScanner { void Scan(); }
public interface IFax { void Fax(); }

public class BasicPrinter : IPrinter {
    public void Print() { }
}
```

#### Q7: Demonstrate OCP using the "Strategy Pattern."
**Question:**
en: Demonstrate how to use the "Strategy Pattern" to apply the Open/Closed Principle (OCP) for a sorting algorithm.
vi: Minh họa cách sử dụng "Strategy Pattern" để áp dụng Nguyên tắc Đóng/Mở (OCP) cho một thuật toán sắp xếp.

**Answer:**
en: Define a `SortStrategy` interface, allowing the `Sorter` class to accept any new sorting algorithm without modification.
vi: Định nghĩa một giao diện `SortStrategy`, cho phép lớp `Sorter` chấp nhận bất kỳ thuật toán sắp xếp mới nào mà không cần sửa đổi.

```csharp
public interface ISortStrategy {
    T[] Sort<T>(T[] data);
}
public class BubbleSort : ISortStrategy {
    public T[] Sort<T>(T[] data) => data;
}
public class QuickSort : ISortStrategy {
    public T[] Sort<T>(T[] data) => data;
}
```

#### Q8: Correct a DIP violation for manual instantiation.
**Question:**
en: Provide a fix for a class `UserService` that manually instantiates a `UserRepository` inside its constructor, violating DIP.
vi: Cung cấp giải pháp khắc phục cho một lớp `UserService` tự khởi tạo một `UserRepository` bên trong hàm tạo của nó (vi phạm DIP).

**Answer:**
en: Use Dependency Injection to pass the repository interface into the constructor.
vi: Sử dụng Tiêm phụ thuộc (Dependency Injection) để truyền giao diện repository vào hàm tạo.

```csharp
// Before: _repo = new UserRepository();
// After fix:
public UserService(IUserRepository repo) {
    _repo = repo;
}
```

#### Q9: Use SRP to extract logic from a UI Controller.
**Question:**
en: In a web application, use SRP to refactor a controller method that handles validation, database saving, and email notifications.
vi: Trong một ứng dụng web, sử dụng SRP để tái cấu trúc một phương thức controller xử lý việc xác thực, lưu cơ sở dữ liệu và thông báo qua email.

**Answer:**
en: Move validation to a Validator class, database saving to a Service or Repository class, and email notifications to a Notifier class.
vi: Di chuyển việc xác thực sang một lớp Validator, lưu cơ sở dữ liệu sang một lớp Service hoặc Repository, và thông báo email sang một lớp Notifier.

#### Q10: Apply LSP to a hierarchy involving File systems (Read-only vs Read-Write).
**Question:**
en: Apply LSP to a hierarchy where you have `ReadOnlyFile` and `ReadWriteFile`. Why is `ReadWriteFile` inheriting from `ReadOnlyFile` often better than vice-versa?
vi: Áp dụng LSP cho một phân cấp nơi bạn có `ReadOnlyFile` và `ReadWriteFile`. Tại sao `ReadWriteFile` kế thừa từ `ReadOnlyFile` thường tốt hơn là ngược lại?

**Answer:**
en: If `ReadOnlyFile` inherits from `ReadWriteFile`, it would have to throw errors for the `write()` method, violating LSP. If `ReadWriteFile` inherits from `ReadOnlyFile`, it simply adds `write()` while satisfying all `ReadOnly` requirements.
vi: Nếu `ReadOnlyFile` kế thừa từ `ReadWriteFile`, nó sẽ phải ném lỗi cho phương thức `write()`, vi phạm LSP. Nếu `ReadWriteFile` kế thừa từ `ReadOnlyFile`, nó chỉ đơn giản là thêm `write()` trong khi vẫn đáp ứng tất cả các yêu cầu về `ReadOnly`.
