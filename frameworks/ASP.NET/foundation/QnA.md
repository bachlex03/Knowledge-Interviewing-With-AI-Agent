# ASP.NET Core Foundation Q&A

### Level 1: Remembering

#### Q_LEVEL1_481: What is ASP.NET Core?
**Question:**
en: What is ASP.NET Core?
vi: ASP.NET Core là gì?

**Answer:**
en: ASP.NET Core is a cross-platform, high-performance, open-source framework for building modern, cloud-based, Internet-connected applications.
vi: ASP.NET Core là một framework mã nguồn mở, hiệu năng cao và đa nền tảng (cross-platform) dùng để xây dựng các ứng dụng hiện đại kết nối Internet dựa trên đám mây.

#### Q_LEVEL1_152: What is Kestrel?
**Question:**
en: What is Kestrel in ASP.NET Core?
vi: Kestrel trong ASP.NET Core là gì?

**Answer:**
en: Kestrel is a cross-platform, incredibly lightweight web server specifically built for ASP.NET Core out-of-the-box.
vi: Kestrel là một web server đa nền tảng, siêu nhẹ và siêu nhanh được tích hợp sẵn (out-of-the-box) chạy mặc định cho ASP.NET Core.

#### Q_LEVEL1_983: What is Middleware?
**Question:**
en: What is Middleware?
vi: Middleware là gì?

**Answer:**
en: Middleware refers to software components assembled into the application pipeline to handle requests and responses.
vi: Middleware là các phần mềm trung gian được ráp vào đường ống xử lý (pipeline) của ứng dụng để lắng nghe request và response.

#### Q_LEVEL1_349: What is the Program.cs file?
**Question:**
en: What is the purpose of the `Program.cs` file?
vi: Mục đích của file `Program.cs` là gì?

**Answer:**
en: It serves as the main entry point to configure the dependency injection container, the application request pipeline, and host execution.
vi: Nó là điểm bắt đầu (entry point) khởi động app, dùng để cấu hình Dependency Injection, xây dựng pipeline request và chạy Host.

#### Q_LEVEL1_552: What is Dependency Injection?
**Question:**
en: What is Dependency Injection (DI) in ASP.NET Core?
vi: Dependency Injection (DI) trong ASP.NET Core là gì?

**Answer:**
en: DI is a core software design pattern built natively into ASP.NET Core aiming to achieve Inversion of Control (IoC) between classes and their dependencies.
vi: DI là một design pattern được tích hợp sẵn bắt buộc trong cốt lõi ASP.NET Core, nhằm đạt được Inversion of Control phân ly sự phụ thuộc.

#### Q_LEVEL1_810: What are the DI Lifetimes?
**Question:**
en: What are the three Service Lifetimes in ASP.NET Core DI?
vi: Ba vòng đời dịch vụ (Lifetimes) trong ASP.NET Core DI là gì?

**Answer:**
en: They are Transient (created each time requested), Scoped (created once per client request/connection), and Singleton (created once per application lifetime).
vi: Chúng gồm Transient (tạo mới mỗi lần gọi), Scoped (tạo mới một lần cho mỗi chu kỳ HTTP Request), và Singleton (tạo đúng một lần tồn tại song song tuổi thọ app).

#### Q_LEVEL1_114: What is a Controller?
**Question:**
en: What is an MVC Controller?
vi: MVC Controller là gì?

**Answer:**
en: A Controller is a class that handles incoming HTTP requests, performs business operations, and returns responses back to the client.
vi: Controller là một class chịu trách nhiệm tiếp nhận HTTP request, xử lý logic và trả về response cho client.

#### Q_LEVEL1_496: What is a Minimal API?
**Question:**
en: What is a Minimal API?
vi: Minimal API là gì?

**Answer:**
en: Introduced in .NET 6, Minimal APIs are simplified architectures to create HTTP APIs with low ceremony and no Controller classes needed.
vi: Ra mắt từ .NET 6, Minimal API là cách thiết kế tinh gọn để tạo các API mà không cần rườm rà viết cấu trúc Controller.

#### Q_LEVEL1_621: What is Model Binding?
**Question:**
en: What is Model Binding?
vi: Model Binding là gì?

**Answer:**
en: Model Binding automatically maps data from HTTP requests (query strings, forms, JSON body) to action method parameters.
vi: Thuật toán tự động đọc dữ liệu từ HTTP (query, form, JSON body...) rồi nhét vào biến tham số đầu vào của các Action.

#### Q_LEVEL1_832: What is a Filter?
**Question:**
en: What is a Filter in ASP.NET Core?
vi: Filter trong ASP.NET Core là gì?

**Answer:**
en: Filters allow running code before or after specific stages of the request processing pipeline, such as Action Execution, Exceptions, or Authorization.
vi: Filter cho phép chạy mã chèn vào trước hoặc sau các bước nhất định của pipeline tại controller như lúc xác thực, thực thi Action hoặc khi phát sinh Exception.

#### Q_LEVEL1_319: What is CORS?
**Question:**
en: What is CORS?
vi: CORS là gì?

**Answer:**
en: Cross-Origin Resource Sharing (CORS) is a W3C standard that allows a server to relax the same-origin policy, dictating which external domains can call the API.
vi: CORS là chuẩn thiết lập trình duyệt yêu cầu Server phản hồi để cho phép hoặc chặn các domain ngoại lai khác gọi Ajax (fetch) thẳng vào API.

#### Q_LEVEL1_744: What is appsettings.json?
**Question:**
en: What is `appsettings.json`?
vi: `appsettings.json` dùng để làm gì?

**Answer:**
en: It is the standard JSON configuration file used to store application-level settings like database connection strings and custom parameters.
vi: Đây là tệp cấu hình trung tâm định dạng JSON lưu các biến kỹ thuật như chuỗi kết nối database (connection strings) hay các settings hệ thống.

#### Q_LEVEL1_381: What is an Action?
**Question:**
en: What is an Action Method?
vi: Action Method là gì?

**Answer:**
en: An Action is a public method on a Controller that responds to an HTTP request routed to it.
vi: Action là một phương thức public cấu hình bên trong Controller sẽ thực thi khi có một HTTP request ánh xạ route tới nó.

#### Q_LEVEL1_536: What is Routing?
**Question:**
en: What is Routing?
vi: Routing là gì?

**Answer:**
en: Routing is the process of mapping incoming HTTP request URLs to the executable code (endpoints/controllers).
vi: Routing là cơ chế định tuyến phân tích URL HTTP gửi tới và ánh xạ nó đi tìm đúng hàm (endpoint/controller) để chạy.

#### Q_LEVEL1_198: What are Tag Helpers?
**Question:**
en: What are Tag Helpers?
vi: Tag Helpers là gì?

**Answer:**
en: They allow back-end code to participate in creating and rendering HTML elements within Razor views dynamically.
vi: Cung cấp tính năng để nhúng mã back-end C# vào quá trình biên dịch nhằm sinh ra các thẻ HTML động trên View HTML Razor.

#### Q_LEVEL1_604: What is Razor Pages?
**Question:**
en: What is Razor Pages?
vi: Razor Pages là gì?

**Answer:**
en: A page-focused architectural paradigm in ASP.NET Core that encapsulates both the view template and page logic model in a single folder.
vi: Một mô hình kiến trúc tập trung vào "Trang", gom giao diện HTML (View) và code xử lý C# (PageModel) dính liền với nhau để dễ bảo trì.

#### Q_LEVEL1_792: What is Entity Framework Core?
**Question:**
en: What is Entity Framework Core?
vi: Entity Framework Core là gì?

**Answer:**
en: It is the standard Microsoft lightweight, extensible, cross-platform Object-Relational Mapper (O/RM) for .NET.
vi: Đây là công cụ hệ sinh thái chuẩn của Microsoft cung cấp giải pháp lập trình CSDL bằng đối tượng (O/RM) cực nhanh và nhẹ.

#### Q_LEVEL1_417: What is ViewComponent?
**Question:**
en: What is a ViewComponent?
vi: ViewComponent là gì?

**Answer:**
en: Think of them as partial views with logic attached; they render HTML chunks based on encapsulated independent dependencies.
vi: Có thể hiểu ViewComponent như một View con (partial) được nhúng code Logic hoàn chỉnh bên trong để tự render mà không cần Controller truyền dữ liệu xuống.

#### Q_LEVEL1_265: What is Environment Variable?
**Question:**
en: What is the ASPNETCORE_ENVIRONMENT variable?
vi: Biến môi trường ASPNETCORE_ENVIRONMENT là gì?

**Answer:**
en: It tells ASP.NET Core the runtime environment (Development, Staging, Production) so code can conditionally execute setup features.
vi: Cho dự án biết nó đang chạy ở môi trường nào (Development, Production) thông qua đó C# có thể bật tắt các setting (ví dụ: Log tắt ở Prod, bật ở Dev).

---

### Level 2: Understanding

#### Q_LEVEL2_754: Pipeline processing
**Question:**
en: How does the request processing middleware pipeline work?
vi: Kênh đường ống pipeline xử lý middleware hoạt động ra sao?

**Answer:**
en: HTTP Requests travel sequentially through registered middlewares. Each middleware processes the request, optionally calls `next()`, and processes the response backward as the call stack unwinds.
vi: Request HTTP đi tuần tự qua từng middleware. Từng tầng sẽ chạy lệnh, gọi `next()` nhường quyền cho tầng kế, sau đó khi trả Response lại đi xoắn ngược lại theo chuẩn Return Stack.

#### Q_LEVEL2_840: AddScoped vs AddTransient
**Question:**
en: Explain the fundamental difference between `AddScoped`, `AddTransient`, and `AddSingleton`.
vi: Giải thích điểm khác nhau cốt lõi giữa `AddScoped`, `AddTransient` và `AddSingleton`.

**Answer:**
en: Transient creates a new instance every single injection. Scoped creates one instance shared strictly across a single HTTP request lifecycle. Singleton creates one persistent instance shared globally forever.
vi: Transient khởi tạo object mới bất kể ở đâu gọi (hao tài nguyên). Scoped giữ duy nhất một bản sao trong một chu kỳ sống của HTTP Request. Singleton tạo ra duy nhất một bản độc tôn trường tồn mãi cho đến khi tắt Server.

#### Q_LEVEL2_183: Model Validation
**Question:**
en: How does Model Validation work?
vi: Cơ chế Xác thực dữ liệu Model (Validation) hoạt động ra sao?

**Answer:**
en: It validates incoming data automatically prior to executing the action logic using Data Annotations (like `[Required]`). If failed, `ModelState.IsValid` flips false.
vi: Framework sử dụng tính năng Data Annotations (vd: `[Required]`, `[EmailAddress]`) gán trên class. Khi dữ liệu vào, nó quét và nếu lỗi thì đánh cờ `ModelState.IsValid = false`.

#### Q_LEVEL2_526: Controller vs Minimal API
**Question:**
en: Compare Minimal APIs with MVC Controllers.
vi: So sánh Minimal API và MVC Controller.

**Answer:**
en: Minimal APIs optimize for raw performance and extreme code brevity by defining routes dynamically as lambdas, omitting the heavy inheritance and filters native to MVC Controller architectures.
vi: Minimal API vứt bỏ tối đa các class thừa, viết route bằng lambda, cực nhanh nhẹ và dễ đọc. MVC Controller hỗ trợ cấu trúc phân tầng, kế thừa và Filter chuyên sâu, phù hợp cho các dự án phức tạp.

#### Q_LEVEL2_891: UseRouting vs UseEndpoints
**Question:**
en: Explain the difference between `UseRouting` and `UseEndpoints` logic.
vi: Mạch logic của `UseRouting` khác `UseEndpoints` ở điểm nào?

**Answer:**
en: `UseRouting` parses the request URL to determine the intended endpoint execution target. `UseEndpoints` represents the end of the pipeline that actively executes the previously resolved target.
vi: `UseRouting` sẽ nhận URL, tra danh bạ và quyết định xem URL này mapping vào hàm nào. Sau khi ra quyết định, `UseEndpoints` (ở tận cùng pipeline) sẽ thực hiện kích hoạt để chạy hàm đó.

#### Q_LEVEL2_394: Formulating Content Negotiation.
**Question:**
en: What is Content Negotiation in API formulation?
vi: Khái niệm Content Negotiation (đàm phán cấu trúc) trong API là gì?

**Answer:**
en: It's the process where the API returns different payload formats (e.g. JSON vs XML) based primarily on what the client specifically asks for using the `Accept` HTTP header.
vi: Là tính năng tự nhận dạng header `Accept` của bộ gọi client. Nếu client muốn nhận `application/xml` API tự nhả XML, nếu đòi JSON API tự nhả JSON thay vi hard-code.

#### Q_LEVEL2_114: Synchronous vs Asynchronous Actions
**Question:**
en: Contrast synchronous and asynchronous Controller action methods.
vi: So sánh hành vi gọi Action Synchronous (đồng bộ) so với Asynchronous (bất đồng bộ) trên Controller.

**Answer:**
en: Sync actions block the runtime execution thread completely while waiting for lengthy I/O operations (like database hits). Async actions (`async/await`) surrender the thread to process other server requests until the I/O bottleneck completes.
vi: Sync Action giam lỏng luôn Thread máy chủ lúc truy xuất DB làm app bị tắc nghẽn cục bộ. Async (`async/await`) lập tức thả Thread đi phục vụ người khác trong lúc chờ DB, giúp thông luồng xử lý hàng ngàn request cùng lúc.

#### Q_LEVEL2_602: The User object (ClaimsPrincipal)
**Question:**
en: What is the purpose of the generic `User` object property present in the HttpContext?
vi: Phân tích mục đích của object `User` có mặt mặc định trong HTTP Context?

**Answer:**
en: It holds the `ClaimsPrincipal` describing the actively authenticated user making the HTTP request, containing parsed access claims (roles, permissions, name) decoded from the Authentication middleware schema.
vi: Nó chứa dữ liệu `ClaimsPrincipal` cấu thành định danh của User đang gọi đến. Khi token (như JWT) giải mã thành công, middleware tự bóc cờ (Claims - như roles, tên gốc) ra chèn vào object này để bạn xét duyệt phân quyền.

#### Q_LEVEL2_753: Sessions in Core
**Question:**
en: How is server-side session state handled safely?
vi: Cách cơ chế Session state (phiên bộ nhớ) được thi hành như thế nào?

**Answer:**
en: Sessions are implemented via added middleware and backed by an `IDistributedCache` storing values server-side while relaying an encrypted session-ID via client tracking cookie.
vi: C# quản lý Session thông qua middleware tiêm `IDistributedCache`. Bản chất nó lưu dữ liệu nằm bên Server, rồi gán một Cookie mã hóa (chỉ ghi chuỗi ID) tống sang trình duyệt Client giữ để tham chiếu.

#### Q_LEVEL2_482: Strongly Typed Configurations
**Question:**
en: Describe how the Options Pattern (`IOptions<T>`) binds configuration parameters.
vi: Mô tả Pattern Options (`IOptions<T>`) gói cấu hình hệ thống ra sao?

**Answer:**
en: It binds loosely typed structural fields from `appsettings.json` into strongly typed C# instantiated classes utilizing the framework DI resolving mechanism implicitly.
vi: Nó kết nối cấu trúc rườm rà của `appsettings.json` rồi ráp giá trị bọc thành một cái Class C# định dạng chặt chẽ. Sau đó sử dụng DI để inject bộ Setting này cho bất kỳ đâu cần.

#### Q_LEVEL2_219: Developer Exception Page
**Question:**
en: How does the framework intercept unhandled application exceptions fundamentally?
vi: Làm thế nào Framework Core giải trình chặn bắt các Exception crash hệ thống ngầm?

**Answer:**
en: Utilizing `UseExceptionHandler` or `UseDeveloperExceptionPage`, it isolates the pipeline stack, traps any bubbling fatal exceptions natively halting crashes, and renders a localized friendly status stack trace.
vi: Framework sử dụng ngầm các middleware bắt Exception (như `UseDeveloperExceptionPage`), trùm lên toàn bộ pipeline chặn các lỗi crash xuyên thấu, đồng thời tự xử lý in ra cái bảng Stack Trace đầy màu để Developer quan sát lỗi.

#### Q_LEVEL2_901: Authentication vs Authorization
**Question:**
en: Contrast Identity Authentication from Access Authorization structurally.
vi: Tách bạch rõ vai trò của Authentication (Xác thực) và Authorization (Phân quyền).

**Answer:**
en: Authentication proves WHO the exact calling request user fundamentally is (e.g. verifying JWT tokens). Authorization proves WHAT that specifically authenticated entity is legitimately permitted to do (e.g. evaluating Roles).
vi: Authentication là cửa ải kiểm chứng gốc BẠN LÀ AI (vd soi mã token người dùng). Dù đã vào nhà, Authorization là cửa kiểm chứng BẠN CÓ QUYỀN ĐƯỢC LÀM NHỮNG GÌ (vd không phải admin cấm gọi hành động xóa gốc).

#### Q_LEVEL2_458: ApplicationBuilder vs EndpointRouteBuilder
**Question:**
en: Explain fundamentally the `IApplicationBuilder` opposed to `IEndpointRouteBuilder`.
vi: Phân tích sự kiện `IApplicationBuilder` đối nghịch với `IEndpointRouteBuilder`.

**Answer:**
en: The `IApplicationBuilder` is utilized globally structuring the comprehensive middleware stack. `IEndpointRouteBuilder` defines exact route configurations exclusively targeting precise endpoints processing maps.
vi: Thể thức `IApplicationBuilder` đóng vai trò vĩ mô giúp cấu trúc lắp ghép đường ống Middleware. Ở đầu kia, `IEndpointRouteBuilder` nằm trong vi mô định nghĩa các trạm dừng thiết lập địa chỉ maps cho API.

#### Q_LEVEL2_663: IActionResult
**Question:**
en: What is the benefit of returning `IActionResult` over simple strings or objects?
vi: Cái lợi của việc Controller Return `IActionResult` thay cho string/object chay là gì?

**Answer:**
en: `IActionResult` supports extensive RESTful HTTP representations natively standardizing explicit response structures like `NotFound()`, `BadRequest()`, or status OK with payloads uniformly.
vi: Khai báo `IActionResult` cho phép hàm trả ra những chỉ định HTTP chuẩn REST cực kỳ chuyên nghiệp như trạng thái mã hóa 404 Not Found, 400 Bad Request một cách rõ ràng rành mạch.

#### Q_LEVEL2_137: Startup vs Program
**Question:**
en: Discuss the architectural consolidation of `Startup.cs` moving to `Program.cs`.
vi: Thảo luận việc hợp nhất hoàn toàn `Startup.cs` biến mất dồn vào file `Program.cs` duy nhất từ .NET 6.

**Answer:**
en: Using top-level statements, modern .NET coalesced configuring the services pipeline into the `Program.cs` implicitly establishing an ultra-lightweight entry point mitigating ceremonial class bloat.
vi: Dùng tính năng Top-level statements, .NET đời mới ép chết `Startup.cs` cồng kềnh, quy gọn mọi thiết lập service và pipeline vào điểm vào duy nhất `Program.cs` làm app sạch sẽ tối giản 90% dòng class dư thừa cũ.

---

### Level 3: Applying

#### Q_LEVEL3_854: Register and use custom Middleware
**Question:**
en: Demonstrate defining and executing a customized HTTP middleware in the runtime pipeline.
vi: Triển khai một file Middleware tự viết để can thiệp luồng pipeline.

**Answer:**
en: Create a class capturing the `RequestDelegate`, implement an `InvokeAsync` interceptor to log or manipulate execution, and `Use` it explicitly inside `Program.cs`.
vi: Viết lớp nhận `RequestDelegate`, trỏ hàm `InvokeAsync` can thiệp logic (ví dụ in thông tin Console) trước khi gọi hàm kế `await _next()`. Cuối cùng gọi Extension bằng `app.Use...`

```csharp
public class CustomAuditMiddleware
{
    private readonly RequestDelegate _next;
    public CustomAuditMiddleware(RequestDelegate next) => _next = next;

    public async Task InvokeAsync(HttpContext context)
    {
        Console.WriteLine("Before Request Processing => auditing");
        await _next(context); // Advance pipeline
        Console.WriteLine("After Request Unwinding");
    }
}

// In Program.cs:
// app.UseMiddleware<CustomAuditMiddleware>();
```

#### Q_LEVEL3_207: Create a Minimal API Endpoint
**Question:**
en: Show the syntax creating a typical RESTful Minimal API endpoint.
vi: Viết code cho ra API RESTful siêu tốc sử dụng kỹ thuật Minimal API.

**Answer:**
en: You simply utilize `.MapGet()` extensions on the dynamic runtime `WebApplication` object passing lamda expression actions without demanding distinct controller structuring templates.
vi: Bạn chỉ việc lôi cổ `app` gọi trực tiếp `.MapGet()` ép biểu thức lamda xử lí vào đó là xong, không dùng class Controller nào cả.

```csharp
var builder = WebApplication.CreateBuilder(args);
var app = builder.Build();

app.MapGet("/api/greetings/{name}", (string name) => 
{
    return Results.Ok(new { message = $"Hello, {name}!" });
});

app.Run();
```

#### Q_LEVEL3_942: Inject DbContext to Controller
**Question:**
en: Demonstrate how DI provides an EF `DbContext` into a Controller constructor seamlessly.
vi: Minh họa cách kỹ thuật Tiêm (DI) nhét cái CSDL C# (`DbContext`) vào cấu trúc thiết lập hàm khởi tạo của Controller C#.

**Answer:**
en: Register the exact context defining the driver connection string within setup, then declare the object type statically as a read-only parameter constructor.
vi: Chỉ lệnh cài cắm ở Program, sau đó ghi hẳn đối tượng kết nối EF vào hàm Constructor, Dependency sẽ tự động mò nhét tham chiếu DbContext vào.

```csharp
[ApiController]
[Route("[controller]")]
public class InventoryController : ControllerBase
{
    private readonly ApplicationDbContext _db;
    
    // Auto-injected context by default Core mechanics
    public InventoryController(ApplicationDbContext context)
    {
        _db = context;
    }

    [HttpGet]
    public IActionResult FetchItems() => Ok(_db.Items.ToList());
}
```

#### Q_LEVEL3_613: Writing a custom Action Filter
**Question:**
en: Demonstrate authoring a personalized Action Filter logic processing execution steps explicitly.
vi: Triển khai một Action Filter tùy chỉnh bọc trên hành vi truy cập.

**Answer:**
en: Implementing native `IActionFilter` interfaces exposes `OnActionExecuting` overriding mechanisms halting requests before controller instantiation logic.
vi: Gắn cờ kế thừa interface lõi là `IActionFilter`. Triển khai đè hàm chặn cổng `OnActionExecuting` giúp cấm đoạt logic trước khi gọi ruột Controller.

```csharp
public class RejectEmptyHeaderFilter : IActionFilter
{
    public void OnActionExecuting(ActionExecutingContext context)
    {
        if (!context.HttpContext.Request.Headers.ContainsKey("X-Required-Agent"))
        {
            // Halts pipeline completely assigning block response 
            context.Result = new BadRequestObjectResult("Missing Agent Header!");
        }
    }
    public void OnActionExecuted(ActionExecutedContext context) { }
}

// Use via attribute: [ServiceFilter(typeof(RejectEmptyHeaderFilter))]
```

#### Q_LEVEL3_485: Configuring CORS Protocol
**Question:**
en: Demonstrate declaring explicitly allowed external origins configuring strict CORS definitions smoothly.
vi: Viết mẫu cấu hình định dạng giới nghiêm trình dịch CORS cho phép những domain được cấp quyền gọi App.

**Answer:**
en: Declare CORS policy templates specifying dynamic hosts securely inside services setup natively prior to registering URL routing bindings.
vi: Khai rành mạch hệ quy tắc Policy ấn định mảng Domain tuyệt đối ngay trong lòng Services setup trước khi cài routing.

```csharp
// Program.cs setup constraints
builder.Services.AddCors(options =>
{
    options.AddPolicy("AllowPortalPolicy",
        policy => policy.WithOrigins("https://my-portal.com")
                        .AllowAnyHeader()
                        .AllowAnyMethod());
});

var app = builder.Build();

// Interject to pipeline
app.UseCors("AllowPortalPolicy");
```
