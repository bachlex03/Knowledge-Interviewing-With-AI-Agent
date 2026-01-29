en: # Advanced Backend Interview Q&A
vi: # Câu hỏi và Trả lời Phỏng vấn Backend Nâng cao

---

en: ## 1. Horizontal vs. Vertical Scaling?
vi: ## 1. Mở rộng theo chiều ngang (Horizontal) so với chiều dọc (Vertical)?

en:
- **Vertical Scaling (Scaling Up)**: Adding more power (CPU, RAM) to an existing server. It has a physical limit and often involves downtime.
- **Horizontal Scaling (Scaling Out)**: Adding more servers to the pool. It is more complex to manage (requires Load Balancers) but offers better availability and virtually unlimited growth.

vi:
- **Mở rộng theo chiều dọc (Scale Up)**: Thêm nhiều tài nguyên hơn (CPU, RAM) vào một máy chủ hiện có. Nó có giới hạn vật lý và thường gây ra thời gian dừng hoạt động (downtime).
- **Mở rộng theo chiều ngang (Scale Out)**: Thêm nhiều máy chủ hơn vào hệ thống. Việc quản lý phức tạp hơn (yêu cầu Load Balancer) nhưng mang lại tính khả dụng tốt hơn và khả năng tăng trưởng gần như không giới hạn.

---

en: ## 2. What are ACID properties in a Database?
vi: ## 2. Các thuộc tính ACID trong Cơ sở dữ liệu là gì?

en:
- **Atomicity**: Either all operations in a transaction succeed or none do.
- **Consistency**: A transaction takes the database from one valid state to another.
- **Isolation**: Concurrent transactions do not interfere with each other.
- **Durability**: Once a transaction is committed, it remains saved even in case of power failure.

vi:
- **Atomicity (Tính nguyên tử)**: Tất cả các hoạt động trong một giao dịch thành công hoặc không có hoạt động nào thành công.
- **Consistency (Tính nhất quán)**: Một giao dịch đưa cơ sở dữ liệu từ trạng thái hợp lệ này sang trạng thái hợp lệ khác.
- **Isolation (Tính cô lập)**: Các giao dịch đồng thời không can thiệp lẫn nhau.
- **Durability (Tính bền vững)**: Một khi giao dịch đã được commit, nó vẫn được lưu lại ngay cả khi xảy ra lỗi hệ thống hoặc mất điện.

---

en: ## 3. Explain Microservices vs. Monolithic Architecture.
vi: ## 3. Giải thích kiến trúc Microservices so với Monolith.

en:
- **Monolith**: A single unified unit where all components are tightly coupled. Easy to develop and deploy initially but hard to scale and maintain as it grows.
- **Microservices**: Application is built as a collection of small autonomous services modeled around a business domain. Services communicate via APIs (REST, gRPC) or message brokers. High scalability and flexibility but high operational complexity.

vi:
- **Monolith**: Một đơn vị thống nhất duy nhất trong đó tất cả các thành phần được liên kết chặt chẽ. Dễ phát triển và triển khai ban đầu nhưng khó mở rộng và bảo trì khi quy mô lớn dần.
- **Microservices**: Ứng dụng được xây dựng như một tập hợp các dịch vụ nhỏ tự chủ được mô hình hóa quanh một miền kinh doanh. Các dịch vụ giao tiếp qua API (REST, gRPC) hoặc message broker. Khả năng mở rộng và linh hoạt cao nhưng độ phức tạp trong vận hành cũng cao.

---

en: ## 4. What is Database Sharding?
vi: ## 4. Sharding cơ sở dữ liệu là gì?

en:
Sharding is a method for distributing data across multiple databases. It involves breaking up a large database into smaller, faster, more easily managed parts called "shards". Each shard is its own separate database, but collectively they form a single logical database.

vi:
Sharding là một phương pháp để phân phối dữ liệu trên nhiều cơ sở dữ liệu. Nó bao gồm việc chia nhỏ một cơ sở dữ liệu lớn thành các phần nhỏ hơn, nhanh hơn, dễ quản lý hơn được gọi là "shards". Mỗi shard là một cơ sở dữ liệu riêng biệt, nhưng tập hợp lại chúng tạo thành một cơ sở dữ liệu logic duy nhất.

---

en: ## 5. What are common Cache Invalidation strategies?
vi: ## 5. Các chiến lược vô hiệu hóa Cache (Cache Invalidation) phổ biến là gì?

en:
- **Write-through cache**: Data is written into the cache and the permanent data store at the same time.
- **Write-around cache**: Data is written directly to permanent storage, bypassing the cache.
- **Write-back cache**: Data is written only to the cache initially, then written to the permanent data store after a delay or under certain conditions.

vi:
- **Write-through cache**: Dữ liệu được ghi vào cache và kho lưu trữ dữ liệu vĩnh viễn cùng một lúc.
- **Write-around cache**: Dữ liệu được ghi trực tiếp vào kho lưu trữ vĩnh viễn, bỏ qua cache.
- **Write-back cache**: Dữ liệu ban đầu chỉ được ghi vào cache, sau đó mới được ghi vào kho lưu trữ vĩnh viễn sau một khoảng thời gian chờ hoặc dưới các điều kiện nhất định.
