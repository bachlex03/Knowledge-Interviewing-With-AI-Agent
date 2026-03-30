# Integers in Rust - Bloom's Taxonomy

## Level 1: Remembering

**Question 1:**
en: What is the default integer type inferred by the Rust compiler if you don't explicitly specify one?
vi: Kiểu số nguyên mặc định nào được trình biên dịch Rust suy diễn nếu bạn không chỉ định kiểu cụ thể?

**Answer 1:**
en: The default integer type is `i32`.
vi: Kiểu số nguyên mặc định là `i32`.

**Question 2:**
en: State the difference between an `i8` and a `u8`.
vi: Nêu sự khác biệt giữa `i8` và `u8`.

**Answer 2:**
en: `i8` is an 8-bit signed integer (can represent both positive and negative numbers). `u8` is an 8-bit unsigned integer (can only represent non-negative numbers like 0 and positive values).
vi: `i8` là số nguyên có dấu (signed integer) 8-bit (có thể biểu diễn cả số âm và số dương). `u8` là số nguyên không dấu (unsigned integer) 8-bit (chỉ có thể biểu diễn số không âm như 0 và các giá trị dương).

---

## Level 2: Understanding

**Question 1:**
en: Explain how the architecture-dependent integer types (`isize` and `usize`) determine their memory size.
vi: Giải thích cách các kiểu số nguyên phụ thuộc vào kiến trúc (architecture-dependent) như `isize` và `usize` xác định kích thước bộ nhớ của chúng.

**Answer 1:**
en: `isize` and `usize` depend on the architecture of the computer your program is running on. If you are on a 64-bit architecture, they will be 64 bits in size. If you are on a 32-bit architecture, they will be 32 bits. They are primarily used for indexing collections (like arrays and vectors) where the maximum index is bounded by the system's memory address space.
vi: `isize` và `usize` phụ thuộc vào kiến trúc của máy tính mà chương trình của bạn đang khởi chạy. Nếu bạn ở trên nền tảng kiến trúc 64-bit, kích thước của chúng sẽ là 64 bits. Nếu trên 32-bit, chúng sẽ là 32 bits. Chúng chủ yếu được dùng để đánh chỉ mục (indexing) các collection (như mảng và vector) nơi mà số chỉ báo tối đa bị giới hạn bởi không gian địa chỉ bộ nhớ hệ thống.

**Question 2:**
en: What happens in Rust during an "integer overflow" when compiling in debug mode versus release mode?
vi: Điều gì xảy ra trong Rust khi hệ thống bị "tràn số nguyên" (integer overflow) lúc biên dịch ở chế độ debug so với chế độ release?

**Answer 2:**
en: 
- In **debug mode**, Rust includes runtime checks for integer overflow that cause the program to panic (crash) if an overflow occurs.
- In **release mode**, Rust does not include these checks. Instead, it performs two's complement wrapping. For example, a `u8` trying to hold 256 wraps around to 0, 257 wraps around to 1, etc. The program will not panic, but it will result in logically incorrect values.
vi: 
- Ở **chế độ debug**, Rust nhúng kèm các đợt kiểm tra tại runtime về lỗi tràn số nguyên, khiến chương trình panic (đứt bóng) nếu bị tràn số.
- Ở **chế độ release**, Rust tháo bỏ các chốt kiểm tra này. Thay vào đó, nó thực hiện bước vòng lặp bù hai (two's complement wrapping). Ví dụ: một `u8` khi cố gồng gánh chứa số 256 thì sẽ tự cuộn bóp vòng về thành 0, số 257 thì cuộn tụt về lại 1, v.v. Chương trình sẽ không bị dọa nới panic crash, nhưng kết quả nôn ra logic giá trị bậy bạ sai bét nhè.

---

## Level 3: Applying

**Question 1:**
en: Write a code snippet showing how to define variables holding the maximum possible value for a `u32` and an `i16` using standard library constants.
vi: Viết mã nguồn hiển thị cách định nghĩa các biến chứa giá trị điểm trần to lớn nhất có thể của một kiểu `u32` và một kiểu `i16` sử dụng các hằng số từ thư viện tiêu chuẩn (standard library constants).

**Answer 1:**
en: We can use the `<type>::MAX` constants provided by the standard library.
vi: Chúng ta có thể dùng hằng số `<type>::MAX` do thư viện tiêu chuẩn cung cấp.

```rust
fn main() {
    let max_u32 = u32::MAX;
    let max_i16 = i16::MAX;
    
    println!("Max u32: {}", max_u32);
    println!("Max i16: {}", max_i16);
}
```

**Question 2:**
en: Demonstrate how to explicitly handle an integer overflow for a `u8` variable holding `255` when adding `1` to it using the `wrapping_add` method.
vi: Trình diễn tài làm sao để ôm cua xử lý rõ ràng pha tràn mảng số nguyên dành cho biến `u8` đang ôm bụng giữ số `255` khi kẹp thêm `1` vào nó nương theo phép mượn hàm `wrapping_add`.

**Answer 2:**
en: Instead of relying on the default operator `+`, we use `.wrapping_add()` which explicitly dictates that we want the overflow to wrap around securely.
vi: Từ khước bỏ phép gán mặc điểm `+`, ta tóm gáy thả cửa bắt bằng tay hàm `.wrapping_add()` quyết lệnh ban thẳng rõ ràng cho tụi nó cuộn xoay vòng trượt xuống vạch đầu.

```rust
fn main() {
    let x: u8 = 255;
    // let y = x + 1; // This would panic in debug mode
    let y = x.wrapping_add(1); // This safely wraps to 0
    println!("Wrapped 255 + 1 = {}", y);
}
```

---

## Level 4: Analyzing

**Question 1:**
en: Analyze the compilation failure in the following code. Connect it to Rust's strong static typing principles.
vi: Phân tích sự cố thất bại thê thảm tại kỳ compilation qua đoạn code sứt sẹo dưới. Móc nối vấn đề xúi giục tới nguyên lý tạc tượng kiểu gán static mạnh như bạo chúa của Rust.
```rust
fn main() {
    let a: i32 = 10;
    let b: i64 = 20;
    let c = a + b;
}
```

**Answer 1:**
en: The code fails to compile because Rust does not forcefully or implicitly cast/coerce primitive types, even if one type entirely encompasses the other (like `i64` encompassing `i32`). The `+` operator requires the left and right operands to be explicitly of the exact same type. By refusing implicit casting, Rust completely eliminates silent truncation or unintended data widening bugs that plague languages like C or C++. You must explicitly use the `as` keyword (e.g., `a as i64 + b`) to signal your specific intent.
vi: Dòng lệnh sụp đổ biên dịch thất thủ do Rust chặn nọng họng không tự tiện khống ép hay gán ngầm lùi đè các type cơ sở (primitive cast), cho dù kể có chuyện một type bao trọn mẹ nó luôn sức chứa type con (giống như `i64` dư sức nhồi trọn ruột `i32`). Thần chú `+` ép giá gắt gao rằng vế trước với vế sau bắt buộc cùng mang mã đúng chuẩn y tạc 1 định dạng type. Xóa sổ trò dán nhãn implicit casting, Rust chặt nát hoàn kiếp bầy bọ bug mất dạy gặm dữ liệu cắt cụt (truncation) âm thầm hay ép cơi nới type vô học vẫn hay ngấu nghiến tụi ngôn ngữ C hoặc C++. Lập trình viên đành phải công khai xách gươm ép kiểu nương qua chữ `as` (dạng `a as i64 + b`) để tung tín hiệu ngầm đồ xẻ type rõ ràng định ý.

**Question 2:**
en: Contrast the semantic usage of `i32` versus `usize` in software architecture. Given they might both represent 32 bits on a 32-bit machine, when is it architecturally incorrect to use `i32` for array element access?
vi: So sánh định mức ngữ nghĩa đối chọi xài `i32` mắm muối với `usize` theo triết lý hạ tầng phần mềm software architecture. Nếu lỡ xui rủi ở vào khung sườn máy dòng 32-bit khiến cả cặp hai con type đều tọng ra định danh 32 bits, mốc khoảnh khắc nào xướng sai sách vở kiến trúc nếu xài bừa `i32` vô đập trỏ hàm array element access?

**Answer 2:**
en: Architecturally, the type represents the conceptual domain of the data. Use `i32` for general mathematical numeric values and computational counters that might potentially plunge below zero into negative territory. Using `i32` for array indexing is architecturally incorrect (and technically forbidden by Rust without casting) because indices represent physical hardware memory offsets or sequential offsets in continuous storage. A memory address offset cannot conceptually be negative. `usize` guarantees compatibility with the host CPU's pointer width boundary naturally, making it the mathematically sound, strictly-positive definition for modeling container capacities and element addressing.
vi: Rẽ nhánh Architecture, cục diện Type thay mặt cái hồn concept khuếch đại của data. Trọng dụng `i32` cống vào đài các thông số tính toán hệ toán học phổ thông, bộ đếm số liệu lú lấp trượt mốc hụt chân rớt phanh xuống hàng chục âm giới hạn nơ. Cầm dao trổ `i32` tỉa xén đâm chọc array indexing (xài đánh dấu) bị xướng phán sai hệ rễ kiến trúc (và bị búng hạch kẹt cổ triệt hạ bắt cast type mới cho xài ở Rust) hỡi vì các cọc dấu indices trình diễn khung cự ly dời offset địa chỉ bộ nhớ mỏng vật lý hoặc offset tuần tự khảm trên mặt block kho chứa kéo dài liên hoàn. Một gá điểm Memory Offset không thể có khái niệm xuyến thủng về hầm số âm trượt lùi được. Cấu trúc `usize` bảo bọc ôm ấp khớp tương thích chiều rộng Pointer host CPU tự thân, khiến nó trồi mặt làm một bộ thiết định tạc mẫu mốc chuẩn ranh giới dương cứng nhằm xoay định vị công suất chứa kho bãi và xướng danh cấu hình truy xuất vạch điểm cho chuẩn khớp tự nhiên.

---

## Level 5: Evaluating

**Question 1:**
en: Evaluate Rust's decision to trigger an aggressive panic on debug integer overflows rather than gracefully defaulting to two's complement wrapping globally like C/C++. Does this strictly "fail fast" paradigm hinder or accelerate system-level development workflows?
vi: Cân nắn lường định lại quyền sinh sát quyết định khép đòn hung hăng kéo sập hệ thống (panic crash) chát chúa tại cửa ải biên dịch tậu lỗi Overflow lúc xài debug số integer thay cho việc mở rào êm ru băm xéo wrapping hai luồng hệt bọn truyền thống C/C++. Thể loại thiết luật "thua cháy nạp nhanh - fail fast" này chặn cổ kéo lùi hay đẩy phi tốc hành vòng luân canh guồng máy phát triển (development workflow) hệ thống lõi system-level?

**Answer 1:**
en: Rust’s "fail fast" paradigm by panicking on debug integer arithmetic drastically accelerates system-level development. In C/C++, integer overflows silently wrap. This breeds insidious, silent logical corruption that can persist for months in massive codebases and ultimately manifest as subtle critical security vulnerabilities (e.g., buffer overflows sparked by bad wrapped indices or broken financial math loops). By aggressively cratering the application at the precise line where the overflow occurred during the debug/testing phase, Rust forces engineers to definitively acknowledge and architecturally fix edge-case bounds handling upfront. Although it disrupts flow instantly, it terminates the long-tail debugging nightmare later, massively reducing total-cost-of-ownership and increasing platform stability.
vi: Bộ kỉ cương đè lướt "sụp cấp tốc" (fail fast) kích động kíp nổ panic lên nòng hễ vướng integer tràn mảng rút màng kịch bản debug tăng lực tốc chạy cực sốc vòng đẩy hệ system-level. Đứng cõi C/C++, số lồng ngập integer lún lấp im ỉm xoay vòng bù trừ ngầm. Trò dơ đó nuôi tụ rệp bệnh hiểm nghèo tàn bạo, tạo chứng độc thối dữ liệu mục rữa ngầm ủ mầm lù lù nằm yên trong bể codebase phình bự hàng tháng trời và cựa quậy phơi bầy ở diện là một siêu lổ hổng an ninh tàn sát sinh mệnh nền tảng (như ngợp buffer phình do thông số bù xoay dội sai nhịp hay nhồi chết quỵ máy vòng loop cỗ máy tài chánh đứt hơi). Ép bằng chiêu đào hố sập tàn khốc nguyên rập ứng dụng ngay chính tại hố vạch gạch nứt mã hễ nhú overflow tại quãng test/debug mồi, Rust vặn ngàm cưỡng chế hội thợ kĩ thuật bắt sống trói ghi nhận đối chứng và nhào vô phá giải dán vá xử chèn logic đường ranh rìa góc sườn mẻ (edge-case) lập tức khỏi lấp liếm. Dẫu cắt lụp cụp khoảnh khắc rớt luồng dòng code ngẫu hứng, trò đè nén đó tiễn vong hàng chuỗi ác mộng debug đuôi dài lằn ngoằng ở hậu kiếp, đập lủng mớ chi phí nuôi bảo tồn tổng thể system và bồi đắp vĩnh cửu độ vững chãi thiết kế gốc rễ.
