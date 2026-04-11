# Event-Driven Architecture (EDA) Pitfalls Q&A

### Level 1: Remembering

#### Q_LEVEL1_811: What is the common mistake with event schema changes?

**Question:**
en: What is a common mistake when changing an event schema in EDA?
vi: Lỗi phổ biến khi thay đổi event schema trong EDA là gì?

**Answer:**
en: A common mistake is changing or removing fields in an existing event without considering old consumers. That can break downstream services unexpectedly. Safer systems version contracts and keep backward compatibility whenever possible.
vi: Lỗi rất phổ biến là thay đổi hoặc xóa field trong event hiện có mà không nghĩ đến các consumer cũ. Điều đó dễ làm hỏng downstream một cách âm thầm. Cách an toàn hơn là version contract và cố giữ backward compatibility khi có thể.

#### Q_LEVEL1_823: What is the common idempotency mistake?

**Question:**
en: What is a common idempotency mistake in event consumers?
vi: Lỗi idempotency thường gặp trong event consumer là gì?

**Answer:**
en: A common mistake is assuming the broker delivers each message only once, then writing consumers that create side effects every time they receive the event. In real systems, duplicates happen and consumers must be safe to run more than once.
vi: Lỗi hay gặp là nghĩ broker chỉ giao mỗi message đúng một lần, rồi viết consumer tạo side effect mỗi lần nhận event. Trong thực tế, message trùng lặp vẫn xảy ra nên consumer phải an toàn ngay cả khi bị chạy nhiều lần.

#### Q_LEVEL1_836: What is a retry storm?

**Question:**
en: What is a retry storm in Event-Driven Architecture?
vi: Retry storm trong Event-Driven Architecture là gì?

**Answer:**
en: A retry storm happens when many failed messages are retried aggressively at the same time, creating more load on an already unhealthy dependency and making recovery harder.
vi: Retry storm xảy ra khi nhiều message lỗi bị retry quá gắt cùng lúc, làm tăng thêm tải lên dependency vốn đã không khỏe và khiến việc hồi phục khó hơn.

#### Q_LEVEL1_849: What is the misconception about exactly-once processing?

**Question:**
en: What is a common misconception about exactly-once processing?
vi: Hiểu lầm phổ biến về exactly-once processing là gì?

**Answer:**
en: The misconception is believing the broker alone guarantees exactly-once business outcomes. In practice, databases, external APIs, and side effects still require idempotent handling.
vi: Hiểu lầm phổ biến là nghĩ chỉ cần broker là đủ để đảm bảo kết quả nghiệp vụ exactly-once. Thực tế, database, external API và các side effect vẫn buộc hệ thống phải xử lý idempotent.

---

### Level 2: Understanding

#### Q_LEVEL2_861: Why is no DLQ strategy a serious production trap?

**Question:**
en: Why is running EDA without a dead-letter queue strategy dangerous?
vi: Vì sao vận hành EDA mà không có chiến lược dead-letter queue lại nguy hiểm?

**Answer:**
en: Without a DLQ strategy, poison messages can loop forever, block throughput, or disappear after repeated failures depending on the platform. A mature design defines when to retry, when to stop, how to inspect failed payloads, and how to replay safely after a fix.
vi: Nếu không có chiến lược DLQ, poison message có thể bị lặp vô hạn, chặn throughput hoặc biến mất sau nhiều lần thất bại tùy nền tảng. Thiết kế trưởng thành phải xác định rõ khi nào retry, khi nào dừng, cách kiểm tra payload lỗi và cách replay an toàn sau khi đã sửa.

#### Q_LEVEL2_874: Why are ordering assumptions dangerous?

**Question:**
en: Why is assuming events always arrive in order a common EDA mistake?
vi: Vì sao giả định event luôn đến đúng thứ tự là một lỗi rất thường gặp trong EDA?

**Answer:**
en: Ordering guarantees are usually narrower than teams expect. Messages may be reordered across partitions, consumer groups, retries, or replay flows. If business logic implicitly depends on global ordering, production bugs appear when scale increases or failures happen.
vi: Cam kết về ordering thường hẹp hơn nhiều so với điều team tưởng tượng. Message có thể bị đảo thứ tự giữa các partition, consumer group, retry hoặc replay. Nếu business logic ngầm phụ thuộc vào global ordering thì bug production rất dễ xuất hiện khi hệ thống scale hoặc gặp sự cố.

#### Q_LEVEL2_887: Why is missing observability a major EDA failure mode?

**Question:**
en: Why does weak observability become a major problem in Event-Driven Architecture?
vi: Vì sao observability yếu lại trở thành vấn đề lớn trong Event-Driven Architecture?

**Answer:**
en: EDA spreads one business flow across multiple asynchronous hops, so failures are harder to see than in direct request-response systems. Without correlation IDs, consumer lag metrics, retry counts, DLQ visibility, and traceable event history, incidents turn into guesswork.
vi: EDA chia một luồng nghiệp vụ thành nhiều bước bất đồng bộ nên lỗi khó nhìn thấy hơn nhiều so với hệ thống request-response trực tiếp. Nếu thiếu correlation ID, metric về consumer lag, số lần retry, khả năng quan sát DLQ và lịch sử event có thể trace, việc xử lý sự cố gần như chỉ còn là đoán mò.

#### Q_LEVEL2_899: Why can too much choreography become an anti-pattern?

**Question:**
en: Why can overusing choreography become an anti-pattern in EDA?
vi: Vì sao lạm dụng choreography có thể trở thành anti-pattern trong EDA?

**Answer:**
en: Choreography looks elegant at first because services react independently, but complex workflows can become invisible and hard to reason about. When too many services trigger each other indirectly, ownership blurs, change impact becomes unpredictable, and debugging turns expensive.
vi: Choreography ban đầu trông rất đẹp vì các service phản ứng độc lập, nhưng workflow phức tạp dễ trở nên vô hình và khó suy luận. Khi quá nhiều service kích hoạt lẫn nhau một cách gián tiếp, ownership bị mờ, tác động của thay đổi khó dự đoán và debug trở nên rất tốn kém.

#### Q_LEVEL2_912: Why is skipping the outbox pattern risky?

**Question:**
en: Why is publishing events directly after a database write a risky shortcut?
vi: Vì sao publish event trực tiếp sau khi ghi database là một đường tắt đầy rủi ro?

**Answer:**
en: If the database commit succeeds but event publishing fails, other services never learn about the business change. If publishing succeeds but the transaction later fails, consumers react to something that never really happened. The outbox pattern reduces this dual-write inconsistency.
vi: Nếu commit database thành công nhưng publish event thất bại, các service khác sẽ không bao giờ biết thay đổi nghiệp vụ đó. Nếu publish thành công nhưng transaction sau đó lại fail, consumer sẽ phản ứng với một việc thực ra chưa từng xảy ra. Outbox pattern giúp giảm rủi ro dual-write inconsistency này.

#### Q_LEVEL2_925: Why is weak event ownership a governance problem?

**Question:**
en: Why is unclear ownership of events and schemas a long-term EDA problem?
vi: Vì sao ownership không rõ ràng cho event và schema lại là một vấn đề dài hạn trong EDA?

**Answer:**
en: When nobody clearly owns an event contract, teams change payloads casually, add fields with unclear semantics, and break consumers without accountability. Strong governance defines who owns the schema, who approves changes, and how compatibility is validated before rollout.
vi: Khi không ai thực sự sở hữu event contract, các team rất dễ sửa payload tùy tiện, thêm field với ý nghĩa mơ hồ và làm hỏng consumer mà không có trách nhiệm rõ ràng. Governance tốt phải xác định ai sở hữu schema, ai phê duyệt thay đổi và kiểm tra compatibility ra sao trước khi rollout.

---

### Level 3: Applying

#### Q_LEVEL3_941: Apply a safer idempotent consumer design

**Question:**
en: How would you make a C# event consumer safer against duplicate delivery and duplicate side effects?
vi: Bạn sẽ làm gì để một C# event consumer an toàn hơn trước việc nhận trùng event và tạo side effect trùng lặp?

**Answer:**
en: Persist a stable event ID or business operation key before applying side effects, and skip processing if that key already exists. The important point is to make duplicate detection part of the same consistency boundary as the side effect when possible.
vi: Hãy lưu một event ID ổn định hoặc business operation key trước khi tạo side effect, và bỏ qua nếu key đó đã tồn tại. Điểm quan trọng là cơ chế phát hiện trùng lặp nên nằm trong cùng ranh giới nhất quán với side effect khi có thể.

```csharp
public sealed class OrderPaidConsumer
{
    private readonly PaymentsDbContext _db;

    public OrderPaidConsumer(PaymentsDbContext db)
    {
        _db = db;
    }

    public async Task HandleAsync(OrderPaidIntegrationEvent message, CancellationToken cancellationToken)
    {
        var alreadyProcessed = await _db.ProcessedMessages
            .AnyAsync(x => x.MessageId == message.MessageId, cancellationToken);

        if (alreadyProcessed)
        {
            return;
        }

        _db.ProcessedMessages.Add(new ProcessedMessage
        {
            MessageId = message.MessageId,
            ProcessedAtUtc = DateTime.UtcNow
        });

        _db.LedgerEntries.Add(new LedgerEntry
        {
            OrderId = message.OrderId,
            Amount = message.Amount
        });

        await _db.SaveChangesAsync(cancellationToken);
    }
}
```

#### Q_LEVEL3_956: Apply a retry and poison-message policy

**Question:**
en: How would you handle retries without creating a retry storm or endless poison-message loops?
vi: Bạn sẽ xử lý retry như thế nào để tránh retry storm hoặc vòng lặp poison message vô tận?

**Answer:**
en: Separate transient failures from invalid messages, use bounded retries with backoff, and move unrecoverable messages to a DLQ with enough context for investigation. Retrying everything aggressively is the mistake; controlled retry plus DLQ is the safer operating model.
vi: Hãy tách lỗi tạm thời khỏi message không hợp lệ, dùng retry có giới hạn với backoff, và chuyển message không thể phục hồi sang DLQ kèm đủ context để điều tra. Lỗi thường gặp là retry tất cả thật gắt; mô hình an toàn hơn là retry có kiểm soát kết hợp với DLQ.

```csharp
public async Task HandleAsync(InventoryReservedIntegrationEvent message, CancellationToken cancellationToken)
{
    if (message.Quantity <= 0)
    {
        await _deadLetterWriter.WriteAsync(message, "Invalid quantity", cancellationToken);
        return;
    }

    try
    {
        await _inventoryProjector.ApplyAsync(message, cancellationToken);
    }
    catch (HttpRequestException) when (message.RetryCount < 5)
    {
        await _retryScheduler.ScheduleAsync(
            message with { RetryCount = message.RetryCount + 1 },
            delay: TimeSpan.FromSeconds(Math.Pow(2, message.RetryCount)),
            cancellationToken);
    }
}
```

#### Q_LEVEL3_971: Apply a safer event publishing approach

**Question:**
en: How would you avoid the common dual-write mistake when saving business data and publishing integration events?
vi: Bạn sẽ tránh lỗi dual-write phổ biến như thế nào khi vừa lưu business data vừa publish integration event?

**Answer:**
en: Write the business change and the outgoing event record in one local transaction, then publish from the outbox asynchronously. This avoids the fragile pattern where database state and published events can diverge.
vi: Hãy ghi thay đổi nghiệp vụ và bản ghi event outgoing trong cùng một local transaction, rồi publish từ outbox theo cách bất đồng bộ. Cách này tránh được pattern mong manh khi trạng thái database và event đã publish có thể lệch nhau.

```csharp
public async Task PlaceOrderAsync(CreateOrderCommand command, CancellationToken cancellationToken)
{
    var order = Order.Create(command.CustomerId, command.Items);

    _db.Orders.Add(order);
    _db.OutboxMessages.Add(new OutboxMessage
    {
        Id = Guid.NewGuid(),
        Type = "OrderPlaced",
        Payload = JsonSerializer.Serialize(new
        {
            order.Id,
            order.CustomerId,
            order.TotalAmount
        }),
        OccurredAtUtc = DateTime.UtcNow
    });

    await _db.SaveChangesAsync(cancellationToken);
}
```
