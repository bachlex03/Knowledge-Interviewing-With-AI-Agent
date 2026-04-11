#### Q_LEVEL2_688: Configuration and model validation pitfalls

**Question:**
en: What are common mistakes around configuration binding and model validation in ASP.NET Core?
vi: Những lỗi phổ biến quanh configuration binding và model validation trong ASP.NET Core là gì?

**Answer:**
en: Developers often bind configuration to a class and assume the values are valid without validating required settings at startup. Another common mistake is assuming model binding or validation always succeeded, then using invalid DTO data directly. Practical advice: validate options early, know what `[ApiController]` does for automatic 400 responses, and never trust incoming models blindly.
vi: Nhiều người bind cấu hình vào class rồi mặc định tin rằng giá trị luôn hợp lệ, đến lúc chạy production mới lộ ra setting thiếu hoặc sai kiểu. Với request DTO cũng vậy, nhiều người nghĩ model binding chắc chắn thành công rồi dùng dữ liệu ngay mà không kiểm tra validation. Kinh nghiệm thực tế là: validate options từ sớm, hiểu `[ApiController]` tự trả `400` trong trường hợp nào, và tuyệt đối không tin dữ liệu đầu vào một cách mù quáng.

#### Q_LEVEL2_577: EF Core query behavior

**Question:**
en: Explain a few common EF Core query mistakes that look harmless but hurt API performance.
vi: Hãy giải thích vài lỗi truy vấn EF Core tưởng vô hại nhưng rất dễ làm API chậm đi.

**Answer:**
en: Common mistakes include calling `ToList()` too early, loading full entities when only a few fields are needed, forgetting `AsNoTracking()` for read-only queries, and causing N+1 query patterns by navigating related data carelessly. These issues increase memory usage, round trips, and SQL cost. In interviews, mention that `IQueryable` composes the SQL, while materialization should happen as late as practical.
vi: Các lỗi phổ biến gồm `ToList()` quá sớm, lấy nguyên entity trong khi chỉ cần vài cột, quên `AsNoTracking()` cho truy vấn chỉ đọc, hoặc truy cập navigation thiếu kiểm soát làm phát sinh kiểu N+1 query. Những lỗi này làm tăng bộ nhớ, tăng số lần round-trip xuống DB và SQL sinh ra cũng nặng hơn. Khi đi phỏng vấn, nên nhấn mạnh rằng `IQueryable` còn đang xây câu SQL, còn việc materialize dữ liệu nên diễn ra muộn và có chủ đích.

#### Q_LEVEL2_712: Async/await mistakes in ASP.NET Core

**Question:**
en: Why are `async void`, `.Result`, and `.Wait()` common mistakes in ASP.NET Core request handling?
vi: Vì sao `async void`, `.Result` và `.Wait()` là các lỗi hay gặp trong luồng xử lý request của ASP.NET Core?

**Answer:**
en: `async void` hides completion and exception flow, so the framework cannot await it properly. `.Result` and `.Wait()` block threads while waiting for I/O, which reduces scalability and can create thread-pool starvation under load. In ASP.NET Core, request code should usually return `Task` or `Task<T>` and use `await` end-to-end.
vi: `async void` làm mất khả năng chờ hoàn tất và bắt exception theo cách chuẩn, nên framework không kiểm soát được vòng đời lời gọi đó. Còn `.Result` và `.Wait()` thì khóa thread lại trong lúc chờ I/O, làm giảm khả năng scale và dễ gây thread-pool starvation khi tải tăng cao. Trong ASP.NET Core, cách an toàn là để action/service trả về `Task` hoặc `Task<T>` và `await` xuyên suốt.

#### Q_LEVEL2_668: DI lifetimes and thread safety

**Question:**
en: Why is injecting request-specific or mutable state into a Singleton a common ASP.NET Core pitfall?
vi: Vì sao việc nhét state theo request hoặc mutable state vào Singleton là một pitfall rất phổ biến trong ASP.NET Core?

**Answer:**
en: A Singleton lives for the whole application, so any mutable data inside it is shared across all requests and threads. That causes race conditions, stale user data, and lifetime mismatches if it depends on `Scoped` services such as `DbContext`. In interviews, the key point is: a Singleton must be thread-safe and should not hold per-request state.
vi: `Singleton` sống suốt vòng đời ứng dụng nên mọi dữ liệu mutable bên trong nó sẽ bị chia sẻ cho tất cả request và tất cả thread. Kết quả là rất dễ dính race condition, giữ nhầm dữ liệu user cũ, và còn lệch lifetime nếu nó phụ thuộc vào service `Scoped` như `DbContext`. Ý chính khi trả lời phỏng vấn là: `Singleton` phải thread-safe và không nên giữ state theo từng request.

#### Q_LEVEL1_672: What is the common `HttpClient` lifetime mistake?

**Question:**
en: What is a common mistake when using `HttpClient` in ASP.NET Core?
vi: Lỗi phổ biến khi dùng `HttpClient` trong ASP.NET Core là gì?

**Answer:**
en: A common mistake is creating and disposing a new `HttpClient` per request. That can lead to socket exhaustion and unstable outbound calls. In ASP.NET Core, prefer `IHttpClientFactory`.
vi: Lỗi rất thường gặp là `new HttpClient()` liên tục cho từng lần gọi API rồi dispose ngay. Cách đó dễ làm cạn socket, DNS refresh không ổn định và hệ thống outbound bị chập chờn. Trong ASP.NET Core nên ưu tiên `IHttpClientFactory`.
