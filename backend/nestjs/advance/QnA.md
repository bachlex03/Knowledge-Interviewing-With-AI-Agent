# NestJS Advance Q&A

### Level 4: Analyzing

#### Q1: Analyze the difference between guards and interceptors.

**Question:**
en: Analyze the difference between guards and interceptors.
vi: Phân tích sự khác nhau giữa guards và interceptors.

**Answer:**
en: Guards decide whether a request can proceed, so they are best for authentication and authorization. Interceptors wrap execution, so they are better for logging, response shaping, timing, and caching.
vi: Guards quyết định request có được tiếp tục hay không, nên phù hợp nhất cho xác thực và phân quyền. Interceptors bao bọc quá trình thực thi, nên phù hợp hơn cho logging, định dạng response, đo thời gian và cache.

#### Q2: Compare services and controllers in terms of responsibility.

**Question:**
en: Compare services and controllers in terms of responsibility.
vi: So sánh services và controllers về mặt trách nhiệm.

**Answer:**
en: Controllers manage transport concerns such as routes and request/response handling, while services contain reusable business rules. Mixing them increases coupling and makes testing harder.
vi: Controllers xử lý các vấn đề liên quan đến transport như route và request/response, còn services chứa các quy tắc nghiệp vụ có thể tái sử dụng. Trộn hai phần này với nhau làm tăng độ phụ thuộc và khiến việc test khó hơn.

#### Q3: Analyze why exception filters are useful.

**Question:**
en: Analyze why exception filters are useful.
vi: Phân tích vì sao exception filter hữu ích.

**Answer:**
en: Exception filters centralize error handling, make responses consistent, and allow custom mapping from application errors to HTTP status codes. This keeps controllers and services cleaner.
vi: Exception filter tập trung hóa xử lý lỗi, làm cho response nhất quán và cho phép ánh xạ lỗi ứng dụng sang mã trạng thái HTTP tùy chỉnh. Điều này giúp controller và service gọn hơn.

#### Q4: Analyze the role of pipes in application flow.

**Question:**
en: Analyze the role of pipes in application flow.
vi: Phân tích vai trò của pipes trong luồng xử lý ứng dụng.

**Answer:**
en: Pipes transform and validate input before it reaches the handler. They enforce data quality at the boundary, preventing invalid or malformed data from spreading deeper into the application.
vi: Pipes chuyển đổi và validate dữ liệu đầu vào trước khi nó tới handler. Chúng đảm bảo chất lượng dữ liệu ở ranh giới đầu vào, ngăn dữ liệu sai hoặc không hợp lệ lan sâu vào ứng dụng.

#### Q5: Analyze a scalable NestJS module structure.

**Question:**
en: Analyze a scalable NestJS module structure.
vi: Phân tích cấu trúc module NestJS có khả năng mở rộng.

**Answer:**
en: A scalable module keeps one business domain per module, separates API, business logic, and infrastructure concerns, and uses shared providers only when necessary. This reduces coupling and improves maintainability.
vi: Một module có khả năng mở rộng sẽ giữ một miền nghiệp vụ cho mỗi module, tách API, logic nghiệp vụ và hạ tầng thành các phần riêng, và chỉ dùng shared providers khi cần thiết. Điều này giảm phụ thuộc chặt và tăng khả năng bảo trì.

---

### Level 5: Evaluating

#### Q1: Evaluate whether business logic should live in controllers or services.

**Question:**
en: Evaluate whether business logic should live in controllers or services.
vi: Đánh giá việc logic nghiệp vụ nên nằm trong controller hay service.

**Answer:**
en: Business logic should usually live in services. Controllers should stay thin so they remain focused on HTTP concerns, while services become easier to test, reuse, and evolve.
vi: Logic nghiệp vụ thường nên nằm trong service. Controller nên giữ gọn để tập trung vào các vấn đề HTTP, còn service sẽ dễ test, tái sử dụng và phát triển hơn.

#### Q2: Critique the idea of putting all reusable logic in a single shared service.

**Question:**
en: Critique the idea of putting all reusable logic in a single shared service.
vi: Phê bình ý tưởng đặt toàn bộ logic tái sử dụng vào một shared service duy nhất.

**Answer:**
en: This often creates a god service with unclear responsibilities and strong coupling between unrelated features. A better approach is to split shared logic by domain or concern.
vi: Cách này thường tạo ra một god service với trách nhiệm không rõ ràng và độ phụ thuộc chặt giữa các tính năng không liên quan. Cách tốt hơn là tách logic dùng chung theo miền nghiệp vụ hoặc theo mối quan tâm.

#### Q3: Defend the use of DTOs and validation in a NestJS API.

**Question:**
en: Defend the use of DTOs and validation in a NestJS API.
vi: Bảo vệ việc sử dụng DTO và validation trong một API NestJS.

**Answer:**
en: DTOs and validation create a clear contract at the boundary of the application. They reduce bad input, improve documentation, and make refactoring safer.
vi: DTO và validation tạo ra một hợp đồng rõ ràng ở ranh giới ứng dụng. Chúng giảm dữ liệu đầu vào sai, cải thiện tài liệu hóa và làm cho việc refactor an toàn hơn.

#### Q4: Evaluate the trade-off of using custom decorators heavily in NestJS.

**Question:**
en: Evaluate the trade-off of using custom decorators heavily in NestJS.
vi: Đánh giá đánh đổi khi sử dụng nhiều custom decorator trong NestJS.

**Answer:**
en: Custom decorators can reduce repetition and improve readability when used carefully. Overusing them can hide behavior, make debugging harder, and increase cognitive load for new team members.
vi: Custom decorator có thể giảm lặp lại và cải thiện khả năng đọc nếu dùng cẩn thận. Nhưng dùng quá nhiều có thể che giấu hành vi, làm debug khó hơn và tăng độ phức tạp nhận thức cho thành viên mới.

#### Q5: Judge when you should split a NestJS feature into multiple modules.

**Question:**
en: Judge when you should split a NestJS feature into multiple modules.
vi: Đánh giá khi nào nên tách một feature NestJS thành nhiều module.

**Answer:**
en: Split the feature when the responsibilities, dependencies, or release cadence diverge enough that a single module becomes hard to understand or maintain. The split should reduce coupling, not just increase file count.
vi: Nên tách feature khi trách nhiệm, dependency hoặc nhịp phát hành đã khác nhau đủ nhiều khiến một module duy nhất trở nên khó hiểu hoặc khó bảo trì. Việc tách ra phải giúp giảm phụ thuộc chứ không chỉ tăng số lượng file.
