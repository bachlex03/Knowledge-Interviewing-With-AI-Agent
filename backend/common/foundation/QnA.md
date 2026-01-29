en: # Foundational Backend Interview Q&A
vi: # Câu hỏi và Trả lời Phỏng vấn Backend Cơ bản

---

en: ## 1. What is the difference between GET and POST methods in HTTP?
vi: ## 1. Sự khác biệt giữa phương thức GET và POST trong HTTP là gì?

en:
- **GET**: Used to request data from a specified resource. Parameters are sent in the URL string. It is idempotent (multiple identical requests have the same effect as a single request).
- **POST**: Used to send data to a server to create/update a resource. Data is sent in the request body. It is not idempotent.

vi:
- **GET**: Được sử dụng để yêu cầu dữ liệu từ một tài nguyên cụ thể. Các tham số được gửi trong chuỗi URL. Nó có tính idempotent (nhiều yêu cầu giống hệt nhau có tác dụng tương tự như một yêu cầu duy nhất).
- **POST**: Được sử dụng để gửi dữ liệu đến máy chủ để tạo/cập nhật tài nguyên. Dữ liệu được gửi trong thân yêu cầu (request body). Nó không có tính idempotent.

---

en: ## 2. Explain the common HTTP status code categories.
vi: ## 2. Giải thích các nhóm mã trạng thái HTTP phổ biến.

en:
- **2xx (Success)**: The request was successfully received, understood, and accepted (e.g., 200 OK, 201 Created).
- **3xx (Redirection)**: Further action needs to be taken relative to the request (e.g., 301 Moved Permanently).
- **4xx (Client Error)**: The request contains bad syntax or cannot be fulfilled (e.g., 400 Bad Request, 404 Not Found).
- **5xx (Server Error)**: The server failed to fulfill an apparently valid request (e.g., 500 Internal Server Error).

vi:
- **2xx (Thành công)**: Yêu cầu đã được nhận, hiểu và chấp nhận thành công (ví dụ: 200 OK, 201 Created).
- **3xx (Chuyển hướng)**: Cần thực hiện thêm hành động liên quan đến yêu cầu (ví dụ: 301 Moved Permanently).
- **4xx (Lỗi Client)**: Yêu cầu chứa cú pháp sai hoặc không thể thực hiện được (ví dụ: 400 Bad Request, 404 Not Found).
- **5xx (Lỗi Server)**: Máy chủ không thể thực hiện một yêu cầu có vẻ hợp lệ (ví dụ: 500 Internal Server Error).

---

en: ## 3. What is a Database Index and why is it used?
vi: ## 3. Index trong cơ sở dữ liệu là gì và tại sao nó được sử dụng?

en:
A database index is a data structure that improves the speed of data retrieval operations on a database table. It works like an index in a book, allowing the database engine to find data without scanning the entire table (Full Table Scan). However, it adds overhead to write operations (INSERT, UPDATE, DELETE).

vi:
Index trong cơ sở dữ liệu là một cấu trúc dữ liệu giúp cải thiện tốc độ của các hoạt động truy xuất dữ liệu trên một bảng. Nó hoạt động giống như mục lục của một cuốn sách, cho phép bộ máy cơ sở dữ liệu tìm thấy dữ liệu mà không cần quét toàn bộ bảng (Full Table Scan). Tuy nhiên, nó làm tăng chi phí cho các hoạt động ghi (INSERT, UPDATE, DELETE).

---

en: ## 4. Difference between SQL and NoSQL databases?
vi: ## 4. Sự khác biệt giữa cơ sở dữ liệu SQL và NoSQL?

en:
- **SQL (Relational)**: Structured data, predefined schema, uses SQL for queries, vertically scalable (usually), ACID compliant. Examples: MySQL, PostgreSQL.
- **NoSQL (Non-relational)**: Unstructured or semi-structured data, dynamic schema, horizontally scalable, often follows BASE (Basically Available, Soft state, Eventual consistency). Examples: MongoDB, Redis.

vi:
- **SQL (Quan hệ)**: Dữ liệu có cấu trúc, schema xác định trước, sử dụng SQL để truy vấn, thường mở rộng theo chiều dọc, tuân thủ ACID. Ví dụ: MySQL, PostgreSQL.
- **NoSQL (Phi quan hệ)**: Dữ liệu không cấu trúc hoặc bán cấu trúc, schema linh hoạt, mở rộng theo chiều ngang, thường tuân theo BASE (Tính khả dụng cơ bản, Trạng thái mềm, Tính nhất quán cuối cùng). Ví dụ: MongoDB, Redis.

---

en: ## 5. What is JWT (JSON Web Token)?
vi: ## 5. JWT (JSON Web Token) là gì?

en:
JWT is an open standard for securely transmitting information as a JSON object. It is compact and self-contained, often used for authentication and information exchange. It consists of three parts: Header, Payload, and Signature.

vi:
JWT là một tiêu chuẩn mở để truyền thông tin một cách an toàn dưới dạng đối tượng JSON. Nó nhỏ gọn và tự chứa, thường được sử dụng để xác thực và trao đổi thông tin. Nó bao gồm ba phần: Header, Payload và Signature.
