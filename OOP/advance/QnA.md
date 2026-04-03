# OOP Advance Q&A

### Level 4: Analyzing
#### Q1: Compare Abstract Classes vs Interfaces for large-scale systems.
**Question:**
en: Compare and contrast the usage of Abstract Classes and Interfaces when designing a large-scale framework.
vi: So sánh và đối chiếu việc sử dụng Lớp trừu tượng và Giao diện khi thiết kế một framework quy mô lớn.

**Answer:**
en: Abstract classes are better for sharing common implementation and internal state ("what it is"). Interfaces are better for defining external capabilities and contracts ("what it can do"). Interfaces allow multiple implementation, making them more flexible for plugin-style architectures.
vi: Lớp trừu tượng tốt hơn trong việc chia sẻ thực thi chung và trạng thái nội bộ ("nó là gì"). Giao diện tốt hơn trong việc định nghĩa các khả năng bên ngoài và các hợp đồng ("nó có thể làm gì"). Giao diện cho phép đa thực thi, giúp chúng linh hoạt hơn cho các kiến trúc dạng plugin.

#### Q2: Analyze the impact of "Tight Coupling" in inheritance hierarchies.
**Question:**
en: Analyze why deep inheritance hierarchies can lead to the "Fragile Base Class" problem.
vi: Phân tích tại sao các phân cấp kế thừa sâu có thể dẫn đến vấn đề "Lớp cha dễ vỡ" (Fragile Base Class).

**Answer:**
en: Tight coupling means subclasses depend heavily on the base class's internal logic. A small change in the base class can unintentionally break behavior in dozens of inherited subclasses, making refactoring extremely risky.
vi: Liên kết chặt chẽ có nghĩa là các lớp con phụ thuộc nặng nề vào logic nội bộ của lớp cha. Một thay đổi nhỏ trong lớp cha có thể vô tình làm hỏng hành vi trong hàng tá lớp con kế thừa, khiến cho việc tái cấu trúc trở nên cực kỳ rủi ro.

#### Q3: Contrast Private vs Protected visibility for internal API design.
**Question:**
en: Examine the trade-offs between using `private` and `protected` accessibility from an API security perspective.
vi: Xem xét sự đánh đổi giữa việc sử dụng khả năng truy cập `private` và `protected` từ góc nhìn bảo mật API.

**Answer:**
en: `private` offers maximum encapsulation and prevents any external tampering. `protected` allows extensibility through subclasses but creates a wider surface area of dependency that you must support indefinitely.
vi: `private` mang lại khả năng đóng gói tối đa và ngăn chặn bất kỳ sự can thiệp nào từ bên ngoài. `protected` cho phép tính mở rộng thông qua các lớp con nhưng tạo ra một diện tích bề mặt phụ thuộc rộng hơn mà bạn phải hỗ trợ vô thời hạn.

#### Q4: Analyze Static Polymorphism vs Dynamic Polymorphism.
**Question:**
en: Analyze the performance and flexibility differences between Static Polymorphism (Overloading, Generics) and Dynamic Polymorphism (Overriding).
vi: Phân tích sự khác biệt về hiệu suất và tính linh hoạt giữa Đa hình tĩnh (Nạp chồng, Generics) và Đa hình động (Ghi đè).

**Answer:**
en: Static polymorphism is resolved at compile-time, offering better performance (no vtable lookup). Dynamic polymorphism is resolved at runtime via the vtable, offering maximum flexibility for swapping behaviors at execution time.
vi: Đa hình tĩnh được giải quyết tại thời điểm biên dịch, mang lại hiệu suất tốt hơn (không cần tra cứu bảng vtable). Đa hình động được giải quyết tại thời điểm chạy thông qua bảng vtable, mang lại tính linh hoạt tối đa cho việc thay đổi hành vi tại thời điểm thực thi.

#### Q5: Examine the "Diamond Problem" and how C# addresses it.
**Question:**
en: What is the "Diamond Problem" in multiple inheritance, and how does C# avoid it?
vi: Vấn đề "Kim cương" (Diamond Problem) trong đa kế thừa là gì, và C# tránh nó như thế nào?

**Answer:**
en: It occurs when a subclass inherits from two classes that share a common ancestor, leading to ambiguity in method calls. C# avoids this by only allowing single inheritance for classes but allowing multiple interface implementation.
vi: Nó xảy ra khi một lớp con kế thừa từ hai lớp có chung một tổ tiên, dẫn đến sự mơ hồ trong việc gọi phương thức. C# tránh điều này bằng cách chỉ cho phép đơn kế thừa đối với các lớp nhưng cho phép thực thi nhiều giao diện.

#### Q6: Compare Shallow Copy vs Deep Copy logic.
**Question:**
en: Analyze the logic behind Shallow Copy vs Deep Copy when cloning complex objects.
vi: Phân tích logic đằng sau Sao chép nông (Shallow Copy) và Sao chép sâu (Deep Copy) khi nhân bản các đối tượng phức tạp.

**Answer:**
en: Shallow copy clones only the top-level references (sharing internal objects). Deep copy recursively clones all child objects, creating a completely independent copy in memory.
vi: Sao chép nông chỉ nhân bản các tham chiếu cấp cao nhất (chia sẻ các đối tượng nội bộ). Sao chép sâu nhân bản đệ quy tất cả các đối tượng con, tạo ra một bản sao hoàn toàn độc lập trong bộ nhớ.

#### Q7: Analyze how Encapsulation facilitates Unit Testing.
**Question:**
en: How does proper Encapsulation improve the isolatability of units for software testing?
vi: Việc Đóng gói đúng cách cải thiện khả năng cô lập các đơn vị (units) cho việc kiểm thử phần mềm như thế nào?

**Answer:**
en: By exposing only a specific interface and hiding the state, you can test a class's behavior through its public API without needing to mock internal fields or worry about side effects from hidden global state.
vi: Bằng cách chỉ hiển thị một giao diện cụ thể và che giấu trạng thái, bạn có thể kiểm thử hành vi của một lớp thông qua API công khai của nó mà không cần phải giả lập các trường nội bộ hoặc lo lắng về các tác dụng phụ từ trạng thái toàn cục ẩn.

#### Q8: Compare Virtual methods vs Abstract methods.
**Question:**
en: Contrast the intent and rules of `virtual` methods versus `abstract` methods in C#.
vi: Đối chiếu ý định và quy tắc của các phương thức `virtual` so với các phương thức `abstract` trong C#.

**Answer:**
en: A `virtual` method has a default implementation that can optionally be overridden. An `abstract` method has no implementation and MUST be overridden by non-abstract subclasses.
vi: Một phương thức `virtual` có một thực thi mặc định có thể (tùy chọn) bị ghi đè. Một phương thức `abstract` không có thực thi và BẮT BUỘC phải được ghi đè bởi các lớp con không trừu tượng.

#### Q9: Examine the impact of `sealed` classes on performance.
**Question:**
en: Analyze the architectural and performance impact of marking a class as `sealed`.
vi: Phân tích tác động về mặt kiến trúc và hiệu suất của việc đánh dấu một lớp là `sealed`.

**Answer:**
en: Architecturally, it prevents inheritance (finality). Performance-wise, it allows the JIT compiler to "devirtualize" calls, turning virtual lookups into faster direct calls since it knows no subclass can override the methods.
vi: Về mặt kiến trúc, nó ngăn chặn việc kế thừa (tính cuối cùng). Về mặt hiệu suất, nó cho phép trình biên dịch JIT "giải đa hình" (devirtualize) các lệnh gọi, chuyển các lần tra cứu ảo thành các lệnh gọi trực tiếp nhanh hơn vì nó biết không có lớp con nào có thể ghi đè phương thức.

#### Q10: Analyze the risks of "Hidden State" in static classes.
**Question:**
en: What are the risks of using static classes to store mutable global state in a multi-threaded OOP application?
vi: Rủi ro của việc sử dụng các lớp tĩnh để lưu trữ trạng thái toàn cục có thể thay đổi trong một ứng dụng OOP đa luồng là gì?

**Answer:**
en: It creates thread-safety issues (race conditions), makes unit testing difficult due to side effects between tests, and violates encapsulation by allowing any code to modify the state globally.
vi: Nó tạo ra các vấn đề về an toàn luồng (race conditions), khiến việc kiểm thử đơn vị trở nên khó khăn do các tác dụng phụ giữa các lần kiểm thử, và vi phạm tính đóng gói bằng cách cho phép bất kỳ mã nguồn nào cũng có thể sửa đổi trạng thái trên toàn cục.

---

### Level 5: Evaluating
#### Q1: Critically evaluate OOP vs Procedural programming for UI development.
**Question:**
en: Evaluate whether OOP is inherently more suitable for Graphical User Interface (GUI) development than Procedural programming.
vi: Đánh giá xem OOP có vốn dĩ phù hợp hơn cho việc phát triển Giao diện người dùng đồ họa (GUI) so với lập trình Thủ tục hay không.

**Answer:**
en: OOP is generally superior for GUIs because UI elements (buttons, windows) naturally map to objects with state (colors, text) and behavior (click events), which is easier to manage than managing a global procedural loop.
vi: OOP thường vượt trội hơn đối với GUI vì các thành phần giao diện (nút, cửa sổ) ánh xạ tự nhiên với các đối tượng có trạng thái (màu sắc, văn bản) và hành vi (sự kiện nhấp chuột), điều này dễ quản lý hơn việc quản lý một vòng lặp thủ tục toàn cục.

#### Q2: Judge when an Interface is preferable over an Abstract Class.
**Question:**
en: Under what circumstances would you judge an Interface to be a better choice than an Abstract Class?
vi: Trong những trường hợp nào bạn sẽ đánh giá Giao diện là một lựa chọn tốt hơn so với Lớp trừu tượng?

**Answer:**
en: Use an interface when you want to define a contract that can be implemented by many unrelated classes (e.g., `IDisposable`, `IComparable`) or when you need a class to support multiple behaviors.
vi: Sử dụng giao diện khi bạn muốn định nghĩa một hợp đồng có thể được thực thi bởi nhiều lớp không liên quan (ví dụ: `IDisposable`, `IComparable`) hoặc khi bạn cần một lớp hỗ trợ nhiều hành vi.

#### Q3: Critique the "Everything is an Object" philosophy.
**Question:**
en: Critique the potential downsides of the strict "Everything is an Object" philosophy found in languages like early Java or C#.
vi: Phê bình những nhược điểm tiềm tàng của triết lý nghiêm ngặt "Mọi thứ đều là đối tượng" được tìm thấy trong các ngôn ngữ như Java thời kỳ đầu hoặc C#.

**Answer:**
en: It can lead to "Boilerplate Overload" where simple mathematical functions require a class wrapper, increasing verbosity and memory overhead compared to a more multi-paradigm approach.
vi: Nó có thể dẫn đến "Quá tải mã mẫu" nơi các hàm toán học đơn giản yêu cầu một lớp bao bọc, làm tăng độ dài dòng và chi phí bộ nhớ so với cách tiếp cận đa mô hình.

#### Q4: Evaluate a design that uses public fields instead of properties.
**Question:**
en: Evaluate the long-term design implications of using `public int Age;` instead of `public int Age { get; set; }`.
vi: Đánh giá các hệ lụy thiết kế dài hạn của việc sử dụng `public int Age;` thay vì `public int Age { get; set; }`.

**Answer:**
en: Using public fields breaks encapsulation. You cannot later add validation or logic to the field without breaking binary compatibility for any external code that uses it.
vi: Sử dụng các trường công khai sẽ phá vỡ tính đóng gói. Bạn không thể sau này thêm việc xác thực hoặc logic vào trường đó mà không làm hỏng tính tương thích nhị phân đối với bất kỳ mã nguồn bên ngoài nào sử dụng nó.

#### Q5: Judge the trade-off of large objects vs small focused objects.
**Question:**
en: Judge the architectural trade-off between having one large "User" object versus splitting it into "UserCredentials," "UserProfile," and "UserSettings."
vi: Đánh giá sự đánh đổi về mặt kiến trúc giữa việc có một đối tượng "User" lớn so với việc chia nó thành "UserCredentials", "UserProfile" và "UserSettings".

**Answer:**
en: Large objects are easier for initial development but become "God Objects" that are hard to maintain. Small objects follow SRP, are easier to test, and reduce memory pressure if only part of the data is needed.
vi: Các đối tượng lớn dễ dàng cho việc phát triển ban đầu nhưng trở thành các "God Objects" khó bảo trì. Các đối tượng nhỏ tuân theo SRP, dễ kiểm thử hơn và giảm áp lực bộ nhớ nếu chỉ cần một phần dữ liệu.

#### Q6: Critically evaluate the performance cost of Polymorphism in high-frequency trading systems.
**Question:**
en: In systems where every microsecond matters (like HFT), evaluate whether the use of virtual/polymorphic calls is a justifiable risk.
vi: Trong các hệ thống mà mỗi micro giây đều quan trọng (như HFT), hãy đánh giá xem việc sử dụng các lệnh gọi ảo/đa hình có phải là một rủi ro chính đáng hay không.

**Answer:**
en: Often it is not. The indirect jump in a vtable can cause CPU branch mispredictions and prevents inlining. Seniors often switch to "Devirtualization" or "Data-Oriented Design" for these critical paths.
vi: Thường là không. Việc nhảy gián tiếp trong vtable có thể gây ra việc dự đoán sai nhánh CPU và ngăn chặn việc inline. Các chuyên gia thường chuyển sang "Giải đa hình" (Devirtualization) hoặc "Thiết kế hướng dữ liệu" cho các luồng quan trọng này.

#### Q7: Appraise the value of Design Patterns in modern OOP.
**Question:**
en: Appraise whether classical Design Patterns (Gang of Four) are still relevant in modern OOP with the rise of Functional features.
vi: Đánh giá xem các Mẫu thiết kế cổ điển (Gang of Four) có còn phù hợp trong OOP hiện đại với sự trỗi dậy của các tính năng Hàm (Functional) hay không.

**Answer:**
en: Many patterns like "Strategy" or "Observer" are now simpler using Lambdas or Events, but behavioral patterns like "Command" or structural ones like "Bridge" remain vital for managing complexity in large systems.
vi: Nhiều mẫu thiết kế như "Strategy" hoặc "Observer" hiện nay đơn giản hơn khi sử dụng Lambdas hoặc Events, nhưng các mẫu hành vi như "Command" hoặc các mẫu cấu trúc như "Bridge" vẫn cực kỳ quan trọng để quản lý sự phức tạp trong các hệ thống lớn.

#### Q8: Judge the effectiveness of Encapsulation in preventing security vulnerabilities.
**Question:**
en: Judge how effective Encapsulation is at preventing "Insecure Direct Object Reference" (IDOR) style vulnerabilities compared to external authorization.
vi: Đánh giá mức độ hiệu quả của Tính đóng gói trong việc ngăn chặn các lỗ hổng kiểu "Tham chiếu đối tượng trực tiếp không an toàn" (IDOR) so với việc ủy quyền bên ngoài.

**Answer:**
en: Encapsulation only protects internal state from *other code* in the same memory space, not from *unauthorized users* over a network. External authorization logic is always required for security.
vi: Đóng gói chỉ bảo vệ trạng thái nội bộ khỏi *mã nguồn khác* trong cùng không gian bộ nhớ, không phải khỏi *người dùng không được phép* qua mạng. Logic ủy quyền bên ngoài luôn là cần thiết cho bảo mật.

#### Q9: Evaluate the risk of "Multiple Inheritance" in languages that support it (like C++).
**Question:**
en: Evaluate why most modern languages (C#, Java, Rust) chose to avoid multiple implementation inheritance.
vi: Đánh giá tại sao hầu hết các ngôn ngữ hiện đại (C#, Java, Rust) chọn tránh đa kế thừa thực thi.

**Answer:**
en: The "Diamond Problem" and the sheer complexity of managing memory layout for multiple parent classes outweigh the benefits. Composition is almost always a cleaner alternative.
vi: Vấn đề "Kim cương" và sự phức tạp tuyệt đối của việc quản lý bố cục bộ nhớ cho nhiều lớp cha vượt qua các lợi ích. Thành phần (Composition) hầu như luôn là một giải pháp thay thế sạch sẽ hơn.

#### Q10: Appraise the future of OOP in the era of Functional-First languages.
**Question:**
en: Appraise the long-term survival of the OOP paradigm as languages like Rust and Go gain popularity.
vi: Đánh giá sự tồn tại lâu dài của mô hình OOP khi các ngôn ngữ như Rust và Go ngày càng phổ biến.

**Answer:**
en: OOP will likely survive as a dominant paradigm for UI and high-level business systems, but it is increasingly adopting "Functional" elements (immutability, records) to address the flaws of shared mutable state.
vi: OOP có khả năng sẽ tồn tại như một mô hình chiếm ưu thế cho UI và các hệ thống kinh doanh cấp cao, nhưng nó đang ngày càng áp dụng các yếu tố "Hàm" (tính bất biến, records) để giải quyết các khiếm khuyết của trạng thái có thể thay đổi và chia sẻ.
