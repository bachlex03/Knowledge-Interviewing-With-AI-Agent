# Node.js Advance Q&A

### Level 4: Analyzing

#### Q1: Analyze the performance implications of CPU-bound tasks in a Node.js application and compare potential solutions.
**Question:**
en: Analyze the performance implications of CPU-bound tasks in a Node.js application and compare potential solutions.
vi: Phân tích các tác động về hiệu suất của các tác vụ nặng về CPU (CPU-bound) trong một ứng dụng Node.js và so sánh các giải pháp tiềm năng.

**Answer:**
en: Node.js uses a single main thread. CPU-bound tasks block the Event Loop, causing all incoming requests to wait, severely degrading application responsiveness. Solutions include: 1) `Worker Threads` for multi-threading within Node.js, 2) `child_process.fork()` to spawn separate processes, or 3) Offloading the task to external microservices written in more suitable languages (e.g., Rust, Go).
vi: Node.js sử dụng một luồng chính duy nhất. Các tác vụ nặng về CPU sẽ chặn Event Loop, khiến tất cả các yêu cầu đến phải chờ đợi, làm giảm nghiêm trọng tốc độ phản hồi của ứng dụng. Các giải pháp bao gồm: 1) `Worker Threads` cho đa luồng bên trong Node.js, 2) `child_process.fork()` để tạo ra các tiến trình riêng biệt, hoặc 3) Chuyển giao tác vụ cho các microservices bên ngoài viết bằng các ngôn ngữ phù hợp hơn (ví dụ: Rust, Go).

#### Q2: Compare the differences between `process.nextTick()` and `setImmediate()`.
**Question:**
en: Compare the differences between `process.nextTick()` and `setImmediate()` in the context of the Event Loop phases.
vi: So sánh sự khác biệt giữa `process.nextTick()` và `setImmediate()` trong bối cảnh các giai đoạn của Event Loop.

**Answer:**
en: `process.nextTick()` is not part of the Event Loop; its callback is executed immediately after the current operation completes, before the Event Loop continues. `setImmediate()` is part of the Event Loop and its callback is executed in the "check" phase, immediately after the "poll" phase completes.
vi: `process.nextTick()` không thuộc về Event Loop; callback của nó được thực thi ngay sau khi hoạt động hiện tại hoàn thành, trước khi Event Loop tiếp tục. `setImmediate()` là một phần của Event Loop và callback của nó được thực thi trong giai đoạn "check", ngay sau khi giai đoạn "poll" kết thúc.
```javascript
console.log('Start');
setImmediate(() => console.log('Immediate'));
process.nextTick(() => console.log('NextTick'));
console.log('End');
// Output: Start, End, NextTick, Immediate
```

#### Q3: Analyze a memory leak scenario.
**Question:**
en: Analyze a scenario where a Node.js application experiences memory leaks. What tools and techniques would you use to identify the cause?
vi: Phân tích một kịch bản trong đó một ứng dụng Node.js gặp phải tình trạng rò rỉ bộ nhớ (memory leaks). Bạn sẽ sử dụng những công cụ và kỹ thuật nào để xác định nguyên nhân?

**Answer:**
en: Memory leaks often happen when global variables, closures, or event listeners hold onto memory that is no longer needed, preventing garbage collection. I would use the `--inspect` flag to run Node.js, then use Chrome DevTools or tools like `heapdump` and `clinic.js` to take heap snapshots. By comparing snapshots over time, I can analyze which objects are growing and failing to be garbage collected.
vi: Rò rỉ bộ nhớ thường xảy ra khi các biến toàn cục (global variables), closures hoặc các event listeners giữ lại bộ nhớ không còn cần thiết, ngăn chặn quá trình thu gom rác (garbage collection). Tôi sẽ sử dụng cờ `--inspect` để chạy Node.js, sau đó sử dụng Chrome DevTools hoặc các công cụ như `heapdump` và `clinic.js` để chụp lại heap (heap snapshots). Bằng cách so sánh các bản chụp qua thời gian, tôi có thể phân tích xem đối tượng nào đang tăng lên và không được thu gom rác.

#### Q4: Contrast horizontal and vertical scaling strategies for Node.js.
**Question:**
en: Contrast horizontal and vertical scaling strategies for Node.js applications.
vi: Sự tương phản (so sánh) giữa các chiến lược mở rộng theo chiều ngang (horizontal) và chiều dọc (vertical) cho các ứng dụng Node.js.

**Answer:**
en: Vertical scaling involves adding more power (CPU, RAM) to a single machine. It's limited by hardware limits and doesn't fully utilize multi-core systems natively unless using the `cluster` module. Horizontal scaling involves adding more machines or instances, often managed via Docker/Kubernetes and Load Balancers. Horizontal scaling is generally preferred for Node.js to easily distribute load and ensure high availability.
vi: Mở rộng theo chiều dọc liên quan đến việc thêm sức mạnh (CPU, RAM) vào một máy duy nhất. Nó bị giới hạn bởi phần cứng và không tận dụng hết hệ thống đa lõi nguyên bản trừ khi sử dụng module `cluster`. Mở rộng theo chiều ngang liên quan đến việc thêm nhiều máy hoặc instances, thường được quản lý qua Docker/Kubernetes và Load Balancers. Mở rộng theo chiều ngang thường được ưu tiên cho Node.js để dễ dàng phân phối tải và đảm bảo tính sẵn sàng cao.

#### Q5: Examine the security risks associated with the `eval()` function.
**Question:**
en: Examine the security risks associated with the `eval()` function and arbitrary code execution in Node.js.
vi: Xem xét các rủi ro bảo mật liên quan đến hàm `eval()` và việc thực thi mã tùy ý trong Node.js.

**Answer:**
en: `eval()` executes any string passed to it as JavaScript code. If user input is passed into `eval()` without strict sanitization, attackers can execute arbitrary code (Remote Code Execution - RCE), gaining access to the server's file system, environment variables, or database. It bypasses conventional security checks and should generally never be used in production applications.
vi: `eval()` thực thi bất kỳ chuỗi nào được truyền vào nó dưới dạng mã JavaScript. Nếu dữ liệu đầu vào của người dùng được truyền vào `eval()` mà không có sự kiểm tra kỹ lưỡng (sanitization), những kẻ tấn công có thể thực thi mã tùy ý (Remote Code Execution - RCE), giành quyền truy cập vào hệ thống tệp của máy chủ, các biến môi trường hoặc cơ sở dữ liệu. Nó bỏ qua các kiểm tra bảo mật thông thường và nói chung không bao giờ nên được sử dụng trong các ứng dụng production.

---

### Level 5: Evaluating

#### Q1: Evaluate the trade-offs between monolithic architecture versus microservices.
**Question:**
en: Evaluate the trade-offs between using a monolithic architecture versus microservices for a large-scale Node.js enterprise application.
vi: Đánh giá sự đánh đổi giữa việc sử dụng kiến trúc nguyên khối (monolithic) so với microservices cho một ứng dụng doanh nghiệp Node.js quy mô lớn.

**Answer:**
en: Monoliths are simpler to develop, deploy, and test initially but become hard to maintain, scale, and start-up as they grow. Microservices allow independent scaling, language flexibility (mixing Node.js with others), and faster deployments, but introduce network latency, complex debugging, and the challenge of distributed data consistency. The choice depends on team size, domain complexity, and scaling needs.
vi: Các ứng dụng monolithic dễ phát triển, triển khai và kiểm thử ban đầu nhưng trở nên khó bảo trì, mở rộng và khởi động khi chúng lớn lên. Microservices cho phép mở rộng độc lập, linh hoạt ngôn ngữ (kết hợp Node.js với ngôn ngữ khác) và triển khai nhanh hơn, nhưng lại gây ra độ trễ mạng, gỡ lỗi phức tạp và thách thức về tính nhất quán của dữ liệu phân tán. Sự lựa chọn phụ thuộc vào quy mô nhóm, độ phức tạp của nghiệp vụ và nhu cầu mở rộng.

#### Q2: Judge the effectiveness of using TypeScript over plain JavaScript.
**Question:**
en: Judge the effectiveness of using TypeScript over plain JavaScript in a large-team Node.js project.
vi: Đánh giá tính hiệu quả của việc sử dụng TypeScript so với JavaScript thuần trong một dự án Node.js có đội ngũ lớn.

**Answer:**
en: TypeScript is highly effective in large teams. It introduces static typing, which acts as built-in documentation and catches a significant percentage of bugs at compile-time before they reach production. While it has a learning curve and requires a build step, the long-term benefits of refactoring confidence, better IDE support, and maintainability far outweigh the initial overhead.
vi: TypeScript cực kỳ hiệu quả trong các nhóm lớn. Nó giới thiệu kiểu tĩnh (static typing), đóng vai trò như một tài liệu tích hợp sẵn và bắt được một tỷ lệ lớn các lỗi tại thời điểm biên dịch (compile-time) trước khi chúng được đưa lên production. Mặc dù nó đòi hỏi thời gian học tập và yêu cầu bước build (biên dịch), những lợi ích dài hạn của sự tự tin khi refactor, hỗ trợ IDE tốt hơn và khả năng bảo trì lớn hơn nhiều so với chi phí thiết lập ban đầu.

#### Q3: Defend the choice of using a message broker in a distributed Node.js system.
**Question:**
en: Defend the choice of using a message broker (like RabbitMQ or Kafka) in a distributed Node.js system.
vi: Bảo vệ sự lựa chọn sử dụng một message broker (như RabbitMQ hoặc Kafka) trong một hệ thống Node.js phân tán.

**Answer:**
en: In a distributed system, synchronous HTTP calls between services can lead to cascading failures if one service goes down. A message broker decouples services by allowing them to communicate asynchronously via events. It provides buffering, retry mechanisms, and ensures message delivery (guaranteed delivery), significantly increasing the fault tolerance and scalability of the entire architecture.
vi: Trong một hệ thống phân tán, các lệnh gọi HTTP đồng bộ giữa các dịch vụ có thể dẫn đến lỗi dây chuyền nếu một dịch vụ ngừng hoạt động. Một message broker giúp phân tách (decouple) các dịch vụ bằng cách cho phép chúng giao tiếp bất đồng bộ qua các sự kiện. Nó cung cấp khả năng đệm (buffering), cơ chế thử lại (retry) và đảm bảo gửi tin nhắn (guaranteed delivery), làm tăng đáng kể khả năng chịu lỗi và khả năng mở rộng của toàn bộ kiến trúc.

#### Q4: Appraise the use of Serverless functions compared to a traditional Node.js server.
**Question:**
en: Appraise the use of Serverless functions (e.g., AWS Lambda) for running Node.js workloads compared to a traditional always-on Node.js server.
vi: Đánh giá việc sử dụng các Serverless functions (ví dụ: AWS Lambda) để chạy khối lượng công việc Node.js so với một máy chủ Node.js luôn bật truyền thống.

**Answer:**
en: Serverless is excellent for variable or sporadic workloads; it scales to zero, meaning you only pay for compute time used, and it eliminates server maintenance overhead. However, it suffers from "cold starts" (initial latency), making it less ideal for applications requiring consistently low latency. Traditional servers (like Express on EC2/Docker) provide consistent performance and are more cost-effective for continuous, high-volume traffic.
vi: Serverless rất tuyệt vời cho các khối lượng công việc biến đổi hoặc rời rạc; nó có thể scale xuống 0, nghĩa là bạn chỉ trả tiền cho thời gian tính toán được sử dụng và nó loại bỏ chi phí bảo trì máy chủ. Tuy nhiên, nó bị ảnh hưởng bởi "cold starts" (độ trễ ban đầu khởi động), khiến nó ít lý tưởng hơn đối với các ứng dụng yêu cầu độ trễ thấp liên tục. Các máy chủ truyền thống (như Express trên EC2/Docker) cung cấp hiệu suất ổn định và tiết kiệm chi phí hơn cho lưu lượng truy cập liên tục, khối lượng lớn.

#### Q5: Critique the statement: "Node.js is not suitable for high-performance applications because it is single-threaded."
**Question:**
en: Critique the statement: "Node.js is not suitable for high-performance applications because it is single-threaded."
vi: Phê bình nhận định: "Node.js không phù hợp cho các ứng dụng hiệu suất cao vì nó đơn luồng (single-threaded)."

**Answer:**
en: This statement is partially true but largely misleading. Node.js is indeed single-threaded for its main execution environment, making it unsuitable for heavy CPU-bound tasks natively. However, it is exceptionally high-performing for I/O-bound tasks (like web servers, API gateways, and real-time chat) due to its non-blocking architecture. Moreover, modern Node.js includes "Worker Threads" and native C++ add-ons, allowing it to efficiently handle CPU-heavy tasks when properly engineered.
vi: Nhận định này đúng một phần nhưng phần lớn dễ gây hiểu lầm. Node.js thực sự đơn luồng trong môi trường thực thi chính, khiến nó không phù hợp cho các tác vụ nặng về CPU một cách nguyên bản. Tuy nhiên, nó đạt hiệu suất cực cao đối với các tác vụ I/O (như máy chủ web, API gateways và chat thời gian thực) nhờ kiến trúc không chặn (non-blocking). Hơn nữa, Node.js hiện đại bao gồm "Worker Threads" và các add-on C++ nguyên bản, cho phép nó xử lý hiệu quả các tác vụ nặng về CPU khi được thiết kế đúng cách.