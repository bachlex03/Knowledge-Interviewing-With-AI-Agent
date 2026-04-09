**Question 1:**
Module cấp cao là gì? Module cấp thấp là gì?
Ý nghĩa của câu "Cả hai nên phụ thuộc vào abstraction (interface)."

**Answer:** Nguyên lý **Dependency Inversion Principle (DIP) - Đảo ngược Phụ thuộc** là một trong những khái niệm quan trọng nhất để xây dựng các kiến trúc phần mềm lớn (như Clean Architecture hay Microservices).

Để hiểu rõ, chúng ta hãy bóc tách từng khái niệm bằng cả lý thuyết lẫn những ví dụ đời thực.

### 1. Module cấp cao (High-level Module) là gì?

- **Định nghĩa:** Đây là "bộ não" của ứng dụng. Nó chứa các **logic nghiệp vụ cốt lõi (Business Logic)** và các quy tắc hoạt động quan trọng nhất. Nó trả lời cho câu hỏi: _"Ứng dụng này sinh ra để giải quyết bài toán gì của con người?"_.
- **Đặc điểm:** Nó mang tính trừu tượng cao, gần gũi với ngôn ngữ của con người/doanh nghiệp. Nó không quan tâm đến các yếu tố kỹ thuật rườm rà.
- **Ví dụ trong phần mềm:** Class `OrderProcessor` (Xử lý đơn hàng), `PayrollCalculator` (Tính lương), `UserAuthentication` (Xác thực người dùng).
- **Ví dụ đời thực:** Vị Giám đốc điều hành (CEO) của một công ty. CEO ra quyết định: "Hôm nay chúng ta sẽ chạy chiến dịch Marketing giảm giá 20%".

### 2. Module cấp thấp (Low-level Module) là gì?

- **Định nghĩa:** Đây là "chân tay" của ứng dụng. Nó chứa các **chi tiết triển khai kỹ thuật (Implementation Details)**. Nó trực tiếp giao tiếp với phần cứng, mạng, cơ sở dữ liệu, hoặc các hệ thống bên ngoài. Nó trả lời cho câu hỏi: _"Làm công việc đó bằng công cụ công nghệ nào?"_.
- **Đặc điểm:** Nó mang tính chi tiết, cụ thể, thao tác với byte, string, network request, v.v.
- **Ví dụ trong phần mềm:** `MySqlDatabaseConnector` (Kết nối MySQL), `MongoDbRepository` (Lưu vào MongoDB), `SendGridEmailSender` (Gửi email qua SendGrid), `FileWriter` (Ghi file txt).
- **Ví dụ đời thực:** Cậu nhân viên giao hàng, chiếc máy in trong văn phòng, công ty chạy quảng cáo Facebook. (Họ trực tiếp thực hiện công việc cụ thể).

---

### 3. Ý nghĩa của câu: "Cả hai nên phụ thuộc vào Abstraction (Interface)"

Để hiểu câu này, hãy xem **cách thiết kế truyền thống (vi phạm DIP)**:

- Mô hình truyền thống là: **Cấp cao -> gọi trực tiếp -> Cấp thấp**.
- Ví dụ: Class `OrderProcessor` (Cấp cao) khởi tạo thẳng class `MySqlDatabase` (Cấp thấp) để lưu dữ liệu.
- Hậu quả: Giám đốc (Cấp cao) bị phụ thuộc hoàn toàn vào anh nhân viên giao hàng tên Tèo (Cấp thấp). Nếu anh Tèo nghỉ việc (hoặc công ty muốn đổi từ MySQL sang MongoDB), Giám đốc không biết làm việc với ai khác, mọi quy trình bị đình trệ. Code của bộ não ứng dụng phải bị bới móc ra để sửa lại.

**Cách giải quyết của DIP: "Cả hai đều phụ thuộc vào Abstraction"**

Abstraction (Interface) ở đây đóng vai trò như một **Bản hợp đồng** hoặc một **Tiêu chuẩn chung**. Cụm từ "Đảo ngược" có nghĩa là thay vì Cấp cao chạy theo Cấp thấp, bây giờ chúng ta đảo ngược luồng phụ thuộc: đặt một cái Interface ở giữa.

Hãy lấy ví dụ về **Cái Tivi và Ổ cắm điện**:

1.  **Tivi (Module Cấp Cao):** Nhiệm vụ của nó là phát hình ảnh, âm thanh. Nó _không phụ thuộc_ vào việc điện ngoài cột điện là điện hạt nhân, điện thủy điện hay điện gió (Module cấp thấp). Nó chỉ **phụ thuộc vào Abstraction**: "Tôi cần một dòng điện 220V lấy từ một cái phích cắm chuẩn 2 chấu tròn".
2.  **Trạm điện & Đường dây (Module Cấp Thấp):** Nhiệm vụ của nó là tạo ra điện và truyền đi. Nó _cũng không phụ thuộc_ vào việc nhà bạn xài Tivi Samsung hay Tủ lạnh LG. Nó chỉ **phụ thuộc vào Abstraction**: "Tôi cam kết cung cấp đúng điện 220V qua cái lỗ cắm 2 chấu tròn gắn trên tường".
3.  **Ổ cắm điện trên tường (Abstraction/Interface):** Đây chính là nơi Cấp cao và Cấp thấp gặp nhau.

**Áp dụng vào lập trình:**
Thay vì class `OrderProcessor` (Cấp cao) gọi thẳng `MySqlDatabase` (Cấp thấp), chúng ta tạo ra một interface tên là `IDatabase` (Ổ cắm điện).

- **Cấp cao phụ thuộc vào Abstraction:** `OrderProcessor` được code theo kiểu: _"Tôi không cần biết đằng sau là database gì, tôi chỉ gọi hàm `Save()` của interface `IDatabase`"_.
- **Cấp thấp phụ thuộc vào Abstraction:** Các class `MySqlDatabase` hay `MongoDbDatabase` phải implement (thực thi) cái interface `IDatabase` đó: _"Tôi hứa là tôi có hàm `Save()` chuẩn như interface yêu cầu"_.

**Kết quả tuyệt vời:**
Bộ não của hệ thống (Cấp cao) được bảo vệ hoàn toàn. Bạn có thể rút phích cắm `MySqlDatabase` ra, cắm cái `MongoDbDatabase` vào ổ điện, và Tivi (`OrderProcessor`) vẫn tiếp tục chạy hoàn hảo mà không cần phải thay đổi dù chỉ một dòng code bên trong nó. Mọi thứ trở nên linh hoạt, rời rạc (loose coupling) và vô cùng dễ bảo trì.

===============

**Question 2:** Cho 1 code ví dụ về Mô hình truyền thống

**Answer:** Dưới đây là một ví dụ bằng C# minh họa cho **Mô hình truyền thống (Vi phạm DIP)**.

Trong mô hình này, chúng ta sẽ thấy sự phụ thuộc trực tiếp (Tight Coupling) giữa module cấp cao và module cấp thấp thông qua từ khóa `new`.

```csharp
using System;

// ==========================================
// 1. MODULE CẤP THẤP (Chi tiết kỹ thuật)
// ==========================================
public class MySqlDatabase
{
    public void Connect()
    {
        Console.WriteLine("Đang kết nối tới MySQL Server...");
    }

    public void SaveOrder(string orderData)
    {
        Console.WriteLine($"[MySQL] Đã lưu thành công: {orderData}");
    }
}

// ==========================================
// 2. MODULE CẤP CAO (Logic nghiệp vụ)
// ==========================================
public class OrderProcessor
{
    // LỖI THIẾT KẾ Ở ĐÂY:
    // Module cấp cao tự tay khởi tạo trực tiếp Module cấp thấp.
    // Việc này "trói chặt" OrderProcessor với MySQL.
    private MySqlDatabase _database = new MySqlDatabase();

    public void ProcessOrder(string orderDetails)
    {
        Console.WriteLine("Bắt đầu xử lý logic nghiệp vụ của đơn hàng...");

        // Gọi trực tiếp chi tiết kỹ thuật
        _database.Connect();
        _database.SaveOrder(orderDetails);
    }
}

// ==========================================
// 3. CHƯƠNG TRÌNH CHÍNH
// ==========================================
class Program
{
    static void Main()
    {
        OrderProcessor processor = new OrderProcessor();
        processor.ProcessOrder("Đơn hàng #123 - Khách VIP");
    }
}
```

### Tại sao đoạn code trên lại là một "Quả bom nổ chậm"?

Nhìn lướt qua, đoạn code trên chạy rất hoàn hảo và đúng yêu cầu. Nhưng khi dự án lớn lên, nó sẽ bộc lộ 3 vấn đề chí mạng:

1. **Khó khăn khi thay đổi công nghệ:** Nếu công ty quyết định chuyển từ MySQL sang SQL Server hoặc lưu lên Cloud, bạn sẽ phải mở class `OrderProcessor` (nơi chứa logic nghiệp vụ cực kỳ quan trọng) ra để sửa đổi mã nguồn. Việc chạm vào code nghiệp vụ đang chạy ổn định là điều cấm kỵ vì rất dễ sinh ra lỗi mới.
2. **Không thể viết Unit Test:** Khi bạn muốn viết một bài test tự động (Unit Test) để kiểm tra xem thuật toán xử lý của `OrderProcessor` có đúng không, nó sẽ **bắt buộc** phải kết nối với một Database MySQL thật sự. Nếu lúc đó không có mạng internet hoặc server database bị sập, bài test sẽ thất bại (mặc dù code của bạn không sai).
3. **Phát triển song song gặp khó:** Đội làm Database và đội làm Logic phải đợi nhau. Đội Logic không thể code được `OrderProcessor` nếu đội Database chưa viết xong class `MySqlDatabase`.

Nhờ áp dụng nguyên lý DIP (chèn một Interface ở giữa như ví dụ ở câu trước), bạn sẽ giải quyết triệt để được cả 3 vấn đề này.

Đây cũng là nguyên tắc cuối cùng trong chuỗi 5 nguyên tắc SOLID. Bạn cảm thấy nguyên tắc nào trong 5 nguyên tắc này là dễ nhầm lẫn hoặc khó áp dụng nhất trong thực tế?

===============

**Question 3:** Ý nghĩa của câu "Abstraction không nên phụ thuộc vào chi tiết, mà chi tiết phải phụ thuộc vào abstraction"

**Answer:**
