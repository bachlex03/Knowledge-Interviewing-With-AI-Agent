# OOP Advance Q&A

### Level 4: Analyzing

#### Q_LEVEL4_401: Analyze Inheritance vs Composition.

**Question:**
en: Analyze the pros and cons of using Inheritance versus Composition.
vi: Phân tích ưu và nhược điểm của việc sử dụng Tính kế thừa (Inheritance) so với Tính bao gộp (Composition).

**Answer:**
en: Inheritance is good for "IS-A" relationships and polymorphism but tightly couples classes. Composition ("HAS-A") offers more flexibility, loose coupling, and easier testing at the cost of boilerplate code.
vi: Kế thừa tốt cho các mối quan hệ "IS-A" và đa hình, nhưng nó tạo ra sự liên kết chặt chẽ (tight coupling) giữa các lớp, khiến việc thay đổi khó khăn. Composition ("HAS-A") mang lại sự linh hoạt hơn, liên kết lỏng lẻo (loose coupling) và dễ dàng viết unit test hơn, nhưng bù lại phải viết nhiều mã boilerplate (mã lặp lại) hơn để ủy quyền các lệnh gọi.

#### Q_LEVEL4_402: Abstract Class vs Interface usage.

**Question:**
en: Analyze a scenario where an Abstract Class is a better choice than an Interface.
vi: Phân tích một tình huống mà Abstract Class là lựa chọn tốt hơn so với Interface.

**Answer:**
en: Use an Abstract Class when multiple related classes share common core logic and state (fields). Interfaces are better for defining roles across unrelated classes.
vi: Nên sử dụng Abstract Class khi có nhiều lớp liên quan chặt chẽ với nhau cần chia sẻ chung một phần logic cốt lõi và dữ liệu trạng thái (các fields/biến). Ví dụ, một `DatabaseConnection` base class có thể chứa logic kết nối cơ bản, trong khi Interface phù hợp hơn để định nghĩa các vai trò độc lập (như `IDisposable`) cho các lớp không liên quan đến nhau.

#### Q_LEVEL4_403: Liskov Substitution Principle.

**Question:**
en: How does the Liskov Substitution Principle (LSP) relate to OOP inheritance?
vi: Nguyên lý thay thế Liskov (LSP) liên quan thế nào đến tính kế thừa trong OOP?

**Answer:**
en: LSP states that objects of a superclass should be replaceable with objects of its subclasses without breaking the application. Violating LSP means the inheritance relationship is flawed.
vi: LSP phát biểu rằng các đối tượng của một lớp cha phải có thể được thay thế bằng các đối tượng của lớp con mà không làm hỏng tính đúng đắn của ứng dụng. Nếu việc sử dụng lớp con gây ra lỗi hoặc ngoại lệ không mong muốn, điều đó có nghĩa là mối quan hệ kế thừa đã được thiết kế sai và nên cân nhắc dùng Composition.

#### Q_LEVEL4_404: Virtual Method Dispatch.

**Question:**
en: Analyze the performance impact of dynamic polymorphism (virtual method dispatch) in C#.
vi: Phân tích tác động đến hiệu suất của đa hình động (điều phối phương thức ảo) trong C#.

**Answer:**
en: Virtual methods require a vtable lookup at runtime, introducing a slight overhead compared to direct method calls, and can inhibit compiler optimizations like inlining.
vi: Các phương thức ảo yêu cầu tra cứu bảng phương thức ảo (vtable lookup) tại thời điểm chạy (runtime) để tìm đúng phương thức của lớp con. Điều này gây ra một chút độ trễ (overhead) so với các lệnh gọi phương thức trực tiếp, đồng thời có thể ngăn cản các tối ưu hóa của trình biên dịch (như inlining). Dù vậy, trong hầu hết các ứng dụng thông thường, chi phí này là không đáng kể.

#### Q_LEVEL4_405: Multiple Inheritance in C#.

**Question:**
en: Analyze how multiple inheritance is handled in C# compared to C++.
vi: Phân tích cách xử lý đa kế thừa trong C# so với C++.

**Answer:**
en: C++ supports multiple class inheritance, which can lead to the "Diamond Problem". C# prevents this by allowing inheritance from only one class, but supports multiple interface implementation.
vi: C++ hỗ trợ đa kế thừa từ nhiều lớp, điều này có thể dẫn đến "Vấn đề hình thoi" (Diamond Problem) - sự mơ hồ khi kế thừa các thành viên trùng tên. C# ngăn chặn điều này bằng cách chỉ cho phép kế thừa từ duy nhất một lớp (single inheritance), nhưng giải quyết nhu cầu đa kế thừa bằng cách cho phép triển khai nhiều Interface cùng lúc.

---

### Level 5: Evaluating

#### Q_LEVEL5_501: Deep Inheritance Hierarchies.

**Question:**
en: Evaluate the trade-offs of using deeply nested inheritance hierarchies.
vi: Đánh giá sự đánh đổi của việc sử dụng cấu trúc phân cấp kế thừa quá sâu.

**Answer:**
en: While they can model complex taxonomies, deep hierarchies create brittle code. A change in a base class can unexpectedly break distant descendants, making maintenance difficult.
vi: Mặc dù chúng có thể mô hình hóa các phân loại phức tạp, nhưng cấu trúc phân cấp sâu tạo ra mã nguồn dễ gãy vỡ (brittle code). Một thay đổi nhỏ trong lớp cơ sở trên cùng có thể vô tình làm hỏng các lớp con ở xa, gây khó khăn cho việc kiểm thử và bảo trì. Khuyến nghị nên ưu tiên Composition thay vì kế thừa sâu.

#### Q_LEVEL5_502: Fat Interfaces vs ISP.

**Question:**
en: Critique a design that uses "fat" interfaces and propose a solution based on OOP principles.
vi: Chỉ trích một thiết kế sử dụng các interface "mập" (fat interfaces) và đề xuất một giải pháp dựa trên các nguyên lý OOP.

**Answer:**
en: A fat interface forces classes to implement methods they don't need. The solution is the Interface Segregation Principle (ISP): split the fat interface into smaller, role-specific interfaces.
vi: Một interface "mập" (chứa quá nhiều phương thức) buộc các lớp phải triển khai những phương thức mà chúng không hề sử dụng. Giải pháp là áp dụng Nguyên lý phân tách Interface (Interface Segregation Principle - ISP): chia nhỏ interface lớn đó thành các interface nhỏ hơn, cụ thể theo từng vai trò, để các lớp chỉ phụ thuộc vào những gì chúng thực sự cần.

#### Q_LEVEL5_503: Encapsulation in Legacy Systems.

**Question:**
en: Evaluate the effectiveness of Encapsulation in a legacy system heavily reliant on global state.
vi: Đánh giá hiệu quả của Tính đóng gói trong một hệ thống cũ (legacy system) phụ thuộc nhiều vào trạng thái toàn cục (global state).

**Answer:**
en: Encapsulation is severely compromised if global variables (e.g., Singletons with mutable state) are prevalent, as state can be altered from anywhere, defeating the purpose of hidden internal fields.
vi: Tính đóng gói bị suy giảm nghiêm trọng nếu các biến toàn cục (ví dụ: các Singleton có trạng thái có thể thay đổi) xuất hiện khắp nơi. Khi dữ liệu có thể bị thay đổi từ bất kỳ đâu bên ngoài, mục đích của việc ẩn các trường nội bộ bị phá vỡ, dẫn đến các lỗi khó theo dõi (side-effects). Cần dần dần refactor để truyền các dependency (DI) thay vì dùng global state.

#### Q_LEVEL5_504: The Singleton Pattern.

**Question:**
en: Justify the use of the Singleton pattern in an OOP design, considering its common criticisms.
vi: Biện minh cho việc sử dụng mẫu Singleton trong thiết kế OOP, xem xét các chỉ trích phổ biến đối với nó.

**Answer:**
en: While Singleton is often criticized as an anti-pattern (a glorified global variable), it is justified when exactly one instance is strictly required to control a shared resource, like a hardware driver or connection pool.
vi: Mặc dù Singleton thường bị chỉ trích là một anti-pattern (giống như một biến toàn cục được che đậy), nhưng nó vẫn hợp lý khi hệ thống bắt buộc chỉ được phép có duy nhất một instance để kiểm soát một tài nguyên dùng chung, chẳng hạn như trình điều khiển phần cứng (hardware driver) hoặc pool kết nối cơ sở dữ liệu. Để dễ test, nên kết hợp Singleton với Dependency Injection.

#### Q_LEVEL5_505: OOP vs Functional Programming.

**Question:**
en: Evaluate how OOP principles align or conflict with Functional Programming (FP) concepts.
vi: Đánh giá xem các nguyên lý OOP phù hợp hay xung đột như thế nào với các khái niệm Lập trình hàm (Functional Programming).

**Answer:**
en: OOP focuses on mutable state encapsulated in objects, while FP emphasizes pure functions and immutable state. Modern languages like C# blend both, using objects for architecture and FP (like LINQ) for data transformation.
vi: OOP tập trung vào trạng thái có thể thay đổi (mutable state) được đóng gói trong các đối tượng, trong khi FP nhấn mạnh vào các hàm thuần túy (pure functions) và trạng thái bất biến (immutable state) - tránh các side-effects. Mặc dù có vẻ xung đột, các ngôn ngữ hiện đại như C# kết hợp cả hai: sử dụng OOP cho kiến trúc hệ thống tổng thể và dùng FP (như LINQ) để xử lý dữ liệu và luồng logic.
