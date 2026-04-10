# NestJS Advance Q&A

### Level 4: Analyzing

#### Q_LEVEL4_142: Analyze circular dependency problems.

**Question:**
en: Analyze why circular dependencies happen in NestJS modules or providers.
vi: Phân tích vì sao **circular dependency** xảy ra trong module hoặc provider NestJS.

**Answer:**
en: Circular dependencies happen when two modules or providers depend on each other directly. This often reveals unclear boundaries or mixed responsibilities. `forwardRef()` can unblock the application, but the better long-term solution is usually to extract shared logic into a separate provider or redesign the module boundary.
vi: **Vấn đề:** **Circular dependency** xảy ra khi hai module hoặc provider phụ thuộc trực tiếp vào nhau. Điều này thường cho thấy boundary chưa rõ hoặc trách nhiệm bị trộn lẫn. **Giải pháp:** `forwardRef()` có thể gỡ kẹt tạm thời, nhưng về lâu dài nên tách logic chung ra provider riêng hoặc thiết kế lại ranh giới module.

#### Q_LEVEL4_253: Analyze provider scope performance.

**Question:**
en: Analyze the performance impact of request-scoped providers in NestJS.
vi: Phân tích tác động hiệu năng của **request-scoped provider** trong NestJS.

**Answer:**
en: Request-scoped providers create a new instance per request, which increases allocation and dependency resolution cost. They are useful for request-specific context but should not be used by default for high-throughput services.
vi: **Vấn đề:** **Request-scoped provider** tạo instance mới cho mỗi request, làm tăng chi phí cấp phát và resolve dependency. **Giải pháp:** Chỉ dùng khi thật sự cần context theo request; với service throughput cao, ưu tiên **singleton provider** và truyền context rõ ràng.

```csharp
public class RequestContext
{
    public string CorrelationId { get; init; } = "";
}

public class OrderService
{
    // Prefer passing request-specific context explicitly
    // instead of making the whole service request-scoped.
    public void CreateOrder(RequestContext context)
    {
        Console.WriteLine($"Trace: {context.CorrelationId}");
    }
}
```

#### Q_LEVEL4_364: Analyze guard pipe interceptor order.

**Question:**
en: Analyze the execution order of guards, pipes, interceptors, and filters in NestJS.
vi: Phân tích thứ tự chạy của **guards**, **pipes**, **interceptors** và **filters** trong NestJS.

**Answer:**
en: Guards run before a request reaches the handler to decide access. Pipes transform and validate parameters before handler execution. Interceptors wrap handler execution and can transform results. Filters catch exceptions after errors are thrown. Understanding this order prevents placing validation, authorization, or response logic in the wrong layer.
vi: **Guard** chạy trước để quyết định quyền truy cập. **Pipe** biến đổi và validate parameter trước khi handler chạy. **Interceptor** bọc quanh handler và có thể biến đổi kết quả. **Filter** bắt exception sau khi lỗi được throw. Hiểu thứ tự này giúp không đặt authorization, validation hoặc response mapping sai tầng.

#### Q_LEVEL4_475: Analyze monolith module boundaries.

**Question:**
en: Analyze how to design module boundaries in a large NestJS monolith.
vi: Phân tích cách thiết kế ranh giới module trong một NestJS monolith lớn.

**Answer:**
en: Good module boundaries follow business capabilities rather than technical layers alone. A large NestJS monolith should avoid one giant shared module and instead expose deliberate exports, keep internal providers private, and use clear contracts between features.
vi: **Vấn đề:** Monolith lớn dễ biến thành một khối phụ thuộc chéo nếu module boundary yếu. **Giải pháp:** Chia module theo năng lực nghiệp vụ, export có chủ đích, giữ provider nội bộ private và dùng contract rõ giữa các feature.

#### Q_LEVEL4_586: Analyze validation and security risk.

**Question:**
en: Analyze the security risks of weak DTO validation in NestJS APIs.
vi: Phân tích rủi ro bảo mật khi DTO validation yếu trong API NestJS.

**Answer:**
en: Weak validation can allow unexpected fields, invalid types, injection payloads, and mass assignment bugs. In production, `ValidationPipe` should often use options such as whitelist, forbidNonWhitelisted, and transform carefully with explicit DTO rules.
vi: **Vấn đề:** Validation yếu có thể cho phép field lạ, sai kiểu dữ liệu, payload injection hoặc lỗi **mass assignment**. **Giải pháp:** Trong production nên cấu hình `ValidationPipe` với `whitelist`, `forbidNonWhitelisted` và `transform` một cách cẩn trọng, kèm DTO rule rõ ràng.

---

### Level 5: Evaluating

#### Q_LEVEL5_697: Evaluate NestJS for enterprise backend.

**Question:**
en: Evaluate whether NestJS is a good choice for an enterprise backend.
vi: Đánh giá NestJS có phải lựa chọn tốt cho backend doanh nghiệp hay không.

**Answer:**
en: NestJS is strong for enterprise teams because it provides structure, dependency injection, testing support, modular architecture, and TypeScript-first development. The trade-off is more framework complexity than minimal libraries like Express. It is usually a good fit when maintainability and team consistency matter.
vi: NestJS mạnh trong môi trường doanh nghiệp vì có cấu trúc rõ, **dependency injection**, hỗ trợ test, kiến trúc module và ưu tiên **TypeScript**. Đánh đổi là framework phức tạp hơn thư viện tối giản như Express. Nó thường phù hợp khi bảo trì dài hạn và sự nhất quán trong team là ưu tiên.

#### Q_LEVEL5_708: Evaluate NestJS versus Express.

**Question:**
en: Evaluate when to choose NestJS over Express.
vi: Đánh giá khi nào nên chọn NestJS thay vì Express.

**Answer:**
en: Choose NestJS when the team needs opinionated structure, dependency injection, modules, decorators, and consistent testing patterns. Choose Express when the service is small, the team wants minimal abstraction, or the architecture is already enforced elsewhere.
vi: Nên chọn NestJS khi team cần cấu trúc có định hướng, **dependency injection**, module, decorator và pattern test nhất quán. Nên chọn Express khi service nhỏ, team muốn abstraction tối thiểu hoặc kiến trúc đã được kiểm soát ở nơi khác.

#### Q_LEVEL5_819: Defend global validation configuration.

**Question:**
en: Defend using global validation configuration in a production NestJS API.
vi: Bảo vệ việc dùng cấu hình validation global trong API NestJS production.

**Answer:**
en: Global validation reduces duplicated validation setup, ensures consistent request hygiene, and prevents controllers from receiving unexpected input. It should still be combined with explicit DTOs and careful exception formatting to avoid leaking sensitive details.
vi: Validation global giảm lặp cấu hình, đảm bảo request được kiểm tra nhất quán và tránh controller nhận input bất ngờ. Tuy vậy vẫn cần DTO rõ ràng và format exception cẩn thận để không lộ thông tin nhạy cảm.

```csharp
public static class GlobalValidationPolicy
{
    public static void ValidateNoUnexpectedFields(
        IDictionary<string, object> input,
        HashSet<string> allowedFields)
    {
        foreach (var field in input.Keys)
        {
            if (!allowedFields.Contains(field))
                throw new ArgumentException($"Unexpected field: {field}");
        }
    }
}
```

#### Q_LEVEL5_930: Critique overusing decorators.

**Question:**
en: Critique the overuse of decorators and framework magic in NestJS.
vi: Phê bình việc lạm dụng **decorator** và framework magic trong NestJS.

**Answer:**
en: Decorators make NestJS concise, but overusing them can hide control flow and make debugging harder. A healthy codebase keeps decorators for framework integration and leaves business logic in explicit services, plain functions, and well-tested classes.
vi: **Decorator** giúp NestJS gọn hơn, nhưng lạm dụng có thể che giấu luồng xử lý và làm debug khó hơn. Codebase tốt nên dùng decorator cho phần tích hợp framework, còn business logic nên nằm trong service, function rõ ràng và class được test kỹ.

#### Q_LEVEL5_141: Evaluate microservices support.

**Question:**
en: Evaluate NestJS microservices support for a distributed system.
vi: Đánh giá hỗ trợ **microservices** của NestJS cho hệ thống phân tán.

**Answer:**
en: NestJS provides a consistent programming model for transports such as TCP, Redis, RabbitMQ, Kafka, and gRPC, which helps teams reuse patterns. However, distributed systems still require careful handling of retries, idempotency, tracing, schema evolution, and data consistency. NestJS helps structure the code but does not remove distributed-system complexity.
vi: NestJS cung cấp mô hình lập trình nhất quán cho các transport như TCP, Redis, RabbitMQ, Kafka và gRPC, giúp team tái sử dụng pattern. Tuy nhiên hệ thống phân tán vẫn cần xử lý retry, **idempotency**, tracing, schema evolution và data consistency. NestJS giúp tổ chức code, nhưng không làm biến mất độ phức tạp của distributed system.
