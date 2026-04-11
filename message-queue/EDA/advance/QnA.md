# Event-Driven Architecture (EDA) Advance Q&A

### Level 4: Analyzing

#### Q_LEVEL4_604: Analyze when EDA improves a system

**Question:**
en: Analyze when Event-Driven Architecture is a strong fit and when it is a poor fit.
vi: Phân tích khi nào Event-Driven Architecture là lựa chọn phù hợp và khi nào là lựa chọn tệ.

**Answer:**
en: EDA is a strong fit when workflows are asynchronous, multiple consumers need the same business signal, and teams benefit from independent scaling and deployment. It is a poor fit when the domain needs immediate consistency, the flow is simple request-response, or the team cannot support broker operations, tracing, and schema governance.
vi: EDA phù hợp khi luồng xử lý mang tính bất đồng bộ, nhiều consumer cần cùng một tín hiệu nghiệp vụ và team hưởng lợi từ việc scale hoặc deploy độc lập. Nó là lựa chọn tệ khi domain cần nhất quán tức thì, luồng xử lý chỉ là request-response đơn giản hoặc team chưa đủ năng lực vận hành broker, tracing và governance cho schema.

#### Q_LEVEL4_617: Analyze ordering versus throughput

**Question:**
en: Analyze the trade-off between strict message ordering and high throughput.
vi: Phân tích trade-off giữa strict message ordering và throughput cao.

**Answer:**
en: Strict ordering usually requires serial processing within a key, partition, or queue, which limits parallelism and slows throughput. High throughput typically comes from partitioning and concurrency, but that increases the chance of out-of-order processing and requires consumers to tolerate reordering.
vi: Strict ordering thường buộc hệ thống phải xử lý tuần tự theo một key, partition hoặc queue nên giảm khả năng chạy song song và kéo throughput xuống. Throughput cao thường đến từ partitioning và concurrency, nhưng đổi lại nguy cơ out-of-order tăng lên và consumer phải chịu được việc đảo thứ tự.

#### Q_LEVEL4_629: Analyze retries and poison messages

**Question:**
en: Analyze why retries can both improve resilience and create cascading failures.
vi: Phân tích vì sao retry vừa giúp tăng resilience vừa có thể tạo ra cascading failure.

**Answer:**
en: Retries recover from transient issues, but if the downstream system is already unhealthy they can multiply pressure, create retry storms, and delay recovery. The analysis must distinguish transient faults from poison messages and apply backoff, circuit breaking, and DLQs to avoid system-wide amplification.
vi: Retry giúp vượt qua lỗi tạm thời, nhưng nếu downstream đã không khỏe thì retry có thể nhân áp lực lên nhiều lần, tạo retry storm và làm chậm quá trình hồi phục. Phân tích đúng phải tách lỗi tạm thời khỏi poison message, đồng thời dùng backoff, circuit breaker và DLQ để tránh khuếch đại lỗi toàn hệ thống.

#### Q_LEVEL4_643: Analyze choreography vs orchestration

**Question:**
en: Compare saga choreography and orchestration from a maintainability and observability perspective.
vi: So sánh saga choreography và orchestration dưới góc nhìn maintainability và observability.

**Answer:**
en: Choreography keeps services loosely coupled and avoids a central coordinator, but the workflow can become implicit and hard to trace as more services react to each other. Orchestration makes the workflow explicit and easier to monitor, but it introduces a central dependency that can become complex or overly controlling.
vi: Choreography giữ các service tách rời hơn và không cần coordinator trung tâm, nhưng khi số service tăng thì luồng nghiệp vụ trở nên ngầm ẩn và khó trace. Orchestration làm workflow rõ ràng hơn và dễ quan sát hơn, nhưng lại đưa vào một điểm phụ thuộc trung tâm có thể trở nên phức tạp hoặc điều khiển quá mức.

#### Q_LEVEL4_658: Analyze event contract evolution

**Question:**
en: Analyze the risks of changing an event contract after many consumers depend on it.
vi: Phân tích rủi ro khi thay đổi event contract sau khi đã có nhiều consumer phụ thuộc.

**Answer:**
en: Changing an event contract can silently break consumers, corrupt downstream assumptions, and create split-brain interpretations of the same business fact. Mature systems treat event schemas as versioned public contracts, use compatibility testing, and roll out changes gradually.
vi: Thay đổi event contract có thể âm thầm làm hỏng consumer, phá vỡ giả định ở downstream và tạo ra nhiều cách hiểu khác nhau về cùng một business fact. Hệ thống trưởng thành xem event schema như public contract có version, có kiểm thử compatibility và rollout thay đổi theo từng bước.

---

### Level 5: Evaluating

#### Q_LEVEL5_702: Evaluate EDA against synchronous APIs

**Question:**
en: Evaluate Event-Driven Architecture against direct synchronous API calls for service integration.
vi: Đánh giá Event-Driven Architecture so với gọi API đồng bộ trực tiếp để tích hợp giữa các service.

**Answer:**
en: Synchronous APIs are simpler to reason about, easier to debug, and better when an immediate response is required. EDA is stronger when decoupling, independent scaling, fan-out, and resilience to temporary unavailability matter more than immediate consistency and linear control flow.
vi: Gọi API đồng bộ dễ hiểu hơn, dễ debug hơn và phù hợp khi cần phản hồi ngay lập tức. EDA mạnh hơn khi ưu tiên giảm coupling, scale độc lập, fan-out cho nhiều consumer và chịu được việc một thành phần tạm thời không sẵn sàng, dù phải đánh đổi sự nhất quán tức thì và luồng điều khiển tuyến tính.

#### Q_LEVEL5_714: Evaluate the outbox pattern cost

**Question:**
en: Evaluate whether the outbox pattern is worth the added complexity.
vi: Đánh giá liệu outbox pattern có đáng để chấp nhận độ phức tạp tăng thêm hay không.

**Answer:**
en: The outbox pattern is usually worth it when losing or duplicating integration events would create serious business damage. Its cost is extra storage, background publishing, replay logic, and operational monitoring, so for low-risk integrations a simpler approach may be acceptable.
vi: Outbox pattern thường đáng giá khi việc mất hoặc nhân đôi integration event có thể gây thiệt hại nghiệp vụ lớn. Chi phí của nó là thêm lưu trữ, publisher nền, logic replay và giám sát vận hành, nên với tích hợp ít rủi ro thì cách đơn giản hơn có thể vẫn chấp nhận được.

#### Q_LEVEL5_726: Evaluate exactly-once claims

**Question:**
en: Evaluate the claim that a messaging platform provides exactly-once processing.
vi: Đánh giá nhận định rằng một nền tảng message có thể cung cấp exactly-once processing.

**Answer:**
en: Exactly-once claims are usually narrower than they sound. A platform may offer exactly-once delivery or transactional semantics within certain boundaries, but end-to-end business processing still depends on consumer side effects, databases, and external calls, so idempotency remains essential.
vi: Nhận định exactly-once thường hẹp hơn cách nó được quảng bá. Nền tảng có thể hỗ trợ exactly-once delivery hoặc transactional semantics trong một số ranh giới nhất định, nhưng xử lý nghiệp vụ end-to-end vẫn phụ thuộc vào side effect của consumer, database và lời gọi ra ngoài, nên idempotency vẫn là yêu cầu cốt lõi.

#### Q_LEVEL5_738: Evaluate central broker dependency

**Question:**
en: Evaluate the architectural risk of making the message broker a central dependency for many domains.
vi: Đánh giá rủi ro kiến trúc khi biến message broker thành phụ thuộc trung tâm của nhiều domain.

**Answer:**
en: A central broker can simplify integration and standardize patterns, but it also becomes a shared critical dependency whose outages, misconfiguration, or governance problems can affect many teams at once. The decision is acceptable only when the organization invests in broker reliability, capacity planning, and clear ownership.
vi: Broker trung tâm có thể giúp việc tích hợp dễ hơn và chuẩn hóa pattern, nhưng nó cũng trở thành phụ thuộc sống còn dùng chung, nơi outage, cấu hình sai hoặc governance kém có thể ảnh hưởng đến rất nhiều team cùng lúc. Quyết định này chỉ hợp lý khi tổ chức đầu tư nghiêm túc vào độ tin cậy của broker, capacity planning và ownership rõ ràng.

#### Q_LEVEL5_749: Evaluate observability investments

**Question:**
en: Judge whether a team can safely scale EDA without serious observability investment.
vi: Đánh giá liệu một team có thể scale EDA an toàn mà không đầu tư nghiêm túc cho observability hay không.

**Answer:**
en: No, not for long. As the number of services, topics, and asynchronous hops grows, weak observability turns incident response into guesswork. A team that cannot trace message flow, inspect lag, correlate failures, and replay safely will eventually lose control of production behavior.
vi: Không, ít nhất là không bền vững. Khi số lượng service, topic và bước bất đồng bộ tăng lên, observability yếu sẽ biến việc xử lý sự cố thành đoán mò. Nếu team không trace được luồng message, không nhìn được lag, không correlate được lỗi và không replay an toàn thì sớm muộn cũng mất kiểm soát production.

