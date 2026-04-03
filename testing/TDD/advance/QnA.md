# TDD Advance Q&A

### Level 4: Analyzing
#### Q1: Analyze the risks of skipping the "Refactor" phase in a long-term TDD project.
**Question:**
en: Analyze the risks of skipping the "Refactor" phase in a long-term TDD project.
vi: Phân tích các rủi ro của việc bỏ qua giai đoạn "Refactor" trong một dự án TDD dài hạn.

**Answer:**
en: It leads to technical debt, tight coupling, and brittle code. Tests become harder to write as the internal design rots.
vi: Nó dẫn đến nợ kỹ thuật, liên kết chặt chẽ và mã nguồn dễ gãy. Các kiểm thử trở nên khó viết hơn khi thiết kế bên trong bị xuống cấp.

#### Q2: Compare TDD with a "Test-Last" approach regarding bug detection cost.
**Question:**
en: Compare TDD with a "Test-Last" approach regarding bug detection cost.
vi: So sánh TDD với cách tiếp cận "Viết kiểm thử sau" (Test-Last) về chi phí phát hiện lỗi.

**Answer:**
en: TDD detects bugs at development time (lowest cost). Test-Last often finds bugs during integration or production (highest cost), as tests are often retrofitted to the code's flaws.
vi: TDD phát hiện lỗi ngay tại thời điểm phát triển (chi phí thấp nhất). Test-Last thường tìm thấy lỗi trong quá trình tích hợp hoặc vận hành (chi phí cao nhất), vì các kiểm thử thường được lắp đặt sau để phù hợp với các khiếm khuyết của mã nguồn.

#### Q3: Contrast Unit Tests in TDD with Integration Tests.
**Question:**
en: Search for the distinction between Unit Tests in TDD and Integration Tests.
vi: Phân tích sự khác biệt giữa Kiểm thử đơn vị trong TDD và Kiểm thử tích hợp.

**Answer:**
en: Unit tests in TDD focus on logic isolation (fast, specific). Integration tests focus on component interactions (slower, environmental dependencies). TDD primarily drives the former.
vi: Kiểm thử đơn vị trong TDD tập trung vào việc cô lập logic (nhanh, cụ thể). Kiểm thử tích hợp tập trung vào sự tương tác giữa các thành phần (chậm hơn, phụ thuộc vào môi trường). TDD chủ yếu thúc đẩy loại kiểm thử trước.

#### Q4: Analyze the impact of TDD on "Code Coverage" metrics.
**Question:**
en: Analyze the impact of TDD on "Code Coverage" metrics.
vi: Phân tích tác động của TDD đối với các chỉ số "Độ bao phủ mã nguồn" (Code Coverage).

**Answer:**
en: TDD naturally leads to high code coverage (often >90%) because code is only written to satisfy a test. However, high coverage doesn't guarantee logical correctness or quality.
vi: TDD dẫn đến độ bao phủ mã nguồn cao một cách tự nhiên (thường >90%) vì mã nguồn chỉ được viết để đáp ứng một kiểm thử. Tuy nhiên, độ bao phủ cao không đảm bảo tính chính xác về logic hay chất lượng.

#### Q5: Categorize scenarios where TDD might be difficult to apply.
**Question:**
en: Categorize scenarios where TDD might be difficult or impractical to apply.
vi: Phân loại các tình huống mà TDD có thể khó khăn hoặc không thực tế để áp dụng.

**Answer:**
en: 1. Legacy code with tight coupling. 2. Experimental UI/UX research. 3. Highly complex mathematics where the result is not easily predictable.
vi: 1. Mã nguồn cũ với liên kết chặt chẽ. 2. Nghiên cứu UI/UX thử nghiệm. 3. Các bài toán toán học cực kỳ phức tạp nơi kết quả không dễ dự đoán trước.

---

### Level 5: Evaluating
#### Q1: Evaluate the trade-offs between TDD and "Test-Last" development in a rapidly changing startup environment.
**Question:**
en: Evaluate the trade-offs between TDD and "Test-Last" development in a rapidly changing startup environment.
vi: Đánh giá sự đánh đổi giữa TDD và phát triển theo kiểu "Test-Last" (viết kiểm thử sau) trong môi trường startup thay đổi nhanh chóng.

**Answer:**
en: TDD offers long-term stability and faster maintenance. Test-Last offers faster initial delivery but risks critical failures later. TDD is usually the better investment for core business logic.
vi: TDD mang lại sự ổn định lâu dài và bảo trì nhanh hơn. Test-Last mang lại khả năng bàn giao ban đầu nhanh hơn nhưng có nguy cơ thất bại nghiêm trọng sau này. TDD thường là khoản đầu tư tốt hơn cho logic kinh doanh cốt lõi.

#### Q2: Critique the statement: "100% test coverage means the software is bug-free."
**Question:**
en: Critique the statement: "100% test coverage means the software is bug-free."
vi: Phê bình nhận định: "Độ bao phủ kiểm thử 100% có nghĩa là phần mềm không có lỗi."

**Answer:**
en: 100% coverage means every line was executed by a test, not that every logical combination or edge case was validated. It can provide a false sense of security.
vi: Độ bao phủ 100% có nghĩa là mọi dòng mã đều đã được thực thi bởi một kiểm thử, chứ không phải mọi tổ hợp logic hoặc trường hợp biên đều đã được xác thực. Nó có thể tạo ra cảm giác an toàn giả tạo.

#### Q3: Defend the practice of TDD in a team with tight deadlines.
**Question:**
en: Defend the practice of TDD in a team facing tight deadlines.
vi: Bảo vệ việc thực hành TDD trong một nhóm đang đối mặt với thời hạn (deadlines) chặt chẽ.

**Answer:**
en: TDD prevents the "debugging hell" usually found at the end of a sprint. It ensures that "done" actually means working, reducing the feedback loop and rework time.
vi: TDD ngăn chặn "địa ngục gỡ lỗi" thường thấy vào cuối mỗi sprint. Nó đảm bảo rằng "xong" thực sự có nghĩa là hoạt động tốt, làm giảm vòng lặp phản hồi và thời gian làm lại.

#### Q4: Judge when it's appropriate to skip TDD for "Spiking" (Prototyping).
**Question:**
en: Judge when it is appropriate to skip TDD in favor of "Spiking" (Prototyping).
vi: Đưa ra nhận định khi nào là thích hợp để bỏ qua TDD để thực hiện "Spiking" (tạo nguyên mẫu nhanh).

**Answer:**
en: It's appropriate when the problem space is unknown or when learning a new technology. However, the "spike" code should usually be deleted and rewritten with TDD once the direction is clear.
vi: Việc này là thích hợp khi không gian vấn đề chưa được biết rõ hoặc khi đang học một công nghệ mới. Tuy nhiên, mã nguồn "spike" thường nên được xóa và viết lại bằng TDD khi hướng đi đã rõ ràng.

#### Q5: Appraise the effectiveness of TDD in a Microservices architecture.
**Question:**
en: Appraise the effectiveness of TDD in a Microservices architecture.
vi: Đánh giá hiệu quả của TDD trong kiến trúc Microservices.

**Answer:**
en: TDD is highly effective as it helps define clear boundaries and contracts for each service via unit and contract tests, ensuring reliability before integration.
vi: TDD cực kỳ hiệu quả vì nó giúp xác định các ranh giới và hợp đồng rõ ràng cho mỗi dịch vụ thông qua các kiểm thử đơn vị và hợp đồng, đảm bảo tính tin cậy trước khi tích hợp.
