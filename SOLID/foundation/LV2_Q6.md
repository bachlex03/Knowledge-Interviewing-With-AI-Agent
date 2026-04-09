Việc áp dụng các nguyên tắc SOLID là một trong những bước chuyển mình quan trọng nhất để xây dựng kiến trúc phần mềm sạch, đặc biệt là khi làm việc với các hệ thống web phức tạp cần khả năng mở rộng liên tục.

### Tại sao nên dùng SOLID? (Bức tranh Trước và Sau)

- **Trước khi dùng SOLID:** Code thường có xu hướng biến thành "God Object" (một class ôm đồm mọi thứ). Các thành phần dính chặt vào nhau (tight coupling). Khi có yêu cầu thay đổi (ví dụ: đổi database từ SQL Server sang MongoDB, hoặc thêm phương thức thanh toán mới), bạn phải sửa code ở rất nhiều nơi. Điều này dễ gây ra lỗi dây chuyền (regression bugs), khó viết Unit Test và khiến những người mới vào dự án mất rất nhiều thời gian để đọc hiểu.
- **Sau khi dùng SOLID:** Hệ thống được chia nhỏ thành các module/class có nhiệm vụ rõ ràng, độc lập và kết nối với nhau thông qua các bản thiết kế (Interface/Abstraction). Bạn có thể dễ dàng thêm tính năng mới bằng cách "cắm" (plug) thêm code mới mà không cần chạm vào những đoạn code cũ đang chạy ổn định. Code trở nên dễ bảo trì, dễ test và dễ tái sử dụng.

Dưới đây là Demo cụ thể bằng C# cho từng nguyên tắc.

---

### 1. S - Single Responsibility Principle (SRP)

**Một class chỉ nên có một lý do để thay đổi (chỉ làm một việc).**

**❌ Trước khi dùng (Vi phạm):** Class `UserService` vừa xử lý logic lưu dữ liệu người dùng, vừa kiêm luôn việc ghi log lỗi ra file.

```csharp
public class UserService
{
    public void RegisterUser(string username, string password)
    {
        try
        {
            // Logic lưu user vào Database
            Console.WriteLine($"Đã đăng ký user: {username}");
        }
        catch (Exception ex)
        {
            // Vi phạm SRP: Xử lý logic ghi log
            System.IO.File.WriteAllText("error.txt", ex.ToString());
        }
    }
}
```

**✅ Sau khi dùng:** Tách logic ghi log ra một class riêng. `UserService` chỉ lo nghiệp vụ của User.

```csharp
public class ErrorLogger
{
    public void LogError(string message)
    {
        System.IO.File.WriteAllText("error.txt", message);
    }
}

public class UserService
{
    private readonly ErrorLogger _logger = new ErrorLogger();

    public void RegisterUser(string username, string password)
    {
        try
        {
            // Logic lưu user vào Database
            Console.WriteLine($"Đã đăng ký user: {username}");
        }
        catch (Exception ex)
        {
            _logger.LogError(ex.ToString());
        }
    }
}
```

---

### 2. O - Open/Closed Principle (OCP)

**Mở rộng thì thoải mái, nhưng đóng với việc sửa đổi.**

**❌ Trước khi dùng (Vi phạm):** Giả sử bạn cần tính chiết khấu cho các loại thành viên khác nhau. Mỗi lần có thêm hạng thành viên mới (VVIP), bạn phải vào sửa trực tiếp hàm `CalculateDiscount` và thêm `if/else`.

```csharp
public class DiscountCalculator
{
    public double CalculateDiscount(string memberType, double amount)
    {
        if (memberType == "Standard") return amount * 0.05;
        if (memberType == "VIP") return amount * 0.10;
        // Nếu thêm VVIP, phải sửa code ở đây!
        return 0;
    }
}
```

**✅ Sau khi dùng:** Dùng đa hình (Polymorphism). Khi có hạng thành viên mới, chỉ cần tạo class mới kế thừa interface mà không đụng đến code cũ.

```csharp
public interface IDiscountStrategy
{
    double Calculate(double amount);
}

public class StandardDiscount : IDiscountStrategy
{
    public double Calculate(double amount) => amount * 0.05;
}

public class VIPDiscount : IDiscountStrategy
{
    public double Calculate(double amount) => amount * 0.10;
}

// Giờ đây thêm class VVIPDiscount : IDiscountStrategy rất dễ dàng
```

---

### 3. L - Liskov Substitution Principle (LSP)

**Class con phải thay thế được class cha mà không làm hỏng ứng dụng.**

**❌ Trước khi dùng (Vi phạm):** Một ví dụ kinh điển khi làm việc với thao tác ghi/đọc dữ liệu. Cả Database thường và Read-Only Database đều kế thừa từ `Database`, nhưng Read-Only lại ném ra lỗi nếu gọi hàm `Save()`.

```csharp
public class Database
{
    public virtual void Read() { /* ... */ }
    public virtual void Save() { /* ... */ }
}

public class ReadOnlyDatabase : Database
{
    public override void Read() { /* Đọc dữ liệu */ }

    public override void Save()
    {
        // Vi phạm LSP: Class con thay đổi hành vi chuẩn của class cha bằng cách ném Exception
        throw new NotImplementedException("Không thể ghi vào Read-Only DB");
    }
}
```

**✅ Sau khi dùng:** Tách biệt các hành vi thông qua Interface để đảm bảo class con chỉ implement những gì nó thực sự làm được.

```csharp
public interface IReadable { void Read(); }
public interface IWritable { void Save(); }

public class RegularDatabase : IReadable, IWritable
{
    public void Read() { /* ... */ }
    public void Save() { /* ... */ }
}

public class ReadOnlyDatabase : IReadable
{
    public void Read() { /* ... */ }
    // Không hề có hàm Save(), hoàn toàn an toàn khi gọi
}
```

---

### 4. I - Interface Segregation Principle (ISP)

**Nên tạo nhiều interface nhỏ lẻ, chuyên biệt thay vì một interface khổng lồ.**

**❌ Trước khi dùng (Vi phạm):** Bạn có một Interface cho dịch vụ Cloud lưu trữ. Nhưng nếu ứng dụng chỉ dùng Cloud để lưu File mà không cần chạy Server hay CDN, việc implement Interface này là thừa thãi.

```csharp
public interface ICloudProvider
{
    void StoreFile();
    void SetupServer();
    void SetupCDN();
}

public class BasicFileStorage : ICloudProvider
{
    public void StoreFile() { /* Logic lưu file */ }

    // Bị ép buộc phải implement những thứ không cần thiết
    public void SetupServer() => throw new NotImplementedException();
    public void SetupCDN() => throw new NotImplementedException();
}
```

**✅ Sau khi dùng:** Chia nhỏ Interface theo từng khả năng (Capability).

```csharp
public interface IFileStorage { void StoreFile(); }
public interface IServerHosting { void SetupServer(); }
public interface ICDNService { void SetupCDN(); }

// Bây giờ class chỉ cần implement đúng thứ nó cần
public class BasicFileStorage : IFileStorage
{
    public void StoreFile() { /* Logic lưu file */ }
}

public class FullAWSCloud : IFileStorage, IServerHosting, ICDNService
{
    public void StoreFile() { /* ... */ }
    public void SetupServer() { /* ... */ }
    public void SetupCDN() { /* ... */ }
}
```

---

### 5. D - Dependency Inversion Principle (DIP)

**Module cấp cao không nên phụ thuộc vào module cấp thấp. Cả hai nên phụ thuộc vào Abstraction.**

**❌ Trước khi dùng (Vi phạm):** `UserController` (cấp cao) trực tiếp khởi tạo (bằng từ khóa `new`) `SqlUserRepository` (cấp thấp). Nếu muốn đổi sang `MongoUserRepository`, bạn phải đi tìm và sửa dòng `new SqlUserRepository()` trên toàn bộ Project.

```csharp
public class SqlUserRepository
{
    public void SaveUser() { Console.WriteLine("Lưu vào SQL Server"); }
}

public class UserController
{
    // Cột chặt (Tight coupling) vào implementation cụ thể
    private readonly SqlUserRepository _repository = new SqlUserRepository();

    public void Register()
    {
        _repository.SaveUser();
    }
}
```

**✅ Sau khi dùng:** Dùng Interface (`IUserRepository`). Khởi tạo thông qua Constructor (Kỹ thuật Dependency Injection - DI). `UserController` giờ đây không cần biết dữ liệu được lưu ở đâu, chỉ cần biết hàm `SaveUser()` tồn tại.

```csharp
public interface IUserRepository
{
    void SaveUser();
}

public class SqlUserRepository : IUserRepository
{
    public void SaveUser() { Console.WriteLine("Lưu vào SQL Server"); }
}

public class MongoUserRepository : IUserRepository
{
    public void SaveUser() { Console.WriteLine("Lưu vào MongoDB"); }
}

public class UserController
{
    private readonly IUserRepository _repository;

    // Tiêm dependency qua constructor (Loose coupling)
    public UserController(IUserRepository repository)
    {
        _repository = repository;
    }

    public void Register()
    {
        _repository.SaveUser();
    }
}
```
