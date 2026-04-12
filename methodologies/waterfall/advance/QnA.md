# Waterfall Advanced Q&A

### Level 4: Analyzing

#### Q1: Analyze the main trade-off between Waterfall and iterative methodologies.

**Question:**
en: Analyze the main trade-off between Waterfall and iterative methodologies.
vi: Phân tích sự đánh đổi chính giữa Waterfall và các phương pháp lặp.

**Answer:**
en: Waterfall provides stronger upfront planning, control gates, and documentation, but iterative methods provide faster feedback and better adaptation to change. The core trade-off is predictability versus learning speed.
vi: Waterfall cung cấp việc lập kế hoạch sớm, các cổng kiểm soát và tài liệu mạnh hơn, nhưng các phương pháp lặp lại mang đến phản hồi nhanh hơn và khả năng thích nghi tốt hơn với thay đổi. Sự đánh đổi cốt lõi là tính dự đoán trước so với tốc độ học hỏi.

#### Q2: Analyze how requirement uncertainty affects a Waterfall project.

**Question:**
en: Analyze how requirement uncertainty affects a Waterfall project.
vi: Phân tích cách sự không chắc chắn của yêu cầu ảnh hưởng đến một dự án Waterfall.

**Answer:**
en: Requirement uncertainty weakens Waterfall because the model depends on stable upstream decisions. If assumptions are wrong, defects propagate into design, implementation, and test plans, increasing rework across multiple artifacts.
vi: Sự không chắc chắn của yêu cầu làm suy yếu Waterfall vì mô hình này phụ thuộc vào các quyết định đầu nguồn phải ổn định. Nếu các giả định sai, lỗi sẽ lan sang thiết kế, triển khai và kế hoạch kiểm thử, làm tăng khối lượng làm lại trên nhiều loại tài liệu và đầu ra.

#### Q3: Compare scope control in Waterfall versus Agile delivery.

**Question:**
en: Compare scope control in Waterfall versus Agile delivery.
vi: So sánh cách kiểm soát phạm vi trong Waterfall và Agile.

**Answer:**
en: Waterfall tries to control scope by freezing it early and managing changes formally. Agile controls scope by timeboxing delivery and continuously reprioritizing work. Waterfall protects the baseline; Agile protects adaptability.
vi: Waterfall cố gắng kiểm soát phạm vi bằng cách đóng băng phạm vi sớm và quản lý thay đổi một cách chính thức. Agile kiểm soát phạm vi bằng cách giới hạn thời gian giao hàng và liên tục ưu tiên lại công việc. Waterfall bảo vệ đường cơ sở; Agile bảo vệ khả năng thích nghi.

#### Q4: Analyze why defect discovery timing matters more in Waterfall.

**Question:**
en: Analyze why defect discovery timing matters more in Waterfall.
vi: Phân tích vì sao thời điểm phát hiện lỗi lại quan trọng hơn trong Waterfall.

**Answer:**
en: In Waterfall, late defect discovery is especially damaging because downstream work assumes earlier artifacts are correct. A requirement defect found during system testing can invalidate design decisions, code, and test cases all at once.
vi: Trong Waterfall, việc phát hiện lỗi muộn đặc biệt nguy hiểm vì công việc phía sau giả định rằng các đầu ra trước đó là đúng. Một lỗi yêu cầu được phát hiện ở giai đoạn kiểm thử hệ thống có thể làm vô hiệu cùng lúc các quyết định thiết kế, mã nguồn và test case.

#### Q5: Analyze whether Waterfall can work in regulated environments.

**Question:**
en: Analyze whether Waterfall can work in regulated environments.
vi: Phân tích liệu Waterfall có thể hoạt động tốt trong môi trường bị quản lý chặt chẽ hay không.

**Answer:**
en: Yes, Waterfall can work well in regulated environments because traceability, approvals, documentation, and auditability are first-class concerns. However, it still fails if the team mistakes documentation completeness for product correctness.
vi: Có, Waterfall có thể hoạt động tốt trong môi trường bị quản lý chặt chẽ vì khả năng truy vết, phê duyệt, tài liệu và kiểm toán là các mối quan tâm hạng nhất. Tuy nhiên, nó vẫn thất bại nếu nhóm nhầm lẫn giữa việc tài liệu đầy đủ và sản phẩm thực sự đúng.

### Level 5: Evaluating

#### Q6: Evaluate the statement: "Waterfall fails because it is old."

**Question:**
en: Evaluate the statement: "Waterfall fails because it is old."
vi: Đánh giá nhận định: "Waterfall thất bại vì nó lỗi thời."

**Answer:**
en: That statement is too simplistic. Waterfall does not fail because of age; it fails when used in environments with unstable requirements, delayed feedback, or high discovery. In the right context, it can still be effective.
vi: Nhận định đó quá đơn giản. Waterfall không thất bại vì cũ; nó thất bại khi được dùng trong môi trường có yêu cầu không ổn định, phản hồi chậm hoặc mức độ khám phá cao. Trong bối cảnh phù hợp, nó vẫn có thể hiệu quả.

#### Q7: Judge when a hybrid approach is better than pure Waterfall.

**Question:**
en: Judge when a hybrid approach is better than pure Waterfall.
vi: Nhận định khi nào một cách tiếp cận lai tốt hơn Waterfall thuần túy.

**Answer:**
en: A hybrid approach is better when governance and documentation are required, but some solution details still need iteration. For example, a project may keep formal phase gates for compliance while using short feedback cycles inside design or implementation.
vi: Cách tiếp cận lai tốt hơn khi việc quản trị và tài liệu là bắt buộc, nhưng một số chi tiết của giải pháp vẫn cần lặp lại. Ví dụ, dự án có thể giữ các cổng giai đoạn chính thức để tuân thủ trong khi vẫn dùng các vòng phản hồi ngắn bên trong giai đoạn thiết kế hoặc triển khai.

#### Q8: Evaluate whether Waterfall is appropriate for a startup building a new product with unclear market needs.

**Question:**
en: Evaluate whether Waterfall is appropriate for a startup building a new product with unclear market needs.
vi: Đánh giá liệu Waterfall có phù hợp cho một startup xây dựng sản phẩm mới với nhu cầu thị trường chưa rõ ràng hay không.

**Answer:**
en: It is usually a poor fit. A startup with unclear market needs requires experimentation, rapid feedback, and frequent scope adjustment. Waterfall optimizes for certainty and control, which is the opposite of early product discovery.
vi: Nó thường là lựa chọn không phù hợp. Một startup với nhu cầu thị trường chưa rõ ràng cần thử nghiệm, phản hồi nhanh và điều chỉnh phạm vi thường xuyên. Waterfall tối ưu cho sự chắc chắn và kiểm soát, điều này đối lập với giai đoạn khám phá sản phẩm ban đầu.
