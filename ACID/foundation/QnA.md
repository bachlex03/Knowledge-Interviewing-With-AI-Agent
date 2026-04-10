# ACID Foundation Q&A

### Level 1: Remembering

#### Q_LEVEL1_104: What does ACID stand for?

**Question:**
en: What does ACID stand for in database transactions?
vi: ACID trong database transaction là viết tắt của những gì?

**Answer:**
en: ACID stands for Atomicity, Consistency, Isolation, and Durability. These four properties describe how reliable database transactions should behave.
vi: ACID là viết tắt của **Atomicity**, **Consistency**, **Isolation**, và **Durability**. Đây là bốn thuộc tính mô tả cách một **database transaction** đáng tin cậy nên hoạt động.

#### Q_LEVEL1_217: What is a database transaction?

**Question:**
en: What is a database transaction?
vi: **Database transaction** là gì?

**Answer:**
en: A database transaction is a group of operations treated as one logical unit of work. It either completes successfully as a whole or fails and rolls back.
vi: **Database transaction** là một nhóm thao tác được xem như một đơn vị công việc logic. Nó hoặc thành công toàn bộ, hoặc thất bại và được **rollback**.

#### Q_LEVEL1_328: Define Atomicity.

**Question:**
en: Define Atomicity in ACID.
vi: Định nghĩa **Atomicity** trong ACID.

**Answer:**
en: Atomicity means all operations in a transaction succeed together or fail together. There should be no partial update left behind.
vi: **Atomicity** nghĩa là mọi thao tác trong transaction phải thành công cùng nhau hoặc thất bại cùng nhau. Hệ thống không được để lại trạng thái cập nhật dở dang.

#### Q_LEVEL1_439: Define Consistency.

**Question:**
en: Define Consistency in ACID.
vi: Định nghĩa **Consistency** trong ACID.

**Answer:**
en: Consistency means a transaction moves the database from one valid state to another valid state while preserving rules such as constraints, foreign keys, and business invariants.
vi: **Consistency** nghĩa là transaction chuyển database từ một trạng thái hợp lệ sang một trạng thái hợp lệ khác, đồng thời giữ đúng constraint, foreign key và business invariant.

#### Q_LEVEL1_542: Define Isolation.

**Question:**
en: Define Isolation in ACID.
vi: Định nghĩa **Isolation** trong ACID.

**Answer:**
en: Isolation means concurrent transactions should not interfere with each other in a way that causes incorrect results.
vi: **Isolation** nghĩa là các transaction chạy đồng thời không được can thiệp lẫn nhau theo cách tạo ra kết quả sai.

#### Q_LEVEL1_651: Define Durability.

**Question:**
en: Define Durability in ACID.
vi: Định nghĩa **Durability** trong ACID.

**Answer:**
en: Durability means once a transaction is committed, its changes survive crashes, restarts, or power loss.
vi: **Durability** nghĩa là sau khi transaction đã **commit**, thay đổi của nó phải còn tồn tại dù hệ thống crash, restart hoặc mất điện.

#### Q_LEVEL1_762: What is commit?

**Question:**
en: What does `commit` mean in a transaction?
vi: `commit` trong transaction nghĩa là gì?

**Answer:**
en: `commit` makes the transaction's changes permanent and visible according to the database isolation rules.
vi: `commit` làm cho các thay đổi của transaction trở thành chính thức và được nhìn thấy theo quy tắc **Isolation** của database.

#### Q_LEVEL1_873: What is rollback?

**Question:**
en: What does `rollback` mean in a transaction?
vi: `rollback` trong transaction nghĩa là gì?

**Answer:**
en: `rollback` cancels the transaction and restores the database to the state before the transaction began.
vi: `rollback` hủy transaction và đưa database về trạng thái trước khi transaction bắt đầu.

#### Q_LEVEL1_984: What is a transaction boundary?

**Question:**
en: What is a transaction boundary?
vi: **Transaction boundary** là gì?

**Answer:**
en: A transaction boundary defines where a transaction starts, where it commits, and where it rolls back on failure.
vi: **Transaction boundary** xác định transaction bắt đầu ở đâu, commit ở đâu và rollback ở đâu khi có lỗi.

#### Q_LEVEL1_195: What is an isolation level?

**Question:**
en: What is an isolation level?
vi: **Isolation level** là gì?

**Answer:**
en: An isolation level controls how much one transaction can see changes made by other concurrent transactions.
vi: **Isolation level** kiểm soát mức độ một transaction có thể nhìn thấy thay đổi từ các transaction khác đang chạy đồng thời.

#### Q_LEVEL1_206: Name common isolation levels.

**Question:**
en: Name common database isolation levels.
vi: Kể tên các **isolation level** phổ biến trong database.

**Answer:**
en: Common isolation levels include Read Uncommitted, Read Committed, Repeatable Read, Snapshot, and Serializable.
vi: Các **isolation level** phổ biến gồm **Read Uncommitted**, **Read Committed**, **Repeatable Read**, **Snapshot**, và **Serializable**.

#### Q_LEVEL1_317: What is a dirty read?

**Question:**
en: What is a dirty read?
vi: **Dirty read** là gì?

**Answer:**
en: A dirty read happens when a transaction reads data written by another transaction that has not committed yet.
vi: **Dirty read** xảy ra khi một transaction đọc dữ liệu do transaction khác ghi nhưng transaction đó chưa **commit**.

#### Q_LEVEL1_428: What is a non-repeatable read?

**Question:**
en: What is a non-repeatable read?
vi: **Non-repeatable read** là gì?

**Answer:**
en: A non-repeatable read happens when a transaction reads the same row twice and gets different values because another committed transaction changed it.
vi: **Non-repeatable read** xảy ra khi một transaction đọc cùng một dòng hai lần nhưng nhận giá trị khác nhau vì transaction khác đã commit thay đổi.

#### Q_LEVEL1_539: What is a phantom read?

**Question:**
en: What is a phantom read?
vi: **Phantom read** là gì?

**Answer:**
en: A phantom read happens when a transaction repeats a range query and sees new or missing rows because another transaction inserted or deleted matching rows.
vi: **Phantom read** xảy ra khi một transaction chạy lại truy vấn theo khoảng và thấy dòng mới hoặc mất dòng vì transaction khác đã insert/delete dữ liệu phù hợp.

#### Q_LEVEL1_640: What is a deadlock?

**Question:**
en: What is a deadlock in database transactions?
vi: **Deadlock** trong database transaction là gì?

**Answer:**
en: A deadlock happens when two or more transactions wait for each other to release locks, so none can continue.
vi: **Deadlock** xảy ra khi hai hoặc nhiều transaction chờ nhau nhả lock, khiến không transaction nào tiếp tục được.

#### Q_LEVEL1_751: What is pessimistic locking?

**Question:**
en: What is pessimistic locking?
vi: **Pessimistic locking** là gì?

**Answer:**
en: Pessimistic locking locks data before modifying it, assuming conflicts are likely. Other transactions may need to wait until the lock is released.
vi: **Pessimistic locking** khóa dữ liệu trước khi chỉnh sửa vì giả định conflict có khả năng xảy ra. Transaction khác có thể phải chờ đến khi lock được nhả.

#### Q_LEVEL1_862: What is optimistic locking?

**Question:**
en: What is optimistic locking?
vi: **Optimistic locking** là gì?

**Answer:**
en: Optimistic locking allows concurrent work and checks for conflicts at update time, often using a version column.
vi: **Optimistic locking** cho phép xử lý đồng thời và kiểm tra conflict lúc update, thường dùng một cột version.

#### Q_LEVEL1_973: What is write-ahead logging?

**Question:**
en: What is write-ahead logging?
vi: **Write-ahead logging** là gì?

**Answer:**
en: Write-ahead logging records intended changes to a log before applying them to data files, helping the database recover after crashes.
vi: **Write-ahead logging** ghi thay đổi dự kiến vào log trước khi áp dụng vào file dữ liệu, giúp database phục hồi sau sự cố.

#### Q_LEVEL1_184: What is a distributed transaction?

**Question:**
en: What is a distributed transaction?
vi: **Distributed transaction** là gì?

**Answer:**
en: A distributed transaction is a transaction that spans multiple databases, services, or resource managers.
vi: **Distributed transaction** là transaction trải qua nhiều database, service hoặc resource manager khác nhau.

#### Q_LEVEL1_295: What is eventual consistency?

**Question:**
en: What is eventual consistency?
vi: **Eventual consistency** là gì?

**Answer:**
en: Eventual consistency means replicas or services may temporarily differ, but they are expected to become consistent later if no new updates occur.
vi: **Eventual consistency** nghĩa là replica hoặc service có thể tạm thời khác nhau, nhưng sẽ dần nhất quán nếu không có cập nhật mới.

---

### Level 2: Understanding

#### Q_LEVEL2_116: Explain why Atomicity prevents partial updates.

**Question:**
en: Explain why Atomicity prevents partial updates.
vi: Giải thích vì sao **Atomicity** ngăn cập nhật dở dang.

**Answer:**
en: Atomicity treats multiple operations as one indivisible unit. If one operation fails, the database rolls back the entire unit so the system does not keep half-finished data.
vi: **Atomicity** xem nhiều thao tác như một đơn vị không thể chia nhỏ. Nếu một thao tác lỗi, database rollback toàn bộ để hệ thống không giữ dữ liệu nửa thành công nửa thất bại.

#### Q_LEVEL2_227: Explain Consistency with constraints.

**Question:**
en: Explain how Consistency relates to constraints.
vi: Giải thích **Consistency** liên quan thế nào đến constraint.

**Answer:**
en: Consistency ensures that committed transactions do not violate rules such as primary keys, foreign keys, unique constraints, and valid business states.
vi: **Consistency** đảm bảo transaction đã commit không vi phạm rule như primary key, foreign key, unique constraint và trạng thái nghiệp vụ hợp lệ.

#### Q_LEVEL2_338: Explain Isolation in concurrent transfers.

**Question:**
en: Explain Isolation using concurrent bank transfers.
vi: Giải thích **Isolation** bằng ví dụ chuyển tiền đồng thời.

**Answer:**
en: If two transfers update the same account at the same time, Isolation ensures each transaction observes a controlled view of data and does not overwrite or misread another transaction's work.
vi: Khi hai giao dịch chuyển tiền cập nhật cùng một tài khoản, **Isolation** đảm bảo mỗi transaction nhìn dữ liệu theo cách được kiểm soát, không ghi đè hoặc đọc sai công việc của transaction khác.

#### Q_LEVEL2_449: Explain Durability after commit.

**Question:**
en: Explain why Durability matters after commit.
vi: Giải thích vì sao **Durability** quan trọng sau khi commit.

**Answer:**
en: After commit, users and services trust that the change is permanent. Durability protects that trust by ensuring committed data can be recovered after crashes.
vi: Sau khi commit, người dùng và service tin rằng thay đổi đã chính thức. **Durability** bảo vệ niềm tin đó bằng cách đảm bảo dữ liệu đã commit có thể phục hồi sau crash.

#### Q_LEVEL2_550: Compare commit and rollback.

**Question:**
en: Compare `commit` and `rollback`.
vi: So sánh `commit` và `rollback`.

**Answer:**
en: `commit` finalizes a transaction and persists its changes. `rollback` cancels the transaction and removes its uncommitted changes.
vi: `commit` hoàn tất transaction và lưu thay đổi. `rollback` hủy transaction và loại bỏ các thay đổi chưa commit.

#### Q_LEVEL2_661: Explain why transaction boundaries belong near business operations.

**Question:**
en: Explain why transaction boundaries should align with business operations.
vi: Giải thích vì sao **transaction boundary** nên bám theo nghiệp vụ.

**Answer:**
en: A transaction should protect a complete business invariant. If the boundary is too small, related updates may become inconsistent; if too large, locks and contention increase.
vi: Transaction nên bảo vệ một **business invariant** hoàn chỉnh. Boundary quá nhỏ dễ làm dữ liệu liên quan lệch nhau; boundary quá lớn làm lock lâu hơn và tăng contention.

#### Q_LEVEL2_772: Compare Read Committed and Serializable.

**Question:**
en: Compare Read Committed and Serializable isolation levels.
vi: So sánh **Read Committed** và **Serializable**.

**Answer:**
en: Read Committed prevents dirty reads but may still allow non-repeatable reads and phantom reads. Serializable provides the strongest isolation but can reduce concurrency and increase retries.
vi: **Read Committed** ngăn dirty read nhưng vẫn có thể có non-repeatable read và phantom read. **Serializable** cô lập mạnh nhất nhưng có thể giảm concurrency và tăng retry.

#### Q_LEVEL2_883: Explain dirty read risk.

**Question:**
en: Explain why dirty reads are risky.
vi: Giải thích vì sao **dirty read** nguy hiểm.

**Answer:**
en: A dirty read can use data that is later rolled back. This can cause decisions, calculations, or downstream writes based on data that never truly existed.
vi: **Dirty read** có thể dùng dữ liệu sau đó bị rollback. Điều này làm quyết định, tính toán hoặc ghi dữ liệu downstream dựa trên dữ liệu chưa từng thật sự tồn tại.

#### Q_LEVEL2_994: Explain non-repeatable read risk.

**Question:**
en: Explain why non-repeatable reads matter in reports or validations.
vi: Giải thích vì sao **non-repeatable read** quan trọng trong report hoặc validation.

**Answer:**
en: If the same row changes during a transaction, calculations or validations may become inconsistent. This is especially risky for financial summaries and rule checks.
vi: Nếu cùng một dòng thay đổi trong lúc transaction đang chạy, tính toán hoặc validation có thể không nhất quán. Rủi ro này lớn trong report tài chính và kiểm tra rule nghiệp vụ.

#### Q_LEVEL2_105: Explain phantom read risk.

**Question:**
en: Explain why phantom reads matter for range queries.
vi: Giải thích vì sao **phantom read** quan trọng với truy vấn theo khoảng.

**Answer:**
en: Range queries depend on a set of rows. If another transaction inserts or deletes matching rows, repeated range checks can produce different results and break business rules.
vi: Truy vấn theo khoảng phụ thuộc vào một tập dòng. Nếu transaction khác insert/delete dòng phù hợp, việc kiểm tra lại có thể cho kết quả khác và phá rule nghiệp vụ.

#### Q_LEVEL2_216: Compare optimistic and pessimistic locking.

**Question:**
en: Compare optimistic locking and pessimistic locking.
vi: So sánh **optimistic locking** và **pessimistic locking**.

**Answer:**
en: Optimistic locking assumes conflicts are rare and checks at write time. Pessimistic locking assumes conflicts are likely and blocks others earlier by taking locks.
vi: **Optimistic locking** giả định conflict hiếm và kiểm tra khi ghi. **Pessimistic locking** giả định conflict dễ xảy ra và khóa sớm để chặn transaction khác.

#### Q_LEVEL2_327: Explain how deadlocks are resolved.

**Question:**
en: Explain how databases commonly resolve deadlocks.
vi: Giải thích database thường xử lý **deadlock** như thế nào.

**Answer:**
en: Databases detect cycles of waiting transactions, choose one transaction as a victim, abort it, and allow the other transactions to continue.
vi: Database phát hiện vòng chờ giữa các transaction, chọn một transaction làm nạn nhân, abort transaction đó và cho các transaction còn lại tiếp tục.

#### Q_LEVEL2_438: Explain write-ahead logging and recovery.

**Question:**
en: Explain how write-ahead logging supports recovery.
vi: Giải thích cách **write-ahead logging** hỗ trợ phục hồi.

**Answer:**
en: Because changes are written to the log before data pages are finalized, the database can replay committed work or undo incomplete work after a crash.
vi: Vì thay đổi được ghi vào log trước khi page dữ liệu hoàn tất, database có thể replay phần đã commit hoặc undo phần chưa hoàn tất sau crash.

#### Q_LEVEL2_549: Explain why distributed transactions are hard.

**Question:**
en: Explain why distributed transactions are difficult.
vi: Giải thích vì sao **distributed transaction** khó.

**Answer:**
en: They require coordination across multiple systems that can fail independently. Network latency, partial failure, retries, and inconsistent resource managers make atomic commit difficult.
vi: Chúng cần phối hợp nhiều hệ thống có thể lỗi độc lập. Network latency, partial failure, retry và resource manager khác nhau làm atomic commit trở nên khó.

#### Q_LEVEL2_650: Explain ACID versus eventual consistency.

**Question:**
en: Explain the trade-off between ACID and eventual consistency.
vi: Giải thích sự đánh đổi giữa ACID và **eventual consistency**.

**Answer:**
en: ACID prioritizes strong correctness within a transaction boundary. Eventual consistency relaxes immediate consistency to improve availability, scalability, and decoupling in distributed systems.
vi: ACID ưu tiên tính đúng đắn mạnh trong một transaction boundary. **Eventual consistency** nới lỏng nhất quán tức thời để tăng availability, scalability và decoupling trong hệ phân tán.

---

### Level 3: Applying

#### Q_LEVEL3_161: Implement a transaction boundary.

**Question:**
en: Apply ACID principles by implementing a transaction boundary for a money transfer.
vi: Áp dụng ACID bằng cách triển khai **transaction boundary** cho nghiệp vụ chuyển tiền.

**Answer:**
en: The debit and credit operations must be in the same transaction. If either operation fails, rollback prevents partial balance updates.
vi: **Vấn đề:** Trừ tiền và cộng tiền tách rời có thể gây mất tiền nếu một bước lỗi. **Giải pháp:** Đặt cả hai thao tác trong cùng transaction; lỗi thì rollback toàn bộ.

```csharp
using System;
using System.Data;
using System.Threading.Tasks;

public sealed class TransferService
{
    private readonly IDbConnection _connection;

    public TransferService(IDbConnection connection) => _connection = connection;

    public async Task TransferAsync(int fromId, int toId, decimal amount)
    {
        using var tx = _connection.BeginTransaction();
        try
        {
            // Both updates share one transaction boundary.
            await DebitAsync(fromId, amount, tx);
            await CreditAsync(toId, amount, tx);
            tx.Commit();
        }
        catch
        {
            tx.Rollback();
            throw;
        }
    }

    private Task DebitAsync(int accountId, decimal amount, IDbTransaction tx) => Task.CompletedTask;
    private Task CreditAsync(int accountId, decimal amount, IDbTransaction tx) => Task.CompletedTask;
}
```

#### Q_LEVEL3_272: Apply optimistic locking.

**Question:**
en: Apply optimistic locking to prevent lost updates.
vi: Áp dụng **optimistic locking** để tránh lost update.

**Answer:**
en: Store a version number and update the row only if the version has not changed. If no row is affected, reload and retry or return a conflict.
vi: **Vấn đề:** Hai request cùng sửa một bản ghi có thể ghi đè nhau. **Giải pháp:** Dùng cột version; update chỉ thành công khi version hiện tại vẫn đúng.

```csharp
public sealed record Product(int Id, string Name, int Version);

public static class OptimisticLocking
{
    public static string BuildUpdateSql()
    {
        return """
        UPDATE Products
        SET Name = @Name, Version = Version + 1
        WHERE Id = @Id AND Version = @ExpectedVersion
        """;
        // If affected rows = 0, another transaction changed the row.
    }
}
```

#### Q_LEVEL3_383: Use an isolation level.

**Question:**
en: Show how to choose an isolation level for a transaction.
vi: Minh họa cách chọn **isolation level** cho transaction.

**Answer:**
en: Choose the weakest isolation level that protects the business rule. Stronger isolation can reduce concurrency, so it should be intentional.
vi: **Vấn đề:** Isolation quá yếu có thể sai dữ liệu, quá mạnh có thể giảm throughput. **Giải pháp:** Chọn mức thấp nhất vẫn bảo vệ được rule nghiệp vụ.

```csharp
using System.Data;

public static class TransactionFactory
{
    public static IDbTransaction BeginSerializable(IDbConnection connection)
    {
        // Serializable protects strict invariants but can increase contention.
        return connection.BeginTransaction(IsolationLevel.Serializable);
    }
}
```

#### Q_LEVEL3_494: Implement retry after deadlock.

**Question:**
en: Apply retry logic for deadlock or serialization failures.
vi: Áp dụng retry logic cho **deadlock** hoặc lỗi serialization.

**Answer:**
en: Deadlocks are often transient. Retry with a small limit and backoff, but only when the operation is safe to retry.
vi: **Vấn đề:** Database có thể abort transaction do deadlock. **Giải pháp:** Retry có giới hạn và backoff, chỉ áp dụng cho operation idempotent hoặc được thiết kế retry-safe.

```csharp
using System;
using System.Threading.Tasks;

public static class TransactionRetry
{
    public static async Task ExecuteAsync(Func<Task> action)
    {
        for (var attempt = 1; ; attempt++)
        {
            try
            {
                await action();
                return;
            }
            catch (Exception) when (attempt < 3)
            {
                await Task.Delay(100 * attempt);
            }
        }
    }
}
```

#### Q_LEVEL3_505: Validate consistency before commit.

**Question:**
en: Apply a consistency check before committing a transaction.
vi: Áp dụng kiểm tra **Consistency** trước khi commit transaction.

**Answer:**
en: Validate business invariants before commit, such as "balance must not be negative". If the invariant fails, rollback by throwing an error.
vi: **Vấn đề:** Constraint kỹ thuật không phải lúc nào cũng đủ bảo vệ rule nghiệp vụ. **Giải pháp:** Kiểm tra invariant như số dư không âm trước khi commit; sai thì throw để rollback.

```csharp
public static class AccountRules
{
    public static void EnsureValidBalance(decimal balance)
    {
        if (balance < 0)
        {
            // Throwing inside the transaction causes the caller to rollback.
            throw new InvalidOperationException("Account balance cannot be negative.");
        }
    }
}
```
