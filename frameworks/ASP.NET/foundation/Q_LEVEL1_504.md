**Question 1:** sealed, record có từ phiên bản nào?

**Answer:** Trong hệ sinh thái .NET, cả `sealed` (dành cho class) và `record` đều là những từ khóa quan trọng, nhưng chúng xuất hiện ở hai giai đoạn rất khác nhau của ngôn ngữ C#:

### 1. `sealed` (Niêm phong)

Từ khóa `sealed` đã xuất hiện từ rất sớm, ngay từ phiên bản **C# 1.0** (cùng với sự ra đời của .NET Framework vào năm 2002).

- **Mục đích:** Ngăn chặn việc kế thừa từ một lớp (class) hoặc ngăn chặn việc ghi đè (override) một phương thức cụ thể trong các lớp con.
- **Kỹ thuật:** Khi một class được đánh dấu là `sealed`, trình biên dịch sẽ không cho phép bất kỳ class nào khác dùng nó làm base class. Điều này giúp tối ưu hóa hiệu suất (JIT có thể thực hiện _devirtualization_) và đảm bảo tính bảo mật/thiết kế của hệ thống.

---

### 2. `record` (Bản ghi)

Từ khóa `record` hiện đại hơn rất nhiều, được giới thiệu qua hai giai đoạn chính để hoàn thiện:

- **C# 9.0 (.NET 5 - Năm 2020):** Lần đầu tiên xuất hiện. Ở phiên bản này, `record` mặc định được hiểu là một **Reference Type** (tương đương với một `class` nhưng có các tính năng đặc biệt như _value-based equality_ và _with-expressions_).
- **C# 10 (.NET 6 - Năm 2021):** Bổ sung thêm `record struct`. Từ đây, bạn có thể định nghĩa `record class` (kiểu tham chiếu) hoặc `record struct` (kiểu tham trị) để linh hoạt hơn trong việc quản lý bộ nhớ.

---

### Bảng tổng hợp

| Tính năng         | Phiên bản C# | Phiên bản .NET tương ứng | Đặc điểm chính                                                      |
| :---------------- | :----------- | :----------------------- | :------------------------------------------------------------------ |
| **sealed**        | C# 1.0       | .NET Framework 1.0       | Chống kế thừa, có từ thời "sơ khai".                                |
| **record**        | C# 9.0       | .NET 5                   | Định nghĩa kiểu dữ liệu bất biến (immutable), so sánh theo giá trị. |
| **record struct** | C# 10        | .NET 6                   | Kết hợp sức mạnh của `record` vào cấu trúc dữ liệu `struct`.        |

**Lưu ý thú vị:** Bạn có thể kết hợp cả hai để tạo ra một **`sealed record`**. Điều này cực kỳ phổ biến trong C# hiện đại để đảm bảo tính bất biến tối đa và hiệu suất tối ưu cho các Data Transfer Objects (DTOs).
