# NodeJS Pitfalls Q&A

### Level 1: Remembering

#### Q_LEVEL1_812: What is the common mistake with blocking the Event Loop?

**Question:**
en: What is a common mistake related to blocking the Event Loop in NodeJS?
vi: Lỗi phổ biến liên quan đến việc chặn **Event Loop** trong NodeJS là gì?

**Answer:**
en: A common mistake is putting CPU-heavy work, large synchronous loops, or sync file operations on the main thread. That blocks other requests, timers, and callbacks, so latency spikes even if the service looks simple.
vi: Lỗi phổ biến là đặt tác vụ nặng CPU, vòng lặp đồng bộ lớn hoặc thao tác file đồng bộ lên main thread. Điều đó chặn request khác, timer và callback, khiến độ trễ tăng mạnh dù service nhìn có vẻ đơn giản.

#### Q_LEVEL1_824: What is the async await misunderstanding?

**Question:**
en: What is a common misunderstanding when using `async` and `await` in NodeJS?
vi: Hiểu lầm phổ biến khi dùng `async` và `await` trong NodeJS là gì?

**Answer:**
en: A common misunderstanding is thinking `await` makes work non-blocking by itself. `await` only pauses the async function until the Promise settles. If the awaited code does CPU-heavy or sync work, the Event Loop is still blocked.
vi: Hiểu lầm thường gặp là nghĩ `await` tự nó làm mọi thứ trở nên non-blocking. Thực ra `await` chỉ tạm dừng async function cho đến khi Promise hoàn tất. Nếu phần code được chờ vẫn chạy việc nặng CPU hoặc code đồng bộ thì **Event Loop** vẫn bị chặn.

#### Q_LEVEL1_836: What is the danger of unhandled promise rejections?

**Question:**
en: Why are unhandled Promise rejections a serious NodeJS pitfall?
vi: Vì sao **unhandled Promise rejection** là một pitfall nghiêm trọng trong NodeJS?

**Answer:**
en: Unhandled rejections can hide failures, leave requests hanging, or crash the process depending on runtime behavior and configuration. Mature services handle Promise errors explicitly and make failures observable.
vi: **Unhandled rejection** có thể che giấu lỗi, làm request bị treo hoặc thậm chí làm process dừng tùy runtime và cấu hình. Service trưởng thành phải xử lý lỗi Promise rõ ràng và làm cho failure có thể quan sát được.

#### Q_LEVEL1_847: What is the common listener and cache leak mistake?

**Question:**
en: What is a common mistake that leads to memory leaks from listeners or caches in NodeJS?
vi: Lỗi phổ biến nào dẫn đến **memory leak** từ listener hoặc cache trong NodeJS?

**Answer:**
en: A common mistake is adding event listeners repeatedly without removing them, or storing data in in-memory caches without limits or eviction. Over time, memory usage grows and the process becomes unstable.
vi: Lỗi hay gặp là gắn event listener lặp đi lặp lại mà không gỡ ra, hoặc giữ dữ liệu trong cache bộ nhớ mà không có giới hạn hay cơ chế eviction. Theo thời gian, memory tăng dần và process trở nên thiếu ổn định.

---

### Level 2: Understanding

#### Q_LEVEL2_861: Why is ignoring stream backpressure dangerous?

**Question:**
en: Why is ignoring backpressure in NodeJS streams a production trap?
vi: Vì sao bỏ qua **backpressure** trong stream NodeJS là một bẫy production?

**Answer:**
en: If a producer pushes data faster than the consumer can handle, buffers grow, memory rises, and the process may slow down or crash. Safe stream handling respects `drain`, uses `pipeline`, and avoids uncontrolled buffering.
vi: Nếu bên tạo dữ liệu đẩy nhanh hơn bên nhận có thể xử lý, buffer sẽ phình to, memory tăng và process có thể chậm hoặc crash. Cách an toàn là tôn trọng `drain`, dùng `pipeline` và tránh buffer tăng không kiểm soát.

#### Q_LEVEL2_873: Why is cluster or worker misuse a common scaling mistake?

**Question:**
en: Why is using `cluster` or workers without understanding the workload a common NodeJS mistake?
vi: Vì sao dùng `cluster` hoặc worker mà không hiểu workload lại là lỗi thường gặp trong NodeJS?

**Answer:**
en: Teams sometimes assume more processes always solve performance issues. That can hide the real bottleneck, increase memory usage, complicate coordination, and break assumptions about shared state or sticky sessions. Scaling needs to match the actual CPU, I/O, and state model.
vi: Nhiều team nghĩ cứ tăng process là sẽ giải quyết được vấn đề hiệu năng. Cách đó có thể che giấu nút cổ chai thật sự, làm tăng memory, khiến việc phối hợp phức tạp hơn và phá vỡ giả định về shared state hoặc sticky session. Scale phải khớp với mô hình CPU, I/O và state thực tế.

#### Q_LEVEL2_884: Why are environment and config mistakes so costly?

**Question:**
en: Why do weak environment and configuration practices cause serious NodeJS incidents?
vi: Vì sao thực hành environment và configuration yếu dễ gây incident nghiêm trọng trong NodeJS?

**Answer:**
en: NodeJS apps often rely heavily on environment variables for ports, secrets, URLs, and feature flags. If validation is missing, the app can start with invalid config, connect to the wrong system, or behave differently across environments in ways that are hard to debug.
vi: Ứng dụng NodeJS thường phụ thuộc mạnh vào biến môi trường cho port, secret, URL và feature flag. Nếu không validate cấu hình, app có thể vẫn khởi động với giá trị sai, kết nối nhầm hệ thống hoặc hành xử khác nhau giữa các môi trường theo cách rất khó debug.

#### Q_LEVEL2_896: Why is weak error handling a dangerous anti-pattern?

**Question:**
en: Why is weak error handling a dangerous anti-pattern in NodeJS services?
vi: Vì sao xử lý lỗi yếu là một anti-pattern nguy hiểm trong service NodeJS?

**Answer:**
en: Swallowing errors, returning vague `500` responses, or logging without context makes incidents longer and riskier. Good error handling separates operational errors from programmer bugs, adds context, and fails in controlled ways.
vi: Nuốt lỗi, trả về `500` mơ hồ hoặc log mà không có context sẽ khiến incident kéo dài và rủi ro hơn. Xử lý lỗi tốt phải tách operational error khỏi bug lập trình, bổ sung context và fail theo cách có kiểm soát.

#### Q_LEVEL2_908: Why is missing graceful shutdown a real production trap?

**Question:**
en: Why is not implementing graceful shutdown a serious production pitfall for NodeJS services?
vi: Vì sao không triển khai **graceful shutdown** là một pitfall nghiêm trọng cho service NodeJS?

**Answer:**
en: In containers or orchestrated environments, services are terminated regularly during deploys or scaling. If the process exits immediately, requests may be dropped, background work may stop halfway, and open connections may be left in bad states. Graceful shutdown gives the app time to stop accepting traffic and clean up resources safely.
vi: Trong môi trường container hoặc orchestrator, service thường xuyên bị dừng khi deploy hoặc scale. Nếu process thoát ngay, request có thể bị rơi, background work bị dừng giữa chừng và connection còn mở bị để lại ở trạng thái xấu. **Graceful shutdown** cho app thời gian ngừng nhận traffic và dọn tài nguyên an toàn.

#### Q_LEVEL2_921: Why are dependency and observability gaps dangerous in NodeJS?

**Question:**
en: Why are supply-chain risk and weak observability often paired NodeJS pitfalls?
vi: Vì sao rủi ro **dependency supply chain** và observability yếu thường đi cùng nhau như một pitfall trong NodeJS?

**Answer:**
en: NodeJS services often depend on many packages, and failures from vulnerable or broken dependencies can spread quickly. Without structured logs, metrics, traces, and dependency visibility, teams detect the problem late and struggle to isolate which package, release, or code path caused it.
vi: Service NodeJS thường phụ thuộc vào rất nhiều package, nên lỗi từ dependency có lỗ hổng hoặc hỏng hóc có thể lan rất nhanh. Nếu thiếu log có cấu trúc, metric, trace và khả năng nhìn vào dependency, team sẽ phát hiện muộn và rất khó xác định package, bản phát hành hay code path nào là nguyên nhân.

---

### Level 3: Applying

#### Q_LEVEL3_943: Apply a safer startup configuration check

**Question:**
en: How would you apply a safer startup pattern to prevent invalid environment configuration from reaching production behavior?
vi: Bạn sẽ áp dụng pattern khởi động an toàn hơn như thế nào để ngăn cấu hình môi trường sai đi vào hành vi production?

**Answer:**
en: Fail fast at startup by validating required settings, expected formats, and allowed ranges before the app begins handling traffic. The mistake is letting the service boot with bad config and discovering it only after requests fail.
vi: Hãy **fail fast** ngay lúc khởi động bằng cách validate các cấu hình bắt buộc, định dạng mong đợi và khoảng giá trị hợp lệ trước khi app bắt đầu nhận traffic. Lỗi thường gặp là để service vẫn chạy với config sai rồi chỉ phát hiện khi request bắt đầu lỗi.

```csharp
public sealed class AppConfig
{
    public required string ConnectionString { get; init; }
    public required string ApiKey { get; init; }
    public int Port { get; init; }
}

public static class StartupValidation
{
    public static AppConfig Validate(IConfiguration configuration)
    {
        var config = configuration.Get<AppConfig>()
            ?? throw new InvalidOperationException("Configuration is missing.");

        if (string.IsNullOrWhiteSpace(config.ConnectionString))
        {
            throw new InvalidOperationException("ConnectionString is required.");
        }

        if (string.IsNullOrWhiteSpace(config.ApiKey))
        {
            throw new InvalidOperationException("ApiKey is required.");
        }

        if (config.Port is < 1 or > 65535)
        {
            throw new InvalidOperationException("Port must be between 1 and 65535.");
        }

        return config;
    }
}
```

#### Q_LEVEL3_957: Apply resource cleanup to avoid leaks

**Question:**
en: How would you apply a safer pattern to avoid connection or resource leaks in a long-running backend service?
vi: Bạn sẽ áp dụng pattern an toàn hơn như thế nào để tránh leak connection hoặc tài nguyên trong service chạy lâu dài?

**Answer:**
en: Treat cleanup as part of the normal control flow, not as an afterthought. The safer pattern is to scope resources tightly, dispose them deterministically, and make cancellation and shutdown paths release the same resources as success paths.
vi: Hãy xem cleanup là một phần của luồng xử lý bình thường chứ không phải việc thêm vào sau cùng. Pattern an toàn hơn là giới hạn vòng đời tài nguyên rõ ràng, dispose theo cách xác định được và bảo đảm đường đi cancellation hoặc shutdown cũng giải phóng cùng tài nguyên như đường đi thành công.

```csharp
public sealed class ReportExporter
{
    private readonly DbConnection _connection;

    public ReportExporter(DbConnection connection)
    {
        _connection = connection;
    }

    public async Task ExportAsync(Stream destination, CancellationToken cancellationToken)
    {
        await _connection.OpenAsync(cancellationToken);

        await using var command = _connection.CreateCommand();
        command.CommandText = "select Id, Name from Reports";

        await using var reader = await command.ExecuteReaderAsync(cancellationToken);
        await using var writer = new StreamWriter(destination, leaveOpen: true);

        while (await reader.ReadAsync(cancellationToken))
        {
            await writer.WriteLineAsync($"{reader.GetInt32(0)},{reader.GetString(1)}");
        }

        await writer.FlushAsync(cancellationToken);
    }
}
```

#### Q_LEVEL3_972: Apply graceful shutdown and better failure visibility

**Question:**
en: How would you apply graceful shutdown and failure visibility to reduce dropped work and hidden errors in production?
vi: Bạn sẽ áp dụng graceful shutdown và khả năng nhìn thấy lỗi như thế nào để giảm công việc bị mất và lỗi bị che giấu trong production?

**Answer:**
en: Stop accepting new work, cancel background tasks with a deadline, flush logs or telemetry, and close connections deliberately. The key idea is that shutdown behavior must be designed, tested, and observable instead of being treated as an OS detail.
vi: Hãy ngừng nhận việc mới, hủy background task với deadline, flush log hoặc telemetry và đóng connection có chủ đích. Ý chính là hành vi shutdown phải được thiết kế, kiểm thử và quan sát được, thay vì xem nó chỉ là chi tiết của hệ điều hành.

```csharp
public sealed class Worker : BackgroundService
{
    private readonly ILogger<Worker> _logger;

    public Worker(ILogger<Worker> logger)
    {
        _logger = logger;
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        while (!stoppingToken.IsCancellationRequested)
        {
            _logger.LogInformation("Processing background work item.");
            await Task.Delay(TimeSpan.FromSeconds(5), stoppingToken);
        }
    }

    public override async Task StopAsync(CancellationToken cancellationToken)
    {
        _logger.LogInformation("Graceful shutdown started.");
        await base.StopAsync(cancellationToken);
        _logger.LogInformation("Graceful shutdown completed.");
    }
}
```
