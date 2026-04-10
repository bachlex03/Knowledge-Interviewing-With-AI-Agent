# ACID Advance Q&A

### Level 4: Analyzing

#### Q_LEVEL4_176: Analyze isolation anomalies.

**Question:**
en: Analyze the relationship between dirty reads, non-repeatable reads, and phantom reads.
vi: Phân tích mối quan hệ giữa **dirty read**, **non-repeatable read**, và **phantom read**.

**Answer:**
en: These anomalies describe different ways concurrent transactions can observe unstable data. Dirty reads see uncommitted data, non-repeatable reads see changed row values across repeated reads, and phantom reads see changed row sets across repeated range queries. They are controlled by isolation levels with different concurrency trade-offs.
vi: **Vấn đề:** Khi transaction chạy đồng thời, dữ liệu quan sát được có thể không ổn định. **Dirty read** đọc dữ liệu chưa commit, **non-repeatable read** đọc lại cùng dòng nhưng giá trị đổi, còn **phantom read** đọc lại cùng điều kiện nhưng tập dòng thay đổi. **Giải pháp:** Chọn **isolation level** phù hợp với rule nghiệp vụ và chấp nhận đánh đổi concurrency.

#### Q_LEVEL4_287: Analyze deadlock causes.

**Question:**
en: Analyze common causes of deadlocks in transactional systems.
vi: Phân tích nguyên nhân phổ biến gây **deadlock** trong hệ thống transaction.

**Answer:**
en: Deadlocks commonly happen when transactions acquire locks in different orders, hold locks too long, scan too many rows, or mix user interaction with open transactions. Reducing transaction duration, indexing queries, and enforcing consistent lock ordering can lower the risk.
vi: **Deadlock** thường xảy ra khi transaction lấy lock theo thứ tự khác nhau, giữ lock quá lâu, scan quá nhiều dòng hoặc mở transaction trong lúc chờ người dùng/hệ thống ngoài. Nên rút ngắn transaction, thêm index phù hợp và thống nhất thứ tự truy cập dữ liệu.

```csharp
public static class LockOrderingRule
{
    public static (int FirstAccountId, int SecondAccountId) OrderLocks(int accountA, int accountB)
    {
        // Always lock accounts in the same order to reduce deadlock risk.
        return accountA < accountB ? (accountA, accountB) : (accountB, accountA);
    }
}
```

#### Q_LEVEL4_398: Analyze transaction scope size.

**Question:**
en: Analyze the risks of making a transaction scope too large or too small.
vi: Phân tích rủi ro khi transaction scope quá lớn hoặc quá nhỏ.

**Answer:**
en: A scope that is too small may fail to protect a full business invariant, leaving related data inconsistent. A scope that is too large holds locks longer, increases deadlock risk, reduces throughput, and may include external calls that cannot be rolled back.
vi: Scope quá nhỏ có thể không bảo vệ đủ **business invariant**, làm dữ liệu liên quan lệch nhau. Scope quá lớn giữ lock lâu, tăng deadlock, giảm throughput và có thể bao gồm external call không rollback được. Thiết kế tốt là đặt boundary quanh một thay đổi nghiệp vụ cần nhất quán mạnh.

#### Q_LEVEL4_409: Analyze ACID in microservices.

**Question:**
en: Analyze why ACID transactions become harder in microservice architectures.
vi: Phân tích vì sao ACID transaction khó hơn trong kiến trúc microservices.

**Answer:**
en: Microservices often own separate databases and communicate over unreliable networks. A single local transaction cannot atomically update multiple services. Distributed transactions add coordination cost, so teams often use sagas, outbox patterns, and eventual consistency instead.
vi: Trong **microservices**, mỗi service thường sở hữu database riêng và giao tiếp qua network có thể lỗi. Một local transaction không thể atomic update nhiều service. **Distributed transaction** tốn chi phí phối hợp, nên nhiều hệ thống dùng **Saga**, **Outbox pattern** và **eventual consistency**.

#### Q_LEVEL4_510: Analyze durability mechanisms.

**Question:**
en: Analyze how durability is achieved through logs and checkpoints.
vi: Phân tích cách **Durability** được đảm bảo bằng log và checkpoint.

**Answer:**
en: Durability is commonly achieved by writing transaction records to durable logs before acknowledging commit. Checkpoints later flush accumulated changes to data files. After a crash, the database uses logs to redo committed work and undo incomplete work.
vi: **Durability** thường được đảm bảo bằng cách ghi transaction record vào durable log trước khi xác nhận commit. **Checkpoint** sau đó flush thay đổi vào data file. Sau crash, database dùng log để redo phần đã commit và undo phần chưa hoàn tất.

---

### Level 5: Evaluating

#### Q_LEVEL5_621: Evaluate isolation level selection.

**Question:**
en: Evaluate how to choose an isolation level for a high-traffic financial system.
vi: Đánh giá cách chọn **isolation level** cho hệ thống tài chính traffic cao.

**Answer:**
en: The choice should protect financial invariants first, then optimize concurrency. Serializable or repeatable-read behavior may be necessary for balance and limit checks, but lower isolation can be acceptable for read-only dashboards. The system should combine proper constraints, retries, monitoring, and carefully scoped transactions.
vi: Với hệ thống tài chính, ưu tiên đầu tiên là bảo vệ invariant như số dư, hạn mức và trạng thái giao dịch. **Serializable** hoặc hành vi tương đương có thể cần cho phần ghi quan trọng, nhưng dashboard read-only có thể dùng mức thấp hơn. Nên kết hợp constraint, retry, monitoring và transaction scope gọn.

#### Q_LEVEL5_732: Evaluate optimistic versus pessimistic locking.

**Question:**
en: Evaluate when to use optimistic locking versus pessimistic locking.
vi: Đánh giá khi nào nên dùng **optimistic locking** và khi nào dùng **pessimistic locking**.

**Answer:**
en: Optimistic locking is better when conflicts are rare and throughput matters. Pessimistic locking is better when conflicts are frequent, the cost of retry is high, or the system must reserve a resource strictly. The best choice depends on contention, user experience, and failure cost.
vi: **Optimistic locking** phù hợp khi conflict hiếm và cần throughput cao. **Pessimistic locking** phù hợp khi conflict thường xuyên, retry đắt, hoặc cần giữ tài nguyên nghiêm ngặt như seat booking. Lựa chọn phụ thuộc vào mức contention, trải nghiệm người dùng và chi phí thất bại.

#### Q_LEVEL5_843: Defend avoiding distributed transactions.

**Question:**
en: Defend a decision to avoid distributed transactions in a microservice system.
vi: Bảo vệ quyết định tránh **distributed transaction** trong hệ thống microservice.

**Answer:**
en: Avoiding distributed transactions can improve availability, reduce coupling, and simplify failure handling. Patterns such as outbox, saga, idempotent consumers, and compensation can preserve business correctness without requiring all services to commit atomically at the same time.
vi: Tránh **distributed transaction** có thể tăng availability, giảm coupling và đơn giản hóa xử lý lỗi. Các pattern như **Outbox**, **Saga**, **idempotent consumer** và compensation giúp giữ đúng nghiệp vụ mà không bắt mọi service commit atomic cùng lúc.

```csharp
public sealed record OutboxMessage(Guid Id, string Type, string Payload, DateTime CreatedAt);

public static class OutboxIdea
{
    public static void SaveOrderAndEventInSameLocalTransaction()
    {
        // Save domain data and an outbox message in one local ACID transaction.
        // A background publisher sends the message later with retry.
    }
}
```

#### Q_LEVEL5_954: Critique ACID as always required.

**Question:**
en: Critique the statement: "Every system should always use the strongest ACID settings."
vi: Phê bình nhận định: "Mọi hệ thống luôn nên dùng cấu hình ACID mạnh nhất."

**Answer:**
en: The statement is too absolute. Strong ACID settings are essential for critical invariants, but they can reduce throughput, increase lock contention, and make distributed systems harder to operate. Mature design applies strong consistency where correctness requires it and weaker consistency where the business can tolerate it.
vi: Nhận định này quá tuyệt đối. ACID mạnh rất cần cho invariant quan trọng, nhưng có thể giảm throughput, tăng lock contention và làm hệ phân tán khó vận hành. Thiết kế trưởng thành là dùng strong consistency ở nơi nghiệp vụ bắt buộc, và dùng consistency yếu hơn ở nơi có thể chấp nhận.

#### Q_LEVEL5_165: Evaluate recovery strategy.

**Question:**
en: Evaluate what makes a database recovery strategy trustworthy after a crash.
vi: Đánh giá điều gì làm chiến lược phục hồi database đáng tin cậy sau crash.

**Answer:**
en: A trustworthy recovery strategy has durable logs, tested restore procedures, backups, replication checks, point-in-time recovery where needed, and regular disaster-recovery drills. Durability is not only a storage feature; it must be proven operationally.
vi: Chiến lược phục hồi đáng tin cần có durable log, quy trình restore đã test, backup, kiểm tra replication, **point-in-time recovery** khi cần và diễn tập DR định kỳ. **Durability** không chỉ là tính năng storage; nó phải được chứng minh bằng vận hành thực tế.
