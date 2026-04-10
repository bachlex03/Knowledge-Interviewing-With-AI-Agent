# NestJS Foundation Q&A

### Level 1: Remembering

#### Q_LEVEL1_102: What is NestJS?

**Question:**
en: What is NestJS?
vi: NestJS là gì?

**Answer:**
en: NestJS is a progressive Node.js framework for building scalable server-side applications. It uses TypeScript by default and organizes code around modules, controllers, providers, and dependency injection.
vi: NestJS là một **Node.js framework** hiện đại dùng để xây dựng ứng dụng server-side có khả năng mở rộng. Nó dùng **TypeScript** mặc định và tổ chức code quanh **modules**, **controllers**, **providers** và **dependency injection**.

#### Q_LEVEL1_217: What is a module in NestJS?

**Question:**
en: What is a module in NestJS?
vi: **Module** trong NestJS là gì?

**Answer:**
en: A module is a class decorated with `@Module()` that groups related controllers, providers, imports, and exports into a feature boundary.
vi: **Module** là một class được gắn decorator `@Module()`, dùng để gom **controllers**, **providers**, **imports** và **exports** liên quan vào cùng một ranh giới tính năng.

#### Q_LEVEL1_331: What is a controller in NestJS?

**Question:**
en: What is a controller in NestJS?
vi: **Controller** trong NestJS là gì?

**Answer:**
en: A controller handles incoming HTTP requests and returns responses to clients. It usually delegates business logic to providers or services.
vi: **Controller** xử lý HTTP request đầu vào và trả response cho client. Thông thường nó ủy quyền logic nghiệp vụ cho **provider** hoặc **service**.

#### Q_LEVEL1_446: What is a provider in NestJS?

**Question:**
en: What is a provider in NestJS?
vi: **Provider** trong NestJS là gì?

**Answer:**
en: A provider is a class managed by the NestJS dependency injection container. Services, repositories, factories, and helpers can all be providers.
vi: **Provider** là class được quản lý bởi **dependency injection container** của NestJS. **Service**, repository, factory và helper đều có thể là provider.

#### Q_LEVEL1_558: What is dependency injection?

**Question:**
en: What is dependency injection in NestJS?
vi: **Dependency injection** trong NestJS là gì?

**Answer:**
en: Dependency injection is a pattern where NestJS creates and supplies dependencies to classes instead of each class manually creating them.
vi: **Dependency injection** là pattern trong đó NestJS tạo và cung cấp dependency cho class, thay vì mỗi class tự khởi tạo dependency của nó.

#### Q_LEVEL1_664: What is Injectable?

**Question:**
en: What does the `@Injectable()` decorator mean?
vi: Decorator `@Injectable()` có ý nghĩa gì?

**Answer:**
en: `@Injectable()` marks a class as a provider that can be managed and injected by the NestJS dependency injection system.
vi: `@Injectable()` đánh dấu một class là **provider** có thể được NestJS quản lý và inject thông qua hệ thống **dependency injection**.

#### Q_LEVEL1_775: What is a decorator in NestJS?

**Question:**
en: What is a decorator in NestJS?
vi: **Decorator** trong NestJS là gì?

**Answer:**
en: A decorator is metadata attached to a class, method, parameter, or property. NestJS uses decorators to define routing, modules, injection, validation, and request handling behavior.
vi: **Decorator** là metadata được gắn vào class, method, parameter hoặc property. NestJS dùng decorator để định nghĩa routing, module, injection, validation và cách xử lý request.

#### Q_LEVEL1_886: What is a pipe?

**Question:**
en: What is a pipe in NestJS?
vi: **Pipe** trong NestJS là gì?

**Answer:**
en: A pipe transforms or validates input data before it reaches the route handler.
vi: **Pipe** dùng để biến đổi hoặc validate dữ liệu đầu vào trước khi dữ liệu đó đi vào route handler.

#### Q_LEVEL1_991: What is a guard?

**Question:**
en: What is a guard in NestJS?
vi: **Guard** trong NestJS là gì?

**Answer:**
en: A guard decides whether a request is allowed to continue. It is commonly used for authentication and authorization.
vi: **Guard** quyết định request có được phép đi tiếp hay không. Nó thường được dùng cho **authentication** và **authorization**.

#### Q_LEVEL1_123: What is an interceptor?

**Question:**
en: What is an interceptor in NestJS?
vi: **Interceptor** trong NestJS là gì?

**Answer:**
en: An interceptor can run logic before and after a route handler. It is often used for logging, response mapping, caching, and timing.
vi: **Interceptor** có thể chạy logic trước và sau route handler. Nó thường dùng cho logging, biến đổi response, caching và đo thời gian xử lý.

#### Q_LEVEL1_234: What is an exception filter?

**Question:**
en: What is an exception filter in NestJS?
vi: **Exception filter** trong NestJS là gì?

**Answer:**
en: An exception filter catches thrown exceptions and controls how error responses are sent to the client.
vi: **Exception filter** bắt exception được throw ra và kiểm soát cách trả error response về client.

#### Q_LEVEL1_345: What is middleware?

**Question:**
en: What is middleware in NestJS?
vi: **Middleware** trong NestJS là gì?

**Answer:**
en: Middleware is a function or class that runs before the route handler. It can inspect or modify request and response objects.
vi: **Middleware** là function hoặc class chạy trước route handler. Nó có thể kiểm tra hoặc thay đổi request và response object.

#### Q_LEVEL1_456: What is DTO?

**Question:**
en: What is a DTO in NestJS?
vi: **DTO** trong NestJS là gì?

**Answer:**
en: A DTO, or Data Transfer Object, defines the shape of data sent between client and server, often for request validation.
vi: **DTO** là **Data Transfer Object**, định nghĩa cấu trúc dữ liệu truyền giữa client và server, thường dùng cho request validation.

#### Q_LEVEL1_567: What is ValidationPipe?

**Question:**
en: What is `ValidationPipe` used for?
vi: `ValidationPipe` dùng để làm gì?

**Answer:**
en: `ValidationPipe` validates incoming request data against DTO rules, commonly using `class-validator` and `class-transformer`.
vi: `ValidationPipe` validate dữ liệu request dựa trên rule trong DTO, thường kết hợp với `class-validator` và `class-transformer`.

#### Q_LEVEL1_678: What is the main.ts file?

**Question:**
en: What is the purpose of `main.ts` in a NestJS application?
vi: Mục đích của file `main.ts` trong ứng dụng NestJS là gì?

**Answer:**
en: `main.ts` is the application bootstrap file. It creates the Nest application instance, configures global settings, and starts the server.
vi: `main.ts` là file bootstrap của ứng dụng. Nó tạo instance Nest application, cấu hình các thiết lập global và khởi động server.

#### Q_LEVEL1_789: What is AppModule?

**Question:**
en: What is `AppModule` in NestJS?
vi: `AppModule` trong NestJS là gì?

**Answer:**
en: `AppModule` is usually the root module of a NestJS application. It imports feature modules and connects the main application structure.
vi: `AppModule` thường là root module của ứng dụng NestJS. Nó import các feature module và kết nối cấu trúc chính của ứng dụng.

#### Q_LEVEL1_894: What is a custom provider?

**Question:**
en: What is a custom provider in NestJS?
vi: **Custom provider** trong NestJS là gì?

**Answer:**
en: A custom provider defines how a dependency should be created or supplied, using options such as `useClass`, `useValue`, `useFactory`, or `useExisting`.
vi: **Custom provider** định nghĩa cách một dependency được tạo hoặc cung cấp, thông qua các option như `useClass`, `useValue`, `useFactory` hoặc `useExisting`.

#### Q_LEVEL1_905: What is ConfigModule?

**Question:**
en: What is `ConfigModule` commonly used for in NestJS?
vi: `ConfigModule` thường được dùng để làm gì trong NestJS?

**Answer:**
en: `ConfigModule` loads and provides configuration values, such as environment variables, database URLs, secrets, and feature flags.
vi: `ConfigModule` load và cung cấp giá trị cấu hình như biến môi trường, database URL, secret và feature flag.

#### Q_LEVEL1_116: What is a lifecycle hook?

**Question:**
en: What is a lifecycle hook in NestJS?
vi: **Lifecycle hook** trong NestJS là gì?

**Answer:**
en: A lifecycle hook is a method called by NestJS during application startup, shutdown, or module initialization, such as `onModuleInit()`.
vi: **Lifecycle hook** là method được NestJS gọi trong quá trình startup, shutdown hoặc khởi tạo module, ví dụ `onModuleInit()`.

#### Q_LEVEL1_227: What is TestingModule?

**Question:**
en: What is `TestingModule` in NestJS testing?
vi: `TestingModule` trong testing NestJS là gì?

**Answer:**
en: `TestingModule` creates an isolated NestJS module for unit or integration tests, allowing providers and controllers to be compiled with mock dependencies.
vi: `TestingModule` tạo một NestJS module cô lập cho unit test hoặc integration test, cho phép compile provider và controller cùng mock dependency.

---

### Level 2: Understanding

#### Q_LEVEL2_138: Explain the relationship between modules controllers and providers.

**Question:**
en: Explain the relationship between modules, controllers, and providers in NestJS.
vi: Giải thích mối quan hệ giữa **modules**, **controllers** và **providers** trong NestJS.

**Answer:**
en: Modules define feature boundaries, controllers expose request endpoints, and providers contain reusable business logic. A clean NestJS application keeps controllers thin and moves core behavior into providers.
vi: **Module** định nghĩa ranh giới tính năng, **controller** mở endpoint cho request, còn **provider** chứa logic nghiệp vụ tái sử dụng. Một ứng dụng NestJS tốt thường giữ controller mỏng và đưa logic chính vào provider.

#### Q_LEVEL2_249: Explain why NestJS uses dependency injection.

**Question:**
en: Explain why dependency injection is important in NestJS.
vi: Giải thích vì sao **dependency injection** quan trọng trong NestJS.

**Answer:**
en: Dependency injection reduces tight coupling between classes, improves testability, and centralizes object creation. This makes large applications easier to maintain and refactor.
vi: **Dependency injection** giảm coupling giữa các class, cải thiện khả năng test và tập trung việc khởi tạo object. Điều này giúp ứng dụng lớn dễ bảo trì và refactor hơn.

#### Q_LEVEL2_351: Explain why controllers should stay thin.

**Question:**
en: Why should NestJS controllers stay thin?
vi: Vì sao **controller** trong NestJS nên được giữ mỏng?

**Answer:**
en: Controllers should focus on HTTP concerns such as routing, parameters, and responses. Business rules should live in services so they can be reused and tested independently.
vi: **Controller** nên tập trung vào vấn đề HTTP như routing, parameter và response. Business rule nên nằm trong **service** để tái sử dụng và test độc lập.

#### Q_LEVEL2_462: Compare pipe and guard.

**Question:**
en: Compare pipes and guards in NestJS.
vi: So sánh **pipe** và **guard** trong NestJS.

**Answer:**
en: Guards decide whether a request is allowed to proceed, while pipes transform or validate input data. Guards answer "can this request continue?", while pipes answer "is this data valid and usable?"
vi: **Guard** quyết định request có được đi tiếp hay không, còn **pipe** biến đổi hoặc validate dữ liệu đầu vào. **Guard** trả lời "request này có quyền tiếp tục không?", còn **pipe** trả lời "dữ liệu này có hợp lệ và dùng được không?".

#### Q_LEVEL2_573: Compare interceptor and middleware.

**Question:**
en: Compare interceptors and middleware in NestJS.
vi: So sánh **interceptor** và **middleware** trong NestJS.

**Answer:**
en: Middleware runs before route matching logic reaches the handler and works close to Express/Fastify request objects. Interceptors wrap handler execution and can transform results, measure execution time, or handle response streams.
vi: **Middleware** chạy trước khi request đi tới handler và làm việc gần với request object của Express/Fastify. **Interceptor** bao quanh quá trình handler chạy, có thể biến đổi kết quả, đo thời gian hoặc xử lý response stream.

#### Q_LEVEL2_684: Explain exception filter purpose.

**Question:**
en: Explain the purpose of exception filters in NestJS.
vi: Giải thích mục đích của **exception filter** trong NestJS.

**Answer:**
en: Exception filters centralize error response formatting. They help prevent inconsistent error handling across controllers and make APIs easier for clients to consume.
vi: **Exception filter** tập trung hóa cách format error response. Nó giúp tránh xử lý lỗi không nhất quán giữa các controller và làm API dễ dùng hơn cho client.

#### Q_LEVEL2_795: Explain global pipes.

**Question:**
en: Explain what a global pipe is in NestJS.
vi: Giải thích **global pipe** trong NestJS là gì.

**Answer:**
en: A global pipe applies to all matching requests in the application. For example, a global `ValidationPipe` can validate DTOs across all controllers.
vi: **Global pipe** áp dụng cho các request phù hợp trên toàn ứng dụng. Ví dụ, `ValidationPipe` global có thể validate DTO trong tất cả controller.

#### Q_LEVEL2_806: Explain provider scopes.

**Question:**
en: Explain singleton, request, and transient provider scopes in NestJS.
vi: Giải thích các scope **singleton**, **request** và **transient** của provider trong NestJS.

**Answer:**
en: Singleton providers are shared across the application. Request-scoped providers are created per request. Transient providers are created each time they are injected.
vi: **Singleton provider** được dùng chung toàn ứng dụng. **Request-scoped provider** được tạo theo từng request. **Transient provider** được tạo mới mỗi lần được inject.

#### Q_LEVEL2_917: Explain module exports.

**Question:**
en: Explain why a provider must be exported from a module.
vi: Giải thích vì sao một provider cần được export từ module.

**Answer:**
en: A provider must be exported if another module needs to inject it. Without exporting, the provider stays private to the module that declares it.
vi: Một provider cần được **export** nếu module khác muốn inject nó. Nếu không export, provider chỉ private trong module khai báo nó.

#### Q_LEVEL2_128: Explain dynamic modules.

**Question:**
en: Explain dynamic modules in NestJS.
vi: Giải thích **dynamic module** trong NestJS.

**Answer:**
en: A dynamic module is a module whose configuration is built at runtime through a static method such as `forRoot()` or `register()`. It is useful for reusable libraries and configurable infrastructure modules.
vi: **Dynamic module** là module có cấu hình được tạo lúc runtime qua static method như `forRoot()` hoặc `register()`. Nó hữu ích cho thư viện tái sử dụng và module hạ tầng cần cấu hình linh hoạt.

#### Q_LEVEL2_239: Explain request validation workflow.

**Question:**
en: Explain a common request validation workflow in NestJS.
vi: Giải thích workflow validate request phổ biến trong NestJS.

**Answer:**
en: The client sends data, NestJS maps it to a DTO, `ValidationPipe` checks decorators such as `@IsString()`, and the controller only receives valid data or an error response is returned.
vi: Client gửi dữ liệu, NestJS map dữ liệu vào DTO, `ValidationPipe` kiểm tra decorator như `@IsString()`, và controller chỉ nhận dữ liệu hợp lệ hoặc trả error response.

#### Q_LEVEL2_342: Explain why testing is easier with NestJS DI.

**Question:**
en: Why does NestJS dependency injection make testing easier?
vi: Vì sao **dependency injection** của NestJS giúp test dễ hơn?

**Answer:**
en: Dependencies can be replaced with mocks in `TestingModule`. This lets tests focus on one class without calling real databases, HTTP clients, or external services.
vi: Dependency có thể được thay bằng mock trong `TestingModule`. Nhờ đó test tập trung vào một class mà không cần gọi database, HTTP client hoặc service bên ngoài thật.

#### Q_LEVEL2_453: Explain guards for authorization.

**Question:**
en: Explain how guards are used for authorization.
vi: Giải thích cách dùng **guard** cho **authorization**.

**Answer:**
en: A guard checks request context, such as user roles or permissions, before allowing the route handler to run. If the user lacks permission, the guard rejects the request.
vi: **Guard** kiểm tra context của request như role hoặc permission của user trước khi cho route handler chạy. Nếu user không đủ quyền, guard sẽ từ chối request.

#### Q_LEVEL2_564: Explain lifecycle hooks use cases.

**Question:**
en: Explain common use cases for NestJS lifecycle hooks.
vi: Giải thích các use case phổ biến của **lifecycle hook** trong NestJS.

**Answer:**
en: Lifecycle hooks are useful for initializing connections, warming caches, starting consumers, and cleaning resources during shutdown.
vi: **Lifecycle hook** hữu ích để khởi tạo connection, warm cache, chạy consumer và dọn tài nguyên khi ứng dụng shutdown.

#### Q_LEVEL2_675: Explain NestJS platform abstraction.

**Question:**
en: Explain why NestJS can run on Express or Fastify.
vi: Giải thích vì sao NestJS có thể chạy trên Express hoặc Fastify.

**Answer:**
en: NestJS separates its application architecture from the underlying HTTP adapter. Express and Fastify are platform adapters that handle low-level HTTP details.
vi: NestJS tách kiến trúc ứng dụng khỏi HTTP adapter bên dưới. Express và Fastify là **platform adapter** xử lý chi tiết HTTP cấp thấp.

---

### Level 3: Applying

#### Q_LEVEL3_186: Apply dependency injection with a service.

**Question:**
en: Demonstrate the dependency injection idea used by NestJS services.
vi: Minh họa ý tưởng **dependency injection** được dùng trong service NestJS.

**Answer:**
en: A service should depend on an abstraction or injected dependency instead of creating everything itself. This improves testability and separation of concerns.
vi: **Vấn đề:** Nếu service tự tạo dependency, code bị coupling và khó mock khi test. **Giải pháp:** Inject dependency từ bên ngoài, giống cách NestJS inject provider vào constructor.

```csharp
public interface IUserRepository
{
    User? FindById(int id);
}

public class UserService
{
    private readonly IUserRepository _repository;

    public UserService(IUserRepository repository)
    {
        _repository = repository; // Dependency is injected, not manually created.
    }

    public User? GetUser(int id) => _repository.FindById(id);
}
```

#### Q_LEVEL3_297: Apply validation pipe behavior.

**Question:**
en: Demonstrate behavior similar to a NestJS validation pipe.
vi: Minh họa hành vi tương tự **ValidationPipe** trong NestJS.

**Answer:**
en: A validation step checks incoming data before the handler receives it. Invalid data should be rejected early.
vi: **Vấn đề:** Controller nhận dữ liệu sai sẽ làm logic nghiệp vụ phức tạp và dễ lỗi. **Giải pháp:** Validate dữ liệu trước khi handler chạy, giống `ValidationPipe` trong NestJS.

```csharp
public record CreateUserDto(string Email, string Password);

public static class CreateUserValidator
{
    public static void Validate(CreateUserDto dto)
    {
        if (!dto.Email.Contains("@"))
            throw new ArgumentException("Email is invalid.");

        if (dto.Password.Length < 8)
            throw new ArgumentException("Password is too short.");
    }
}
```

#### Q_LEVEL3_308: Apply guard authorization.

**Question:**
en: Demonstrate behavior similar to a NestJS authorization guard.
vi: Minh họa hành vi tương tự **authorization guard** trong NestJS.

**Answer:**
en: A guard checks whether a user has enough permissions before the handler is executed.
vi: **Vấn đề:** Không nên để handler tự kiểm tra quyền ở mọi nơi. **Giải pháp:** Tách logic quyền vào **guard** để kiểm tra trước khi handler chạy.

```csharp
public class RoleGuard
{
    public bool CanActivate(User user, string requiredRole)
    {
        // Similar to a NestJS guard returning true or false.
        return user.Roles.Contains(requiredRole);
    }
}

public record User(string Name, List<string> Roles);
```

#### Q_LEVEL3_419: Apply interceptor timing.

**Question:**
en: Demonstrate behavior similar to a NestJS logging interceptor.
vi: Minh họa hành vi tương tự **logging interceptor** trong NestJS.

**Answer:**
en: An interceptor can run code before and after handler execution, making it useful for measuring latency.
vi: **Vấn đề:** Cần đo thời gian xử lý mà không rải logging vào từng handler. **Giải pháp:** Dùng **interceptor** để bọc handler và ghi log trước/sau khi chạy.

```csharp
using System.Diagnostics;

public static class TimingInterceptor
{
    public static T Execute<T>(Func<T> handler)
    {
        var watch = Stopwatch.StartNew();
        try { return handler(); }
        finally
        {
            watch.Stop();
            Console.WriteLine($"Handler took {watch.ElapsedMilliseconds}ms");
        }
    }
}
```

#### Q_LEVEL3_520: Apply middleware pipeline.

**Question:**
en: Demonstrate middleware pipeline behavior similar to NestJS middleware.
vi: Minh họa hành vi pipeline **middleware** tương tự NestJS.

**Answer:**
en: Middleware runs before the route handler and can call the next step in the pipeline.
vi: **Vấn đề:** Nhiều request cần logging hoặc gắn correlation ID trước khi vào controller. **Giải pháp:** Dùng **middleware** để xử lý sớm rồi gọi bước tiếp theo.

```csharp
public delegate Task RequestDelegate(HttpContext context);

public class LoggingMiddleware
{
    private readonly RequestDelegate _next;

    public LoggingMiddleware(RequestDelegate next)
    {
        _next = next;
    }

    public async Task Invoke(HttpContext context)
    {
        Console.WriteLine($"Request: {context.Path}");
        await _next(context); // Continue to the next middleware or handler.
    }
}

public record HttpContext(string Path);
```
