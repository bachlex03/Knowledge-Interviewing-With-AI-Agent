# Advanced OOP Questions & Answers

## 1. Composition vs. Inheritance: When to use which?
**Answer**:
en: 
- **Inheritance** represents an "is-a" relationship (e.g., a Dog is an Animal). It is useful for code reuse but can lead to tight coupling and deep hierarchies.
- **Composition** represents a "has-a" relationship (e.g., a Car has an Engine). It is generally preferred because it provides more flexibility, loose coupling, and allows changing behavior at runtime.

vi:
- **Kế thừa (Inheritance)** đại diện cho mối quan hệ "là một" (ví dụ: Chó là một loài Động vật). Nó hữu ích cho việc tái sử dụng mã nguồn nhưng có thể dẫn đến sự phụ thuộc chặt chẽ (tight coupling) và phân cấp quá sâu.
- **Thành phần (Composition)** đại diện cho mối quan hệ "có một" (ví dụ: Ô tô có một Động cơ). Nó thường được ưu tiên hơn vì cung cấp sự linh hoạt hơn, phụ thuộc lỏng lẻo (loose coupling) và cho phép thay đổi hành vi tại thời điểm thực thi.

## 2. What are SOLID principles?
**Answer**:
en:
- **S**ingle Responsibility: A class should have only one reason to change.
- **O**pen/Closed: Software entities should be open for extension but closed for modification.
- **L**iskov Substitution: Objects of a superclass should be replaceable with objects of its subclasses without affecting correctness.
- **I**nterface Segregation: Clients should not be forced to depend on interfaces they do not use.
- **D**ependency Inversion: Depend on abstractions, not concretions.

vi:
- **S**ingle Responsibility (Đơn nhiệm): Một lớp chỉ nên có một lý do duy nhất để thay đổi.
- **O**pen/Closed (Đóng/Mở): Các thực thể phần mềm nên mở để mở rộng nhưng đóng để sửa đổi.
- **L**iskov Substitution (Thay thế Liskov): Các đối tượng của lớp cha có thể được thay thế bằng các đối tượng của lớp con mà không làm ảnh hưởng đến tính đúng đắn của chương trình.
- **I**nterface Segregation (Phân tách giao diện): Các client không nên bị buộc phải phụ thuộc vào các giao diện mà họ không sử dụng.
- **D**ependency Inversion (Đảo ngược phụ thuộc): Hãy phụ thuộc vào các trừu tượng (abstractions), đừng phụ thuộc vào các triển khai cụ thể (concretions).

## 3. Difference between Abstract Class and Interface?
**Answer**:
en:
- **Abstract Class**: Can have both abstract (without body) and concrete methods. Can have state (fields). A class can inherit only one abstract class. Represents "identity".
- **Interface**: Traditionally only had abstract methods (though modern languages allow default methods). Cannot have state. A class can implement multiple interfaces. Represents "capability".

vi:
- **Lớp trừu tượng (Abstract Class)**: Có thể có cả phương thức trừu tượng (không có thân) và phương thức cụ thể. Có thể có trạng thái (các trường dữ liệu). Một lớp chỉ có thể kế thừa một lớp trừu tượng duy nhất. Đại diện cho "bản sắc" (identity).
- **Giao diện (Interface)**: Theo truyền thống chỉ có các phương thức trừu tượng (mặc dù các ngôn ngữ hiện đại cho phép phương thức mặc định). Không thể có trạng thái. Một lớp có thể triển khai nhiều giao diện. Đại diện cho "khả năng" (capability).

## 4. What is the "Diamond Problem" and how is it solved?
**Answer**:
en: It occurs in multiple inheritance where a class inherits from two classes that both inherit from a common base class. This leads to ambiguity. Many languages (like Java) solve this by disallowing multiple inheritance of classes and using Interfaces instead. Languages like Python use Method Resolution Order (MRO).

vi: Vấn đề này xảy ra trong đa kế thừa khi một lớp kế thừa từ hai lớp mà cả hai lớp đó đều kế thừa từ một lớp cơ sở chung. Điều này dẫn đến sự nhập nhằng. Nhiều ngôn ngữ (như Java) giải quyết vấn đề này bằng cách không cho phép đa kế thừa lớp và thay vào đó sử dụng Interface. Các ngôn ngữ như Python sử dụng Thứ tự giải quyết phương thức (MRO).
