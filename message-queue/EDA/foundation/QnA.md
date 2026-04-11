# Event-Driven Architecture (EDA) Foundation Q&A

### Level 1: Remembering

#### Q_LEVEL1_104: What is Event-Driven Architecture?

**Question:**
en: What is Event-Driven Architecture (EDA)?
vi: Kiến trúc hướng sự kiện (Event-Driven Architecture - EDA) là gì?

**Answer:**
en: Event-Driven Architecture is a design style where services communicate by producing and consuming events that describe something that already happened in the system.
vi: EDA là một kiểu kiến trúc trong đó các thành phần giao tiếp với nhau bằng cách phát ra và xử lý các event mô tả một việc đã xảy ra trong hệ thống.

#### Q_LEVEL1_117: What is an event?

**Question:**
en: What is an event in an EDA system?
vi: Event trong một hệ thống EDA là gì?

**Answer:**
en: An event is an immutable record of a fact, such as `OrderCreated` or `PaymentFailed`, usually containing metadata and business data needed by consumers.
vi: Event là một bản ghi bất biến về một sự việc đã xảy ra, ví dụ `OrderCreated` hoặc `PaymentFailed`, thường chứa metadata và dữ liệu nghiệp vụ để consumer xử lý.

#### Q_LEVEL1_128: What is an event producer?

**Question:**
en: What is an event producer?
vi: Event producer là gì?

**Answer:**
en: An event producer, or publisher, is the component that emits events when business actions occur.
vi: Event producer, hay publisher, là thành phần phát ra event khi một hành động nghiệp vụ xảy ra.

#### Q_LEVEL1_136: What is an event consumer?

**Question:**
en: What is an event consumer?
vi: Event consumer là gì?

**Answer:**
en: An event consumer, or subscriber, is the component that receives and processes events produced by another component.
vi: Event consumer, hay subscriber, là thành phần nhận và xử lý event do thành phần khác phát ra.

#### Q_LEVEL1_142: What is a message broker?

**Question:**
en: What is a message broker in EDA?
vi: Message broker trong EDA là gì?

**Answer:**
en: A message broker is middleware such as Kafka, RabbitMQ, or Azure Service Bus that routes, stores, and delivers messages between producers and consumers.
vi: Message broker là middleware như Kafka, RabbitMQ hoặc Azure Service Bus, có nhiệm vụ định tuyến, lưu tạm và chuyển message giữa producer và consumer.

#### Q_LEVEL1_155: What is the difference between an event and a command?

**Question:**
en: What is the difference between an event and a command?
vi: Khác nhau giữa event và command là gì?

**Answer:**
en: A command expresses intent and asks something to happen, while an event reports that something already happened.
vi: Command thể hiện ý định và yêu cầu một việc xảy ra, còn event chỉ thông báo rằng một việc đã xảy ra rồi.

#### Q_LEVEL1_168: What is a queue?

**Question:**
en: What is a queue in messaging systems?
vi: Queue trong hệ thống message là gì?

**Answer:**
en: A queue is a channel where messages are typically consumed by one consumer instance from a competing group, which is useful for work distribution.
vi: Queue là một kênh mà message thường được một consumer instance trong nhóm cạnh tranh lấy ra xử lý, phù hợp cho phân phối công việc.

#### Q_LEVEL1_173: What is a topic?

**Question:**
en: What is a topic in messaging systems?
vi: Topic trong hệ thống message là gì?

**Answer:**
en: A topic is a publish-subscribe channel where the same message can be delivered to multiple independent subscribers.
vi: Topic là một kênh publish-subscribe, nơi cùng một message có thể được gửi đến nhiều subscriber độc lập.

#### Q_LEVEL1_184: What is publish-subscribe?

**Question:**
en: What is the publish-subscribe pattern?
vi: Publish-subscribe pattern là gì?

**Answer:**
en: Publish-subscribe is a messaging pattern where publishers send messages without knowing specific subscribers, and subscribers receive messages based on subscriptions.
vi: Publish-subscribe là mẫu giao tiếp mà publisher gửi message mà không cần biết subscriber cụ thể là ai, còn subscriber nhận message dựa trên đăng ký của mình.

#### Q_LEVEL1_196: What is eventual consistency?

**Question:**
en: What is eventual consistency?
vi: Eventual consistency là gì?

**Answer:**
en: Eventual consistency means different parts of the system may not be updated immediately, but they will converge to the correct state over time.
vi: Eventual consistency nghĩa là các phần của hệ thống có thể chưa đồng bộ ngay lập tức, nhưng theo thời gian sẽ hội tụ về trạng thái đúng.

#### Q_LEVEL1_205: What is idempotency?

**Question:**
en: What is idempotency in message processing?
vi: Idempotency trong xử lý message là gì?

**Answer:**
en: Idempotency means processing the same message multiple times results in the same final state as processing it once.
vi: Idempotency nghĩa là xử lý cùng một message nhiều lần vẫn cho ra trạng thái cuối cùng giống như chỉ xử lý một lần.

#### Q_LEVEL1_214: What is at-least-once delivery?

**Question:**
en: What is at-least-once delivery?
vi: At-least-once delivery là gì?

**Answer:**
en: It is a delivery guarantee where each message will be delivered one or more times, so consumers must handle duplicates safely.
vi: Đây là cơ chế đảm bảo mỗi message sẽ được gửi ít nhất một lần, nên consumer phải chịu được việc nhận trùng lặp.

#### Q_LEVEL1_226: What is dead-letter queue?

**Question:**
en: What is a dead-letter queue (DLQ)?
vi: Dead-letter queue (DLQ) là gì?

**Answer:**
en: A dead-letter queue is a holding area for messages that could not be processed successfully after retries or validation failures.
vi: Dead-letter queue là nơi chứa các message không thể xử lý thành công sau nhiều lần retry hoặc do lỗi validation.

#### Q_LEVEL1_238: What is message ordering?

**Question:**
en: What does message ordering mean?
vi: Message ordering nghĩa là gì?

**Answer:**
en: Message ordering means consumers receive and process messages in a specific sequence, often important when later events depend on earlier ones.
vi: Message ordering nghĩa là consumer nhận và xử lý message theo một thứ tự xác định, thường quan trọng khi event sau phụ thuộc vào event trước.

#### Q_LEVEL1_247: What is a saga?

**Question:**
en: What is a saga in distributed systems?
vi: Saga trong hệ thống phân tán là gì?

**Answer:**
en: A saga is a sequence of local transactions across services, with compensating actions used when one step fails.
vi: Saga là chuỗi các local transaction giữa nhiều service, có kèm các hành động bù trừ khi một bước thất bại.

#### Q_LEVEL1_259: What is the outbox pattern?

**Question:**
en: What is the outbox pattern?
vi: Outbox pattern là gì?

**Answer:**
en: The outbox pattern stores domain changes and outgoing events in the same local transaction, then publishes the events asynchronously later.
vi: Outbox pattern lưu thay đổi nghiệp vụ và event cần phát trong cùng một local transaction, rồi publish event bất đồng bộ sau đó.

#### Q_LEVEL1_267: What is a retry policy?

**Question:**
en: What is a retry policy in event processing?
vi: Retry policy trong xử lý event là gì?

**Answer:**
en: A retry policy defines how many times, how often, and under what conditions a failed message should be retried.
vi: Retry policy xác định số lần retry, khoảng cách giữa các lần retry và điều kiện nào thì một message thất bại sẽ được thử lại.

#### Q_LEVEL1_278: What is correlation ID?

**Question:**
en: What is a correlation ID?
vi: Correlation ID là gì?

**Answer:**
en: A correlation ID is an identifier used to trace related messages and operations across distributed services.
vi: Correlation ID là định danh dùng để theo dõi các message và thao tác có liên quan với nhau xuyên suốt nhiều service.

#### Q_LEVEL1_286: What is event schema?

**Question:**
en: What is an event schema?
vi: Event schema là gì?

**Answer:**
en: An event schema defines the structure of an event, including field names, data types, and semantic meaning.
vi: Event schema định nghĩa cấu trúc của event, gồm tên trường, kiểu dữ liệu và ý nghĩa nghiệp vụ của từng trường.

#### Q_LEVEL1_299: What is event versioning?

**Question:**
en: What is event versioning?
vi: Event versioning là gì?

**Answer:**
en: Event versioning is the practice of evolving event contracts over time without breaking existing consumers.
vi: Event versioning là cách tiến hóa contract của event theo thời gian mà không làm hỏng các consumer hiện tại.

---

### Level 2: Understanding

#### Q_LEVEL2_311: Why do teams use EDA?

**Question:**
en: Why do teams adopt Event-Driven Architecture?
vi: Vì sao các team lại áp dụng Event-Driven Architecture?

**Answer:**
en: Teams use EDA to decouple services, improve scalability, support asynchronous workflows, and allow multiple consumers to react to the same business event independently.
vi: Các team dùng EDA để giảm coupling giữa service, tăng khả năng scale, hỗ trợ luồng xử lý bất đồng bộ và cho phép nhiều consumer phản ứng độc lập với cùng một business event.

#### Q_LEVEL2_324: Queue vs topic

**Question:**
en: Explain the practical difference between a queue and a topic.
vi: Giải thích sự khác nhau thực tế giữa queue và topic.

**Answer:**
en: A queue is mainly for distributing work so one message is handled by one worker, while a topic is for broadcasting an event so many subscribers can react to it in their own way.
vi: Queue chủ yếu dùng để chia việc, nghĩa là một message được một worker xử lý. Topic dùng để phát tán sự kiện để nhiều subscriber cùng phản ứng theo logic riêng của họ.

#### Q_LEVEL2_337: Publisher and subscriber coupling

**Question:**
en: How does EDA reduce direct coupling between publishers and subscribers?
vi: EDA giảm coupling trực tiếp giữa publisher và subscriber như thế nào?

**Answer:**
en: Publishers emit events without calling subscribers directly, so they do not need to know who listens, where they run, or how many consumers exist.
vi: Publisher chỉ phát event chứ không gọi trực tiếp subscriber, nên không cần biết ai đang lắng nghe, chạy ở đâu hay có bao nhiêu consumer tồn tại.

#### Q_LEVEL2_348: Eventual consistency trade-off

**Question:**
en: Why is eventual consistency a common trade-off in EDA?
vi: Vì sao eventual consistency là trade-off phổ biến trong EDA?

**Answer:**
en: Because distributed consumers process events asynchronously, state changes across services happen at different times, so temporary inconsistency is accepted in exchange for decoupling and availability.
vi: Vì consumer ở các service xử lý event bất đồng bộ nên việc cập nhật trạng thái xảy ra ở các thời điểm khác nhau. Hệ thống chấp nhận độ lệch tạm thời để đổi lấy tính tách rời và tính sẵn sàng cao hơn.

#### Q_LEVEL2_359: Why duplicates happen

**Question:**
en: Why can duplicate event processing happen even in a healthy system?
vi: Vì sao xử lý event trùng lặp vẫn có thể xảy ra ngay cả khi hệ thống đang hoạt động bình thường?

**Answer:**
en: Duplicates happen when brokers retry delivery, acknowledgments are lost, or a consumer crashes after applying business logic but before confirming completion.
vi: Trùng lặp xảy ra khi broker retry, tín hiệu xác nhận bị mất, hoặc consumer bị crash sau khi đã chạy logic nghiệp vụ nhưng trước khi xác nhận hoàn tất.

#### Q_LEVEL2_364: Ordering challenges

**Question:**
en: Why is preserving global message ordering difficult in distributed systems?
vi: Vì sao giữ thứ tự message toàn cục lại khó trong hệ thống phân tán?

**Answer:**
en: Parallel consumers, partitions, network delays, and retries all affect arrival and processing order, so strict global ordering usually reduces throughput significantly.
vi: Consumer chạy song song, partition, độ trễ mạng và retry đều làm thay đổi thời điểm nhận và xử lý message, nên thứ tự toàn cục rất khó giữ và thường phải đánh đổi throughput lớn.

#### Q_LEVEL2_378: DLQ purpose

**Question:**
en: Why is a dead-letter queue important instead of retrying forever?
vi: Vì sao dead-letter queue quan trọng hơn việc retry vô hạn?

**Answer:**
en: Infinite retries can block resources, create noise, and hide poison messages. A DLQ isolates failures so teams can inspect, replay, or discard them safely.
vi: Retry vô hạn có thể giữ tài nguyên, tạo nhiễu và che giấu poison message. DLQ tách riêng lỗi để team có thể kiểm tra, replay hoặc loại bỏ một cách an toàn.

#### Q_LEVEL2_389: Choreography vs orchestration

**Question:**
en: Explain the difference between saga choreography and saga orchestration.
vi: Giải thích khác nhau giữa saga choreography và saga orchestration.

**Answer:**
en: In choreography, services react to each other's events without a central coordinator. In orchestration, a dedicated component tells each service what step to execute next.
vi: Với choreography, các service tự phản ứng với event của nhau mà không có điều phối trung tâm. Với orchestration, có một thành phần chuyên điều phối và ra lệnh bước tiếp theo cho từng service.

#### Q_LEVEL2_397: Outbox value

**Question:**
en: Why does the outbox pattern improve reliability?
vi: Vì sao outbox pattern giúp tăng độ tin cậy?

**Answer:**
en: It avoids the classic problem where database changes succeed but event publishing fails, or the reverse, by making the local state change and outbox write atomic.
vi: Nó tránh lỗi kinh điển là cập nhật database thành công nhưng publish event thất bại, hoặc ngược lại, bằng cách ghi thay đổi nghiệp vụ và bản ghi outbox trong cùng một transaction cục bộ.

#### Q_LEVEL2_408: Observability in EDA

**Question:**
en: Why is observability especially important in Event-Driven Architecture?
vi: Vì sao observability đặc biệt quan trọng trong Event-Driven Architecture?

**Answer:**
en: EDA breaks one user action into many asynchronous steps across services, so logs, metrics, tracing, and correlation IDs are needed to understand what happened end to end.
vi: EDA biến một thao tác của người dùng thành nhiều bước bất đồng bộ qua nhiều service, nên cần log, metric, tracing và correlation ID để lần ra toàn bộ luồng xử lý.

#### Q_LEVEL2_419: When not to use EDA

**Question:**
en: Why might a team choose not to use EDA for a given problem?
vi: Vì sao một team có thể quyết định không dùng EDA cho một bài toán?

**Answer:**
en: EDA adds operational complexity, async debugging, duplicate handling, schema evolution, and eventual consistency. For simple tightly coupled workflows, synchronous calls can be easier and safer.
vi: EDA làm tăng độ phức tạp vận hành, khó debug bất đồng bộ, phải xử lý duplicate, version schema và eventual consistency. Với bài toán nhỏ, phụ thuộc chặt và ít service, gọi đồng bộ có thể đơn giản và an toàn hơn.

#### Q_LEVEL2_438: Event contract stability

**Question:**
en: Why should event contracts be treated carefully once published?
vi: Vì sao event contract cần được đối xử cẩn thận sau khi đã publish?

**Answer:**
en: Once consumers depend on an event shape, incompatible changes can break production flows unexpectedly. Event contracts are integration boundaries, not just internal classes.
vi: Khi consumer đã phụ thuộc vào cấu trúc event, thay đổi không tương thích có thể làm hỏng luồng production rất khó phát hiện. Event contract là ranh giới tích hợp, không chỉ là một class nội bộ.

#### Q_LEVEL2_446: Retries and transient failures

**Question:**
en: How do retries help with transient failures, and what is the risk?
vi: Retry giúp gì với transient failure, và rủi ro là gì?

**Answer:**
en: Retries help recover from temporary issues like timeouts or broker hiccups, but if used blindly they can amplify load, duplicate side effects, and delay detection of permanent failures.
vi: Retry giúp vượt qua lỗi tạm thời như timeout hoặc broker chập chờn, nhưng nếu dùng mù quáng thì có thể khuếch đại tải, tạo tác dụng phụ lặp lại và làm chậm việc phát hiện lỗi vĩnh viễn.

#### Q_LEVEL2_457: Competing consumers

**Question:**
en: Explain the competing consumers pattern.
vi: Giải thích competing consumers pattern.

**Answer:**
en: Competing consumers means multiple worker instances read from the same queue and share the workload, which improves throughput and scalability.
vi: Competing consumers là mô hình nhiều worker instance cùng đọc từ một queue và chia nhau khối lượng công việc, giúp tăng throughput và khả năng scale.

#### Q_LEVEL2_479: Schema evolution

**Question:**
en: How should teams think about schema evolution for events?
vi: Team nên nghĩ về schema evolution cho event như thế nào?

**Answer:**
en: Teams should prefer backward-compatible changes, add fields carefully, avoid breaking existing meanings, and version events when incompatible changes are unavoidable.
vi: Team nên ưu tiên thay đổi tương thích ngược, thêm field có kiểm soát, tránh đổi nghĩa dữ liệu cũ và dùng version khi không thể tránh thay đổi phá vỡ tương thích.

---

### Level 3: Applying

#### Q_LEVEL3_503: Implement idempotent consumer logic

**Question:**
en: How would you implement an idempotent consumer so duplicate messages do not create duplicate side effects?
vi: Bạn sẽ triển khai idempotent consumer như thế nào để message trùng lặp không tạo ra side effect lặp lại?

**Answer:**
en: A common approach is to keep a durable record of processed message IDs and check it before applying business logic. The check and the business update should be done atomically where possible.
vi: Cách phổ biến là lưu bền vững danh sách message ID đã xử lý và kiểm tra trước khi chạy business logic. Tốt nhất phần kiểm tra và cập nhật nghiệp vụ nên nằm trong cùng một transaction cục bộ nếu làm được.

```csharp
public sealed class OrderPaidConsumer
{
    private readonly IProcessedMessageStore _processedStore;
    private readonly IOrderRepository _orders;

    public OrderPaidConsumer(IProcessedMessageStore processedStore, IOrderRepository orders)
    {
        _processedStore = processedStore;
        _orders = orders;
    }

    public async Task HandleAsync(OrderPaidEvent message, CancellationToken cancellationToken)
    {
        // Skip duplicate messages that were already processed.
        if (await _processedStore.ExistsAsync(message.MessageId, cancellationToken))
        {
            return;
        }

        var order = await _orders.GetByIdAsync(message.OrderId, cancellationToken);
        order.MarkAsPaid(message.PaidAtUtc);

        // Persist both the domain change and processed marker together if possible.
        await _orders.SaveAsync(order, cancellationToken);
        await _processedStore.RecordAsync(message.MessageId, cancellationToken);
    }
}
```

#### Q_LEVEL3_516: Apply the outbox pattern

**Question:**
en: Show how you would apply the outbox pattern when an order is created.
vi: Hãy cho thấy bạn sẽ áp dụng outbox pattern như thế nào khi tạo đơn hàng.

**Answer:**
en: The service should save the order and an outbox record in the same database transaction. A separate publisher process then reads unsent outbox rows and publishes them to the broker.
vi: Service nên lưu đơn hàng và một bản ghi outbox trong cùng transaction database. Sau đó một tiến trình publisher riêng đọc các dòng outbox chưa gửi và phát chúng lên broker.

```csharp
public async Task<Guid> CreateOrderAsync(CreateOrderCommand command, CancellationToken cancellationToken)
{
    var order = Order.Create(command.CustomerId, command.Items);
    var integrationEvent = new OutboxMessage(
        id: Guid.NewGuid(),
        type: "OrderCreated",
        payload: JsonSerializer.Serialize(new
        {
            order.Id,
            order.CustomerId,
            order.TotalAmount
        }),
        occurredAtUtc: DateTime.UtcNow);

    await using var transaction = await _dbContext.Database.BeginTransactionAsync(cancellationToken);

    _dbContext.Orders.Add(order);
    _dbContext.OutboxMessages.Add(integrationEvent);

    await _dbContext.SaveChangesAsync(cancellationToken);
    await transaction.CommitAsync(cancellationToken);

    return order.Id;
}
```

#### Q_LEVEL3_528: Use retry with backoff

**Question:**
en: How would you apply retries with backoff for transient consumer failures?
vi: Bạn sẽ áp dụng retry kèm backoff như thế nào cho lỗi tạm thời của consumer?

**Answer:**
en: Retries should target transient failures only, with bounded attempts and increasing delays. Permanent failures should be moved to a DLQ instead of being retried endlessly.
vi: Retry chỉ nên áp dụng cho lỗi tạm thời, có giới hạn số lần thử và tăng dần thời gian chờ. Lỗi vĩnh viễn nên được chuyển sang DLQ thay vì retry vô hạn.

```csharp
public async Task HandleWithRetryAsync(Func<CancellationToken, Task> action, CancellationToken cancellationToken)
{
    var delays = new[]
    {
        TimeSpan.FromSeconds(1),
        TimeSpan.FromSeconds(3),
        TimeSpan.FromSeconds(10)
    };

    foreach (var delay in delays)
    {
        try
        {
            await action(cancellationToken);
            return;
        }
        catch (TimeoutException) when (!cancellationToken.IsCancellationRequested)
        {
            // Retry only transient timeout failures.
            await Task.Delay(delay, cancellationToken);
        }
    }

    throw new InvalidOperationException("Message processing exceeded retry policy.");
}
```

#### Q_LEVEL3_541: Carry correlation IDs

**Question:**
en: How would you carry a correlation ID through an event flow for tracing?
vi: Bạn sẽ truyền correlation ID qua luồng event như thế nào để trace?

**Answer:**
en: Generate or reuse a correlation ID at the entry point, attach it to message metadata, and include it in logs and traces at each producer and consumer hop.
vi: Hãy tạo hoặc tái sử dụng correlation ID ở điểm vào hệ thống, gắn nó vào metadata của message và đưa nó vào log cũng như trace ở mọi producer và consumer.

```csharp
public sealed record IntegrationMessage<T>(
    Guid MessageId,
    string CorrelationId,
    DateTime OccurredAtUtc,
    T Payload);

public async Task PublishOrderCreatedAsync(Order order, string correlationId, CancellationToken cancellationToken)
{
    var message = new IntegrationMessage<object>(
        MessageId: Guid.NewGuid(),
        CorrelationId: correlationId,
        OccurredAtUtc: DateTime.UtcNow,
        Payload: new
        {
            order.Id,
            order.TotalAmount
        });

    _logger.LogInformation("Publishing OrderCreated with CorrelationId {CorrelationId}", correlationId);
    await _broker.PublishAsync("orders.created", message, cancellationToken);
}
```

#### Q_LEVEL3_557: Build a basic saga step

**Question:**
en: How would you model a basic saga step for order creation followed by payment reservation?
vi: Bạn sẽ mô hình hóa một bước saga cơ bản cho luồng tạo đơn hàng rồi giữ chỗ thanh toán như thế nào?

**Answer:**
en: The service should react to `OrderCreated`, attempt payment reservation, and publish either a success event or a compensating failure event so downstream services can continue consistently.
vi: Service nên phản ứng với `OrderCreated`, thử giữ chỗ thanh toán rồi publish event thành công hoặc event thất bại mang tính bù trừ để các service phía sau tiếp tục xử lý nhất quán.

```csharp
public async Task HandleAsync(OrderCreatedEvent message, CancellationToken cancellationToken)
{
    var reserved = await _paymentGateway.TryReserveAsync(
        message.OrderId,
        message.TotalAmount,
        cancellationToken);

    if (reserved)
    {
        await _broker.PublishAsync(
            "payments.reserved",
            new PaymentReservedEvent(message.OrderId, message.CustomerId),
            cancellationToken);
        return;
    }

    // Publish a compensating event so the order workflow can react.
    await _broker.PublishAsync(
        "payments.failed",
        new PaymentFailedEvent(message.OrderId, "Reservation failed"),
        cancellationToken);
}
```
