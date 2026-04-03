# OOP Foundation Q&A

### Level 1: Remembering
#### Q1: Define OOP.
**Question:**
en: What is the definition of Object-Oriented Programming (OOP)?
vi: Định nghĩa về Lập trình hướng đối tượng (OOP) là gì?

**Answer:**
en: OOP is a programming paradigm based on the concept of "objects," which can contain data (fields) and code (methods).
vi: OOP là một mô hình lập trình dựa trên khái niệm "đối tượng", vốn có thể chứa dữ liệu (trường) và mã nguồn (phương thức).

#### Q2: Name the four pillars of OOP.
**Question:**
en: Name the four fundamental pillars of Object-Oriented Programming.
vi: Hãy nêu tên bốn cột trụ cơ bản của Lập trình hướng đối tượng.

**Answer:**
en: 1. Encapsulation, 2. Inheritance, 3. Polymorphism, 4. Abstraction.
vi: 1. Đóng gói (Encapsulation), 2. Kế thừa (Inheritance), 3. Đa hình (Polymorphism), 4. Trừu tượng (Abstraction).

#### Q3: What is a Class?
**Question:**
en: What is a class in OOP?
vi: Lớp (Class) trong OOP là gì?

**Answer:**
en: A class is a blueprint or template for creating objects.
vi: Lớp là một bản thiết kế hoặc khuôn mẫu để tạo ra các đối tượng.

#### Q4: What is an Object?
**Question:**
en: What is an object in OOP?
vi: Đối tượng (Object) trong OOP là gì?

**Answer:**
en: An object is an instance of a class.
vi: Đối tượng là một thực thể (instance) của một lớp.

#### Q5: Define Encapsulation.
**Question:**
en: What is the primary definition of Encapsulation?
vi: Định nghĩa chính của Tính đóng gói (Encapsulation) là gì?

**Answer:**
en: Encapsulation is the bundling of data and the methods that operate on that data into a single unit (class) and restricting access to some details.
vi: Đóng gói là việc nhóm dữ liệu và các phương thức hoạt động trên dữ liệu đó vào một đơn vị duy nhất (lớp) và hạn chế quyền truy cập vào một số chi tiết.

#### Q6: What is Inheritance?
**Question:**
en: Briefly define Inheritance in OOP.
vi: Định nghĩa ngắn gọn về Tính kế thừa (Inheritance) trong OOP.

**Answer:**
en: Inheritance is a mechanism where one class acquires the properties and behaviors (fields and methods) of another class.
vi: Kế thừa là một cơ chế mà một lớp có được các thuộc tính và hành vi (trường và phương thức) của một lớp khác.

#### Q7: Define Polymorphism.
**Question:**
en: What does the term Polymorphism mean in programming?
vi: Thuật ngữ Đa hình (Polymorphism) có nghĩa là gì trong lập trình?

**Answer:**
en: It means "many shapes." It allows objects of different classes to be treated as objects of a common superclass.
vi: Nó có nghĩa là "nhiều hình dạng". Nó cho phép các đối tượng của các lớp khác nhau được đối xử như các đối tượng của một lớp cha chung.

#### Q8: What is Abstraction?
**Question:**
en: What is the main goal of Abstraction?
vi: Mục tiêu chính của Tính trừu tượng (Abstraction) là gì?

**Answer:**
en: To hide complex implementation details and show only the essential features of an object.
vi: Nhằm che giấu các chi tiết thực thi phức tạp và chỉ hiển thị các đặc tính thiết yếu của một đối tượng.

#### Q9: What is an access modifier?
**Question:**
en: What is the purpose of an access modifier?
vi: Mục đích của một công cụ sửa đổi truy cập (access modifier) là gì?

**Answer:**
en: Access modifiers define the visibility and accessibility of classes, methods, and variables.
vi: Các công cụ sửa đổi truy cập xác định khả năng hiển thị và khả năng truy cập của các lớp, phương thức và biến.

#### Q10: Identify common access modifiers in C#.
**Question:**
en: Identify four common access modifiers used in C#.
vi: Xác định bốn công cụ sửa đổi truy cập phổ biến được sử dụng trong C#.

**Answer:**
en: 1. public, 2. private, 3. protected, 4. internal.
vi: 1. public, 2. private, 3. protected, 4. internal.

---

### Level 2: Understanding
#### Q1: Contrast a Class and an Object.
**Question:**
en: What is the difference between a class and an object?
vi: Sự khác biệt giữa một lớp (class) và một đối tượng (object) là gì?

**Answer:**
en: A class is the blueprint (static), while an object is the living instance created from that blueprint (dynamic).
vi: Lớp là bản thiết kế (tĩnh), trong khi đối tượng là một thực thể sống được tạo ra từ bản thiết kế đó (động).

#### Q2: Contrast an Abstract Class vs an Interface.
**Question:**
en: Contrast an Abstract Class and an Interface.
vi: Phân biệt giữa Lớp trừu tượng (Abstract Class) và Giao diện (Interface).

**Answer:**
en: An abstract class can have code (methods with bodies), while an interface traditionally only defines method signatures (contracts). A class can implement multiple interfaces but inherit from only one class.
vi: Một lớp trừu tượng có thể có mã nguồn (các phương thức có thân hàm), trong khi một giao diện truyền thống chỉ định nghĩa các chữ ký phương thức (hợp đồng). Một lớp có thể thực thi nhiều giao diện nhưng chỉ có thể kế thừa từ một lớp duy nhất.

#### Q3: Explain the benefit of Encapsulation.
**Question:**
en: Why is Encapsulation beneficial for long-term project maintenance?
vi: Tại sao Tính đóng gói lại có lợi cho việc bảo trì dự án dài hạn?

**Answer:**
en: It prevents external code from depending on internal data structures, making it safer to change the implementation without breaking the rest of the application.
vi: Nó ngăn chặn mã nguồn bên ngoài phụ thuộc vào cấu trúc dữ liệu nội bộ, giúp việc thay đổi thực thi trở nên an toàn hơn mà không làm hỏng phần còn lại của ứng dụng.

#### Q4: Describe Method Overriding vs Method Overloading.
**Question:**
en: Describe the difference between Method Overriding and Method Overloading.
vi: Mô tả sự khác biệt giữa Ghi đè phương thức (Method Overriding) và Nạp chồng phương thức (Method Overloading).

**Answer:**
en: **Overloading** is having multiple methods with the same name but different parameters (compile-time). **Overriding** is redefining a base class method in a subclass (runtime).
vi: **Nạp chồng (Overloading)** là có nhiều phương thức cùng tên nhưng khác tham số (thời điểm biên dịch). **Ghi đè (Overriding)** là định nghĩa lại một phương thức của lớp cha trong lớp con (thời điểm chạy).

#### Q5: Explain the concept of "Composition over Inheritance."
**Question:**
en: Explain the general concept of "Composition over Inheritance."
vi: Giải thích khái niệm chung "Ưu tiên thành phần hơn kế thừa" (Composition over Inheritance).

**Answer:**
en: It suggests that complex functionality should be achieved by combining objects (has-a) rather than by creating deep hierarchies of subclasses (is-a).
vi: Nó gợi ý rằng các chức năng phức tạp nên được đạt được bằng cách kết hợp các đối tượng (mối quan hệ has-a) thay vì tạo ra các phân cấp lớp con sâu (mối quan hệ is-a).

#### Q6: Discuss the role of a Constructor.
**Question:**
en: What is the role of a Constructor in a class?
vi: Vai trò của một Hàm tạo (Constructor) trong một lớp là gì?

**Answer:**
en: A constructor is a special method called when an object is instantiated, primarily used to initialize the object's initial state and fields.
vi: Hàm tạo là một phương thức đặc biệt được gọi khi một đối tượng được khởi tạo, chủ yếu được sử dụng để khởi tạo trạng thái ban đầu và các trường của đối tượng.

#### Q7: Explain "Pass by reference" vs "Pass by value" in OOP.
**Question:**
en: How does passing a primitive value differ from passing an object reference in C#?
vi: Việc truyền một giá trị nguyên thủy khác với việc truyền một tham chiếu đối tượng như thế nào trong C#?

**Answer:**
en: Passing by value means copying the actual data. Passing an object reference means copying the memory address, allowing the original object to be modified inside the function.
vi: Truyền theo giá trị có nghĩa là sao chép dữ liệu thực tế. Truyền tham chiếu đối tượng có nghĩa là sao chép địa chỉ bộ nhớ, cho phép đối tượng ban đầu bị sửa đổi bên trong hàm.

#### Q8: Describe the "Has-a" vs "Is-a" relationship.
**Question:**
en: Describe the difference between the "Has-a" and "Is-a" relationships in OOP design.
vi: Mô tả sự khác biệt giữa mối quan hệ "Has-a" (có một) và "Is-a" (là một) trong thiết kế OOP.

**Answer:**
en: "Is-a" refers to Inheritance (Dog is a Pet). "Has-a" refers to Composition/Association (Pet has a Name).
vi: "Is-a" ám chỉ Tính kế thừa (Con chó là một Vật nuôi). "Has-a" ám chỉ Tính thành phần/Liên kết (Vật nuôi có một Cái tên).

#### Q9: Explain static members vs instance members.
**Question:**
en: What is the main difference between a static member and an instance member?
vi: Sự khác biệt chính giữa một thành viên tĩnh (static member) và một thành viên thực thể (instance member) là gì?

**Answer:**
en: Instance members belong to each object. Static members belong to the class itself and are shared by all instances.
vi: Các thành viên thực thể thuộc về từng đối tượng. Các thành viên tĩnh thuộc về chính bản thân lớp và được chia sẻ bởi tất cả các thực thể.

#### Q10: Discuss why abstraction is useful for reducing complexity.
**Question:**
en: In what way does Abstraction reduce cognitive complexity for a developer?
vi: Tính trừu tượng làm giảm độ phức tạp về nhận thức cho một lập trình viên theo cách nào?

**Answer:**
en: It allows the developer to focus on the "what" (interface) rather than the "how" (implementation), effectively hiding unnecessary details of external systems.
vi: Nó cho phép lập trình viên tập trung vào việc "là cái gì" (giao diện) thay vì "làm như thế nào" (thực thi), giúp che giấu hiệu quả các chi tiết không cần thiết của các hệ thống bên ngoài.

---

### Level 3: Applying
#### Q1: Create a class `Person` with encapsulated properties.
**Question:**
en: Implement a `Person` class in C# that uses private fields and public properties (encapsulation).
vi: Triển khai một lớp `Person` trong C# sử dụng các trường riêng tư và các thuộc tính công khai (tính đóng gói).

**Answer:**
```csharp
public class Person {
    private string _name;
    public string Name {
        get { return _name; }
        set { _name = value; }
    }
}
```

#### Q2: Implement Inheritance for an `Employee` class.
**Question:**
en: Demonstrate Inheritance by creating a subclass `Employee` that inherits from the `Person` class.
vi: Minh họa Tính kế thừa bằng cách tạo một lớp con `Employee` kế thừa từ lớp `Person`.

**Answer:**
```csharp
public class Employee : Person {
    public int EmployeeId { get; set; }
}
```

#### Q3: Demonstrate Runtime Polymorphism.
**Question:**
en: Demonstrate Runtime Polymorphism using a base class and an overridden method.
vi: Minh họa Đa hình lúc chạy (Runtime Polymorphism) bằng cách sử dụng một lớp cha và một phương thức được ghi đè.

**Answer:**
```csharp
public class Animal {
    public virtual void Speak() { Console.WriteLine("Animal sound"); }
}
public class Dog : Animal {
    public override void Speak() { Console.WriteLine("Woof!"); }
}
// Animal animal = new Dog(); animal.Speak(); // Outputs "Woof!"
```

#### Q4: Implement an Interface `IRepository`.
**Question:**
en: Define and implement a simple `IRepository` interface for a `Product` class.
vi: Định nghĩa và triển khai một giao diện `IRepository` đơn giản cho một lớp `Product`.

**Answer:**
```csharp
public interface IRepository {
    void Save(object item);
}
public class ProductRepository : IRepository {
    public void Save(object item) { /* save to DB */ }
}
```

#### Q5: Implement an Abstract Class `Shape`.
**Question:**
en: Create an abstract class `Shape` with an abstract method `GetArea()`.
vi: Tạo một lớp trừu tượng `Shape` với một phương thức trừu tượng `GetArea()`.

**Answer:**
```csharp
public abstract class Shape {
    public abstract double GetArea();
}
public class Circle : Shape {
    public double Radius { get; set; }
    public override double GetArea() => Math.PI * Radius * Radius;
}
```

#### Q6: Demonstrate Method Overloading.
**Question:**
en: Provide a code snippet that demonstrates method overloading within a class.
vi: Cung cấp một đoạn mã minh họa việc nạp chồng phương thức (method overloading) bên trong một lớp.

**Answer:**
```csharp
public class Calculator {
    public int Add(int a, int b) => a + b;
    public double Add(double a, double b) => a + b;
}
```

#### Q7: Demonstrate Method Overriding.
**Question:**
en: Implement method overriding where a `Manager` class changes the behavior of a `Work()` method from `Employee`.
vi: Triển khai ghi đè phương thức nơi một lớp `Manager` thay đổi hành vi của phương thức `Work()` từ lớp `Employee`.

**Answer:**
```csharp
public class Employee {
    public virtual void Work() { Console.WriteLine("Working..."); }
}
public class Manager : Employee {
    public override void Work() { Console.WriteLine("Managing team..."); }
}
```

#### Q8: Implement a Constructor with Parameters.
**Question:**
en: Write a constructor for a `Car` class that initializes the `Model` and `Year`.
vi: Viết một hàm tạo cho lớp `Car` để khởi tạo `Model` và `Year`.

**Answer:**
```csharp
public class Car {
    public string Model { get; }
    public int Year { get; }
    public Car(string model, int year) {
        Model = model;
        Year = year;
    }
}
```

#### Q9: Implement a Static Utility Class.
**Question:**
en: Create a static class `MathUtils` with a static method `Square(int x)`.
vi: Tạo một lớp tĩnh `MathUtils` với một phương thức tĩnh `Square(int x)`.

**Answer:**
```csharp
public static class MathUtils {
    public static int Square(int x) => x * x;
}
```

#### Q10: Use a List of polymorphic objects.
**Question:**
en: Create a list containing both `Dog` and `Cat` (subclasses of `Animal`) and call their shared `Speak()` method using a loop.
vi: Tạo một danh sách chứa cả `Dog` và `Cat` (các lớp con của `Animal`) và gọi phương thức `Speak()` chung của chúng bằng một vòng lặp.

**Answer:**
```csharp
List<Animal> animals = new List<Animal> { new Dog(), new Cat() };
foreach (var animal in animals) {
    animal.Speak();
}
```
