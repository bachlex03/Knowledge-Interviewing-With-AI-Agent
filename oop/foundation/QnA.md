# Foundational OOP Questions & Answers

## 1. What are the four main pillars of OOP?
**Answer**:
en: The four main pillars are:
- **Encapsulation**: Bundling data and methods into a single unit (class) and restricting direct access to some components.
- **Inheritance**: Allowing a class (subclass) to acquire properties and behaviors from another class (superclass).
- **Polymorphism**: The ability of different classes to be treated as instances of the same class through a common interface (overloading and overriding).
- **Abstraction**: Hiding complex implementation details and showing only the necessary features of an object.

vi: Bốn cột trụ chính là:
- **Đóng gói (Encapsulation)**: Gói gọn dữ liệu và phương thức vào một đơn vị duy nhất (lớp) và hạn chế truy cập trực tiếp vào một số thành phần.
- **Kế thừa (Inheritance)**: Cho phép một lớp (lớp con) thừa hưởng các thuộc tính và hành vi từ một lớp khác (lớp cha).
- **Đa hình (Polymorphism)**: Khả năng các lớp khác nhau có thể được xử lý như các đối tượng của cùng một lớp thông qua một giao diện chung (nạp chồng và ghi đè).
- **Trừu tượng (Abstraction)**: Ẩn đi các chi tiết triển khai phức tạp và chỉ hiển thị các đặc tính cần thiết của một đối tượng.

## 2. Why is Encapsulation important?
**Answer**:
en: It helps in data hiding, reducing complexity, and increasing reusability. It ensures that the internal state of an object is protected from unauthorized access and modification.
vi: Nó giúp ẩn dữ liệu, giảm độ phức tạp và tăng khả năng tái sử dụng. Nó đảm bảo rằng trạng thái bên trong của một đối tượng được bảo vệ khỏi việc truy cập và sửa đổi trái phép.

## 3. What is the difference between a Class and an Object?
**Answer**:
en: A **Class** is a blueprint or template from which objects are created. An **Object** is an instance of a class, representing a real-world entity with state and behavior.
vi: **Lớp (Class)** là một bản thiết kế hoặc khuôn mẫu để tạo ra các đối tượng. **Đối tượng (Object)** là một thực thể cụ thể (instance) của một lớp, đại diện cho một thực thể trong thế giới thực với trạng thái và hành vi.

## 4. What is a Constructor?
**Answer**:
en: A special method used to initialize objects. It is called automatically when an instance of a class is created.
vi: Một phương thức đặc biệt được sử dụng để khởi tạo đối tượng. Nó được gọi tự động khi một thực thể của lớp được tạo ra.

## 5. What are Access Modifiers?
**Answer**:
en: They define the visibility and accessibility of classes, methods, and variables. Common ones include **public**, **private**, and **protected**.
vi: Chúng xác định mức độ hiển thị và khả năng truy cập của các lớp, phương thức và biến. Các loại phổ biến bao gồm **public** (công khai), **private** (riêng tư) và **protected** (được bảo vệ).

## 6. What is Method Overloading vs. Overriding?
**Answer**:
en: 
- **Overloading**: Multiple methods with the same name but different parameters within the same class (compile-time polymorphism).
- **Overriding**: A subclass provides a specific implementation for a method already defined in its parent class (runtime polymorphism).
vi:
- **Overloading (Nạp chồng)**: Nhiều phương thức có cùng tên nhưng khác tham số trong cùng một lớp (đa hình tại thời điểm biên dịch).
- **Overriding (Ghi đè)**: Một lớp con cung cấp triển khai cụ thể cho một phương thức đã được định nghĩa trong lớp cha của nó (đa hình tại thời điểm thực thi).
