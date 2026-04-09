# C# Foundation Q&A

### Level 1: Remembering

#### Q_LEVEL1_832: What is C#?
**Question:**
en: What is C#?
vi: C# là gì?

**Answer:**
en: C# is a modern, object-oriented, type-safe programming language developed by Microsoft for the .NET platform.
vi: C# là một ngôn ngữ lập trình hiện đại, hướng đối tượng, an toàn kiểu được Microsoft phát triển cho nền tảng .NET.

#### Q_LEVEL1_591: What is the CLR?
**Question:**
en: What is the Common Language Runtime (CLR)?
vi: Common Language Runtime (CLR) là gì?

**Answer:**
en: CLR is the runtime execution engine for .NET, handling memory management, garbage collection, and exception handling.
vi: CLR là môi trường thực thi thời gian chạy của .NET, quản lý bộ nhớ, thu gom rác và xử lý ngoại lệ.

#### Q_LEVEL1_204: What is a value type?
**Question:**
en: What is a value type in C#?
vi: Kiểu giá trị (value type) trong C# là gì?

**Answer:**
en: A value type holds its actual data directly and is usually allocated on the stack (e.g., int, bool, struct).
vi: Kiểu giá trị lưu trữ dữ liệu trực tiếp và thường được cấp phát trên bộ nhớ stack (ví dụ: int, bool, struct).

#### Q_LEVEL1_715: What is a reference type?
**Question:**
en: What is a reference type in C#?
vi: Kiểu tham chiếu (reference type) trong C# là gì?

**Answer:**
en: A reference type holds a memory address pointing to its data and is allocated on the heap (e.g., class, string).
vi: Kiểu tham chiếu lưu trữ địa chỉ bộ nhớ trỏ tới dữ liệu và được cấp phát trên bộ nhớ heap (ví dụ: class, string).

#### Q_LEVEL1_349: What is the out parameter?
**Question:**
en: What is the `out` parameter?
vi: Tham số `out` là gì?

**Answer:**
en: The `out` keyword passes arguments by reference. It does not require initialization before being passed but must be assigned before the method returns.
vi: Từ khóa `out` truyền tham số theo tham chiếu. Nó không cần khởi tạo trước khi truyền nhưng bắt buộc phải được gán giá trị trước khi kết thúc hàm.

#### Q_LEVEL1_801: What is the ref parameter?
**Question:**
en: What is the `ref` parameter?
vi: Tham số `ref` là gì?

**Answer:**
en: The `ref` keyword allows passing arguments by reference. The variable must be initialized before it is passed to the method.
vi: Từ khóa `ref` cho phép truyền tham số theo tham chiếu. Biến bắt buộc phải được khởi tạo trước khi truyền vào phương thức.

#### Q_LEVEL1_492: What are access modifiers?
**Question:**
en: What are access modifiers in C#?
vi: Access modifiers trong C# là gì?

**Answer:**
en: Keywords (public, private, protected, internal) that define the visibility and accessibility of classes and members.
vi: Các từ khóa hạn chế truy cập (public, private, protected, internal) xác định khả năng hiển thị và quyền truy cập của class và các thành viên.

#### Q_LEVEL1_633: What is boxing?
**Question:**
en: What is boxing?
vi: Boxing là gì?

**Answer:**
en: Boxing is the process of converting a value type to an object type (or an interface type), which allocates memory on the heap.
vi: Boxing là quá trình chuyển đổi kiểu giá trị sang kiểu object (hoặc interface), quá trình này cấp phát bộ nhớ trên heap.

#### Q_LEVEL1_118: What is unboxing?
**Question:**
en: What is unboxing?
vi: Unboxing là gì?

**Answer:**
en: Unboxing is the explicit conversion of an object type back to a value type.
vi: Unboxing là quá trình chuyển đổi rõ ràng (explicit) từ một đối tượng (object) trở về kiểu giá trị ban đầu.

#### Q_LEVEL1_954: What is a string?
**Question:**
en: What is a string in C#?
vi: Kiểu string trong C# là gì?

**Answer:**
en: A string is an immutable reference type that represents a sequence of Unicode characters.
vi: String là một kiểu tham chiếu không thể thay đổi (immutable) đại diện cho một chuỗi các ký tự Unicode.

#### Q_LEVEL1_270: What is StringBuilder?
**Question:**
en: What is StringBuilder?
vi: StringBuilder là gì?

**Answer:**
en: StringBuilder is a mutable string class used to efficiently modify text without creating new objects in memory.
vi: StringBuilder là lớp hỗ trợ thao tác chuỗi có thể thay đổi (mutable), giúp chỉnh sửa văn bản hiệu quả mà không cần tạo các đối tượng mới trong bộ nhớ.

#### Q_LEVEL1_405: What is an Interface?
**Question:**
en: What is an Interface?
vi: Interface là gì?

**Answer:**
en: An interface defines a contract with method signatures but no implementation (prior to C# 8). Classes can implement multiple interfaces.
vi: Interface định nghĩa một hợp đồng chứa chữ ký phương thức nhưng không có nội dung (trước C# 8). Một lớp có thể triển khai nhiều interface.

#### Q_LEVEL1_892: What is an Abstract Class?
**Question:**
en: What is an Abstract Class?
vi: Abstract Class là gì?

**Answer:**
en: An abstract class serves as a base class that cannot be instantiated and may contain both abstract and implemented methods.
vi: Abstract class là lớp cơ sở không thể khởi tạo trực tiếp, có thể chứa cả các phương thức abstract mang tính kế thừa trừu tượng và phương thức đã triển khai.

#### Q_LEVEL1_561: What is a static class?
**Question:**
en: What is a static class?
vi: Static class là gì?

**Answer:**
en: A static class cannot be instantiated, and all its members must be static. It is commonly used for utility/helper methods.
vi: Static class không thể khởi tạo, và mọi thành phần bên trong đều phải là static. Thường được dùng cho các hàm tiện ích chung (helper).

#### Q_LEVEL1_387: What is an Exception?
**Question:**
en: What is an Exception?
vi: Exception là gì?

**Answer:**
en: An exception is an event occurring during execution that disrupts the normal flow of instructions. Handled via try-catch blocks.
vi: Ngoại lệ (Exception) là sự kiện xảy ra trong lúc thực thi làm gián đoạn luồng chạy chương trình bình thường, được xử lý qua khối lệnh try-catch.

#### Q_LEVEL1_744: What is a Delegate?
**Question:**
en: What is a delegate?
vi: Delegate là gì?

**Answer:**
en: A delegate is a type-safe function pointer that holds references to methods with a matching signature.
vi: Delegate là một con trỏ hàm an toàn kiểu (type-safe) có thể tàng trữ tham chiếu đến một hoặc nhiều phương thức theo đúng một nguyên mẫu signature.

#### Q_LEVEL1_156: What is an Event?
**Question:**
en: What is an event in C#?
vi: Event trong C# là gì?

**Answer:**
en: An event is a construct that wraps a delegate to allow a class or object to notify other classes when something of interest occurs.
vi: Event được xây dựng để bọc lại delegate, cho phép một class thông báo (notify/publish) cho các class khác khi một sự kiện đáng chú ý xảy ra thông qua kiến trúc Publisher/Subscriber.

#### Q_LEVEL1_932: What is an Attribute?
**Question:**
en: What is an Attribute?
vi: Attribute là gì?

**Answer:**
en: An attribute is a declarative tag used to convey metadata information about elements like classes, methods, or properties at runtime.
vi: Attribute là các thẻ nhãn (metadata annotations) được dùng để cung cấp thông tin metadata cho class, phương thức hoặc biến vào thời gian chạy thông qua Reflection.

#### Q_LEVEL1_468: What is a Struct?
**Question:**
en: What is a Struct?
vi: Struct trong C# là gì?

**Answer:**
en: A struct is a lightweight value type used to encapsulate small groups of related variables. Structs do not support class inheritance.
vi: Struct là một kiểu dữ liệu giá trị hạng nhẹ dùng để đóng gói nhóm các biến liên quan. Struct không hỗ trợ kế thừa hướng đối tượng như Class.

#### Q_LEVEL1_221: What is 'null'?
**Question:**
en: What does the `null` keyword mean in C#?
vi: Từ khóa `null` có ý nghĩa gì trong C#?

**Answer:**
en: `null` represents the deliberate absence of any object value. It is the default value for reference types.
vi: `null` biểu thị tình trạng rỗng (không trỏ đến đối tượng nào). Đây là giá trị mặc định của các kiểu tham chiếu.

---

### Level 2: Understanding

#### Q_LEVEL2_684: How does Garbage Collection work?
**Question:**
en: Explain how Garbage Collection works.
vi: Giải thích cách hoạt động của Thu gom rác (Garbage Collection).

**Answer:**
en: GC automatically reclaims memory by identifying objects that are no longer accessible by the application codebase, running in three generations (0, 1, 2) to optimize performance dynamically.
vi: GC tự động thu hồi bộ nhớ bằng cách tìm các objects không còn được ai tham chiếu liên kết tới, sử dụng cơ chế gom rác theo ba thế hệ (generation 0, 1, 2) để tự động tối ưu hiệu năng.

#### Q_LEVEL2_150: Compare Struct and Class.
**Question:**
en: Compare struct and class in C#.
vi: So sánh struct và class trong C#.

**Answer:**
en: Structs are value types (stack) suitable for small data, without inheritance. Classes are reference types (heap) supporting full inheritance and polymorphism paradigms.
vi: Struct là kiểu giá trị lưu trên stack phù hợp với dữ liệu nhỏ, không kế thừa. Class là kiểu tham chiếu lưu bên không gian heap, hỗ trợ toàn vẹn kế thừa và tính đa hình.

#### Q_LEVEL2_902: Compare Abstract Class and Interface.
**Question:**
en: Explain the difference between an abstract class and an interface.
vi: Giải thích sự khác biệt giữa abstract class và interface.

**Answer:**
en: A class can inherit only one abstract class but implement multiple interfaces. Abstract classes can have state (fields) and implemented logic; traditionally interfaces could not hold state.
vi: Một lớp chỉ kế thừa được duy nhất 1 abstract class nhưng có thể triển khai không giới hạn interface. Abstract class có thể có state (các fields) và thân hàm; interface thì không duy trì state.

#### Q_LEVEL2_347: Explain IEnumerable vs IQueryable.
**Question:**
en: Explain the difference between `IEnumerable` and `IQueryable` for data querying.
vi: Phân biệt `IEnumerable` và `IQueryable` trong truy vấn dữ liệu.

**Answer:**
en: `IEnumerable` fetches all data from memory, executing filters client-side. `IQueryable` builds expression trees and executes filters translation purely server-side (like SQL databases) which is greatly optimized.
vi: `IEnumerable` tải bộ sưu tập đầy đủ lên RAM sau đó truy vấn client-side trên bộ nhớ. `IQueryable` sinh ra Query biểu thức và biên dịch thành SQL chay server-side tối ưu siêu tốc cho các hệ thống CSDL lớn.

#### Q_LEVEL2_812: Explain const vs readonly.
**Question:**
en: What is the difference between `const` and `readonly` modifiers?
vi: Phân biệt thuộc tính giới hạn `const` và `readonly`?

**Answer:**
en: `const` is evaluated strictly at compile-time and cannot change. `readonly` is evaluated dynamically at runtime and can be safely assigned via a constructor.
vi: `const` bắt buộc phải là một hằng số được định nghĩa tĩnh và đánh giá tại lúc biên dịch (compile-time). `readonly` xử lý linh hoạt thời điểm chạy (runtime) và được phép cấu hình tùy biến khi tạo mới trong constructor.

#### Q_LEVEL2_529: What does the yield keyword do?
**Question:**
en: Explain the purpose of the `yield` keyword.
vi: Giải thích công dụng của từ khóa `yield`.

**Answer:**
en: `yield` is used to build custom iterators smoothly; it returns elements one at a time and tracks the execution state between invocations automatically.
vi: `yield` được dùng để tạo các iterator theo kiểu trì hoãn (lazy evaluation); tác vụ trả về từng phần tử một và tự động ghi nhớ trạng thái chờ cho đến vòng lặp GetNext() tiếp theo.

#### Q_LEVEL2_761: Compare Dispose() vs Finalize().
**Question:**
en: What is the distinct difference between `Dispose()` and `Finalize()`?
vi: Sự khác biệt bản chất giữa `Dispose()` và `Finalize()` là gì?

**Answer:**
en: `Dispose()` is called manually (or wrapped in a `using` block) to explicitly/instantly release unmanaged resources. `Finalize()` is a destructor invoked silently by the Garbage Collector non-deterministically.
vi: `Dispose()` được developer gọi thủ công (hoặc qua `using`) để lập tức giải phóng tài nguyên vô chủ. `Finalize()` là hàm hủy tự động bởi GC, lúc nào gọi là hàm không ai biết nên không thể dựa vào nó về mặt thời gian.

#### Q_LEVEL2_438: Differences between Array and List<T>.
**Question:**
en: Compare array (`[]`) and `List<T>`.
vi: Hãy so sánh mảng `[]` và `List<T>`.

**Answer:**
en: Arrays have a fixed sizing upon initialization, offering absolute maximum memory efficiency. `List<T>` is a dynamic size collection that dynamically reallocates a backing array when more space is needed.
vi: Mảng (Array) có kích thước cố định, sử dụng bộ nhớ hiệu quả nhất. List<T> có khả năng thay đổi kích thước động; khi đầy nó sẽ tự phân bổ một mảng mới to hơn ẩn bên dưới rồi copy dữ liệu sang.

#### Q_LEVEL2_195: Explain var vs dynamic.
**Question:**
en: What is the underlying difference between `var` and `dynamic`?
vi: C# phân biệt `var` và `dynamic` nhờ cơ sở hạ tầng nào?

**Answer:**
en: `var` is strongly-statically typed where the compiler determines the exact explicit type seamlessly. `dynamic` suspends compile-time type checking, deferring binding to the runtime DLR.
vi: `var` vẫn có kiểu tĩnh mạnh (static typed), trình phân tích tự đọc rồi đóng thế kiểu đúng cố định lúc biên dịch. `dynamic` bỏ qua kiểm tra biên dịch, chỉ lo giải quyết khi code thật sự đem ra chạy. (runtime typing).

#### Q_LEVEL2_603: What is Method Overloading?
**Question:**
en: What is Method Overloading?
vi: Method Overloading (Nạp chồng phương thức) là gì?

**Answer:**
en: Permitting multiple methods to share the same name providing they have different parameters (type, count, order) within the exact same class (compile-time polymorphism).
vi: Đóng vai trò là tính đa hình (Polymorphism) lúc compile-time; cho phép khai báo nhiều phương thức cùng tên bên trong một class bằng ngoại lệ số lượng và kiểu biến đầu vào phải khác biệt.

#### Q_LEVEL2_856: What is Method Overriding?
**Question:**
en: What is Method Overriding?
vi: Method Overriding (Ghi đè phương thức) là gì?

**Answer:**
en: Delivering an updated implementation in a derived class for an inherited virtual or abstract base system method (runtime polymorphism).
vi: Là thủ tục đa hình lúc runtime; cho phép các lớp con thay đổi và viết lại logic mới trên nền tảng phương thức dùng chung thừa kế bằng `virtual` / `abstract` của lớp cha.

#### Q_LEVEL2_214: Early Binding vs Late Binding.
**Question:**
en: Describe Early Binding vs Late Binding.
vi: Binding sớm (Early Binding) và Binding trễ (Late Binding) khác nhau thế nào?

**Answer:**
en: Early binding handles object types mapping completely at compile-time for ultimate safety. Late binding resolves dependencies and invokes dynamically at runtime (using Reflection).
vi: Early binding kiểm định kiểu và trỏ hàm trong giai đoạn mã hóa lúc compile-time. Late binding để mã tự đối phó tại không gian ảo runtime qua việc tự phân giải kiểu như Dynamic / Reflection.

#### Q_LEVEL2_980: Func, Action, and Predicate.
**Question:**
en: Explain Func, Action, and Predicate delegates.
vi: Phân tích Func, Action, và Predicate .

**Answer:**
en: `Func` dictates a returned value function, `Action` performs a void procedure, and `Predicate` dictates returning a boolean evaluation logic.
vi: Là các delegate generic đại diện mặc định: `Func` quy định hàm luôn trả mốc Output, `Action` quy định rỗng Void, và `Predicate` dành riêng xử lý hàm thẩm định true/false.

#### Q_LEVEL2_375: How does async/await work?
**Question:**
en: Explain how `async`/`await` behaves structurally behind the scenes.
vi: Giải thích cơ chế ngầm của kiến trúc `async`/`await`.

**Answer:**
en: The code translates `async` methods into struct state machines, offloading the waiting delay thread async while restoring code flow non-blockingly afterwards.
vi: Compiler xả hàm biến chúng thành một state machine. Bàn giao thao tác treo IO xuống Thread dưới, trả lại thread GUI/User Interface gốc giúp phần mềm nhẹ nhàng không bị giật lác hay đơ màn hình.

#### Q_LEVEL2_509: What is Dependency Injection?
**Question:**
en: What is Dependency Injection (DI)?
vi: Dependency Injection (DI - Tiêm phụ thuộc) là gì?

**Answer:**
en: A powerful IoC core rule implementation where required external services are injected from constructors, encouraging massive loose coupling design choices.
vi: DI là một biến thể Inversion of Control chèn phụ thuộc của 1 lớp từ thế giới bên ngoài (qua constructor / framework service container), giúp tạo nên cấu hình kết nối lỏng tuyệt đỉnh hoàn mỹ trên diện rộng.

---

### Level 3: Applying

#### Q_LEVEL3_417: Demonstrate async/await usage.
**Question:**
en: Demonstrate how to fetch textual payload asynchronously using an `HttpClient`.
vi: Trình diễn việc lấy nội dung mạng xử lý bất đồng bộ sử dụng thư viện `HttpClient`.

**Answer:**
en: Tying the asynchronous tasks directly to the `await` operator yields smooth background network streaming while avoiding blocking GUI thread halts.
vi: Sử dụng `Task` song song với từ khóa `await` giải thoát luồng giao diện chính để trình chạy network nền thu thập chuỗi mạng.

```csharp
public async Task<string> FetchSiteContentAsync(string uri)
{
    using var httpAgent = new HttpClient();
    string sourceText = await httpAgent.GetStringAsync(uri);
    return sourceText;
}
```

#### Q_LEVEL3_862: Demonstrate Singleton Pattern safely.
**Question:**
en: Show a foolproof thread-safe implementation of the Singleton architecture.
vi: Triển khai pattern Singleton an toàn chống lỗi luồng hoàn toàn trong hệ sinh thái C#.

**Answer:**
en: Taking advantage of .NET framework's base library `Lazy<T>` offers extreme reliability effectively guaranteeing a thread-safe execution structure.
vi: Khai thác tuyệt đối tính năng của cấu trúc `.Lazy<T>` có sẵn trong lõi hệ thống đảm báo đồng bộ hóa luồng mà tự thân nó xử lý mà ta không cần nhúng tay dùng lock khóa phiền phức. 

```csharp
public sealed class ThreadSafeLogger
{
    private static readonly Lazy<ThreadSafeLogger> lazyInstance = new(() => new ThreadSafeLogger());
    
    private ThreadSafeLogger() {} // lock initialization scope

    public static ThreadSafeLogger MasterInstance => lazyInstance.Value;
    
    public void Document(string audit) 
    {
        // Internal persistence logic...
    }
}
```

#### Q_LEVEL3_120: Demonstrate an Extension Method.
**Question:**
en: Show how to produce an extension method for checking boolean true/false parity against null string elements.
vi: Minh họa đoạn mở rộng Method Extend lồng vào chuỗi string để xác thực nó trống.

**Answer:**
en: Build a distinct unchangeable module static blueprint and utilize `this` against the targeted original component data class target.
vi: Viết lớp tiện ích static đóng băng trỏ `this` vào parameter gốc cần can thiệp để lồng hàm bọc vào nó tự nhiên như mặc định.

```csharp
public static class TextStringExtensionsModule
{
    // Extending native .NET object strings
    public static bool EnsureClean(this string payload)
    {
        return string.IsNullOrWhiteSpace(payload);
    }
}
// Using the custom built logic smoothly
// bool isclean = "My Secret Data!".EnsureClean();
```

#### Q_LEVEL3_758: Create a Custom Exception.
**Question:**
en: Implement a custom specialized Exception.
vi: Viết và tùy chỉnh class văng lỗi Ngoại Lệ tùy chọn.

**Answer:**
en: Make sure to base the subclass upon standard generic system `Exception` class library while routing arguments securely downward.
vi: Bắt buộc cấu trúc lớp có quan hệ kế thừa base với `Exception` của hệ thống cơ bản và cung cấp tham số constructor.

```csharp
public class TransactionFailureException : Exception
{
    public TransactionFailureException() { }
    
    public TransactionFailureException(string warnDesc) 
        : base(warnDesc) { }
        
    public TransactionFailureException(string warnDesc, Exception parentErrorReference) 
        : base(warnDesc, parentErrorReference) { }
}
```

#### Q_LEVEL3_904: Demonstrate a basic LINQ Query.
**Question:**
en: Provide code filtering criteria and picking data through advanced query collections LINQ.
vi: Cho ví dụ dùng cấu trúc LINQ truy vấn, lọc, và bốc tách danh sách.

**Answer:**
en: Implement Lambda chains containing `.Where()` mapping logic mixed with custom variable projection constraints natively.
vi: Lồng biểu thức tính toán thông qua cấu trúc chuỗi hàm nối kết nhau như `.Where()` để làm phễu đẩy mảng thu gọn lại kết quả nhanh gọn siêu tiện lợi.

```csharp
// Mocking array db
var dbInts = new List<int> { 50, 112, 18, 93, 21 };

// Execute chain pipeline 
var extractedEvens = dbInts.Where(record => record > 40 && record % 2 == 0)
                           .OrderBy(record => record)
                           .ToList(); 
// List has 50, 112
```
