# Enums in Rust - Bloom's Taxonomy

## Level 1: Remembering

**Question 1:**
en: What keyword is used to define an enumeration in Rust?
vi: Từ khóa nào được sử dụng để định nghĩa một liệt kê (enumeration) trong Rust?

**Answer 1:**
en: The `enum` keyword is used.
vi: Từ khóa `enum` được sử dụng.

**Question 2:**
en: Name the two variants of the standard `Option` enum.
vi: Kể tên hai biến thể (variants) của enum `Option` tiêu chuẩn.

**Answer 2:**
en: `Some` and `None`.
vi: `Some` và `None`.

---

## Level 2: Understanding

**Question 1:**
en: Describe how Rust enums differ from enums in C or Java regarding data storage.
vi: Mô tả sự khác biệt giữa enum trong Rust và enum trong C hay Java về mặt lưu trữ dữ liệu.

**Answer 1:**
en: In traditional languages like C, enums are typically just lists of named integer constants under the hood. In Rust, enums are algebraic data types. This means each variant of an enum can mathematically hold different types and amounts of associated data. For instance, one variant can hold no data, another can hold a single `String`, and a third can hold an anonymous struct with multiple fields.
vi: Ở hệ ngôn ngữ lâu đời rập như C, enums thường chỉ là danh sách các hằng số nguyên được gán tên ẩn dưới màng vỏ. Trong Rust, enums mang danh là kiểu dữ liệu đại số (algebraic data types). Khái niệm này ban quyền năng cho mỗi biến thể variant của enum tùy tiện ôm nạp các khối data dạng type khác nhau và số lượng dị biệt. Tỉ dụ, một variant chả cõng ôm data nào ráo, variant kế lại ngậm 1 cục `String`, còn thằng chót thì ôm thầu một trùm struct vô danh ngợp list đa mảng fields.

**Question 2:**
en: Explain the significance of the `Option<T>` enum substituting for `null` concepts.
vi: Miêu tả sự vĩ đại tầm cỡ lúc gá bộ `Option<T>` enum vào nện đạp bẹp chức vị định danh `null`.

**Answer 2:**
en: Rust eliminates the billion-dollar mistake of null pointers by simply not having them. Instead, it uses `Option<T>`, which forces the compiler to ensure that the developer has explicitly handled the case where a value might not be present (`None`) before allowing them to use the encapsulated value (`Some(v)`).
vi: Rust triệt sản cục lỗi ngập bạc tị tỉ đô liên đới con trỏ null (null pointers) gọn bưng bằng luật tẩy chay gạch mặt bỏ luôn khỏi danh mục. Đè thay thế, Rust gọi ngài `Option<T>`, trét giấy hầu lệnh bắt buộc hội thẩm Compiler đứng canh dí sát cổ đám thợ code phải khai giải xẻ lớp tường tận cái hố kịch bản đen đủi giá trị rớt trống trơn (`None`) trước lúc cấp lệnh ban bài thả tự do moi nhét dữ liệu nhân ngầm (`Some(v)`).

---

## Level 3: Applying

**Question 1:**
en: Define an enum `IpAddr` with two variants: `V4` (which holds four `u8` values) and `V6` (which holds a single `String`). Formally instantiate one of each type.
vi: Tạc khai cục enum `IpAddr` chẻ dời gân làm đôi hai khúc variants: `V4` (ép nạp cõng trọn 4 mầm `u8`) và `V6` (hốt ngậm duy mộc 1 nhãn `String`). Triệu hồi đục đẽo cho ra đời 1 bản thân thể của mọi type.

**Answer 1:**
en: 
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```
vi: 
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```

**Question 2:**
en: Create an `impl` block for the `Message` enum provided below that defines a method `call` which borrows `self` and prints "Processing message".
vi: Rập khối khuôn `impl` bọc trọn con enum `Message` đền cho dưới đây nhằm vạch ra một hàm method danh nhãn `call`, đớp tham chiếu bấu vô `self` và gào ra "Processing message".

**Answer 2:**
en: 
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Processing message");
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```
vi: Đoạn mã chém rập cấu trúc `impl` đè trên thân `Message` enum, thả ra method gọi method gọi `call` mượn gốc `&self`.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Processing message");
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```

---

## Level 4: Analyzing

**Question 1:**
en: Compare structuring data using an Enum versus multiple diverse Structs bundled within a common defining Trait. How does Enum memory layout differ structurally from dynamically dispatched Trait Objects (`Box<dyn Trait>`)?
vi: Đập bàn cần so găng cấu tạo dàn nạp data cậy ngả luồng rẽ Enum đối chọi với đống rác phân tầng đa Struct thoi thóp nhúm gọn chung vòm đúc cái Trait. Thể loại memory bề mặt hạ tầng Enum chia vách cọc cạch cỡ nào khi đem chọi với chùm Đồ gá Trait phé động Runtime tĩnh rớt não hạch (`Box<dyn Trait>`)?

**Answer 1:**
en: 
- **Enums**: An enum encapsulates a closed, finite set of structural variants determined totally at compile time. Memory-wise, the compiler assigns an enum a fixed size equal to its largest variant plus a small discriminator tag byte specifying exactly which variant is active. They reside elegantly on the stack and allow rapid inline dispatch logic (`match` statements).
- **Trait Objects (`Box<dyn Trait>`)**: A trait defines an open set of behaviors, where anyone can implement the trait unrestrictedly later. Using `Box<dyn Trait>`, the structural size is totally unknown at compile time, forcing heap allocation (`Box`). Method resolution occurs dynamically at runtime via a vtable lookup (dynamic dispatch), inducing a slight performance hit.
Architecturally, you use Enums when you exhaustively know *all* possible states (like an AST parser or HTTP codes). You use Traits when you want an extensible, infinitely pluggable architecture (like game plugin mods).
vi: 
- **Phe cánh Enums**: Cáo phó giam giữ cái rạch nếp kín cửa, ấn định trọn trần quân số giới hạn cấu trục variants nhốt chặt chêm chết cứng lúc Compile phán định. Dòm ranh bộ nhớ Memory, ngài Compiler quăng phất xuống mảnh đất vạch diện tích cố đinh ngàm to cứng ngang rập đúng với size con variant chình ình béo mập khốn khiếp nhất dàn đính kẹp mẩu đuôi Tag đánh số phân loại tít mít khai định hiện hành rạc con type nào đập mặt. Cốt nằm khểnh vểnh trên ngai Stack thanh tú cho nổ tung phép nội xuy dispatch tốc váy xé gió (đòn `match`).
- **Gạch đá Trait Objects (`Box<dyn Trait>`)**: Sổ cửa rộng chà bá ngã đường luồng ngầm cho phép thiên hạ thả thòng ngòi vẫy bút nhồi implement trait từ rác mả nào tới cũng hoan nghênh. Khi ốp màng vỏ `Box<dyn Trait>`, thể xác số đo vô ảnh vô hồn, đập móp sọ Compiler khiến bão táp quăng hết vô tọng Heap lưu đày (`Box`). Khắc xuất tra mốc kiếm nhãn nhảy điểm Method tung chảo gá trên màng chạy runtime ngầm nhắm trọn móng vtable lật bài (dynamic dispatch), hao vấy hụt gót nhẹ đòn kình lực performance.
Đỉnh cao chọn bài, bợ Enums khi tay kĩ sư vạch nát cạn cõi ngóc ngách đếm xác *Mọi Toàn bộ* chốt state biến đổi (chắn hệ cưa nhánh AST parser hay định danh mã lỗi HTTP). Tung vác vũ khí Traits lúc lâm cảnh dọn đường gá nếp mở màng (extensible object), xỏ họng tháo lắp cọc nhồi cắm vô cực (như nện dàn mod cho Plugin dạo Game).

**Question 2:**
en: Analyze the compilation failure in the following Option unwrapping scenario. Why didn't Rust allow straightforward arithmetic on `Option<i32>`?
vi: Vạch não đập trần thảm sát thui mòn biên dịch nằm cục bóc vỏ Option Unwrap khía nanh xước móc này. Lối đạo nào mài nén Rust phủ ngạch cấm chặn con đẻ xử gọn nhẩm phép toán số đập thẳng tưng vô ruột `Option<i32>`?

**Answer 2:**
en: 
```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y; // ERROR
```
The design rigorously blocks it because `i8` and `Option<i8>` physically constitute distinct abstract types. `Option<i8>` mathematically represents the "possibility of an `i8` or entirely nothing". The addition operator natively has zero concept of mathematics applied to "nothingness", so Rust's type system refuses to implicitly convert or extract the interior wrapper value blindly. To fix it, you must explicitly unpack the `y` object handling the `None` scenario safely using abstractions like `unwrap_or()` or a definitive `match`.
vi: 
```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y; // ERROR báo gắt
```
Khuôn rập chặn kẽ khóa mỏ chán rào đanh thép tại lẽ type `i8` với `Option<i8>` tạc đắp hình hài phần nguyên thủy tách đôi 2 cõi trời Abstract Types mâu thuẫn khét lẹt. `Option<i8>` vung Toán học gào lên cái "sự nhen nhóm mong manh của `i8` hoặc rụng sạch thành tro". Băng đằng đĩ Toán Học `+` đéo nắm hồn lõi nào ngợi ca triết lý phép màu cọng rơm "hư vô null rỗng", thế chốt là Thần Type System của cõi Rust gõ cọc lờ nát cái móng tự vạch tự biên cast ngầm hay tự tiện đào ruột bợ data mù quáng. Cứu rỗi cõi này phải lết bệt lột trần xác nặn vạch cái phôi ảo `y` nhào lộn rẽ gạch biên rào mồi check êm ái cái xác rỗng `None` nhờ phím vuốt abstraction như `.unwrap_or()` hay búa tạ tĩnh mịch rạch hông `match`.

---

## Level 5: Evaluating

**Question 1:**
en: Evaluate the cost of Enum memory alignment (padding) and size inflation when mixing a colossal payload variant with tiny payload variants. Is this memory penalty justified by the extreme type safety gained?
vi: Đánh giá hạch giá máu chi phí Căn chỉnh kích gá Enum (memory alignment / padding) cùng đống ngấn mỡ size lấp phình ngập ngụa lúc khua khoắng quết mớ 1 ông bự khổng lồ chở kho tàng payload quấn ngang chòm em út rác payload li ti nhẹ dăm ba hột cắc. Ké hỏi móp điểm tụt tì này dạt ra do bị đòn penalty memory có sấn sổ ngang kèo thỏa hiệp cùng món hời đoạt được độ giáp cứng type safety tạc thượng đỉnh ko?

**Answer 1:**
en: Memory inflation in badly balanced enums undeniably wastes resources. Because Rust statically maps stack space, an enum sizing equates to its absolute uncompressed maximum variant plus padding offsets. A variant like `Err(HugeHttpPayload<String>)` mixed with `Ok(())` forces every single successful `Ok()` token throughout the application loop to brutally hog Megabytes of stack real estate unnecessarily despite carrying zero data itself.
However, this penalty is architecturally justified. It completely kills unsafe memory intersection boundaries globally and prevents disjoint union exploits rampant in C (`union`). And most fundamentally, developers have direct, easy ergonomic bypasses natively: simply box the colossal variant via a heap pointer (`Err(Box<HugeHttpPayload>)`). This dynamically shrinks the overarching enum layout footprint down to the size of a single pointer (8 bytes), immediately neutralizing stack bloat while retaining absolute rigorous enum pattern-matching structural safety properties.
vi: Phình bụng nổ size mỡ Enum lệch tâm gá nghiêng xiên xẹo dĩ nhiên đớp trọn oan ức cướp kho memory nhét chật tốn điện. Nối dây Rust phác xăm ngàm bộ sườn Stack cứng chẻ, cỡ bó mộc Enuming dàn cược đong ni ngang tầm đống đo bự khổng tàn cực đại nhất của cái variant lấp bụng padding căn lề giãn khớp. Tỉ nhúm hạch variant đụng cái hàm `Err(HugeHttpPayload<String>)` mà rớt xen ngang một nhãi `Ok(())` tí hon rỗng xốp, cớ thì toàn bộ những con chip báo mồi `Ok()` rầm rập chạy vọt bay ngang Application Loop chềnh ềnh đi nuốt trọn lút nốc hàng tá Megabyte móng đất stack vô phương vô nghĩa trơ trẽn cho dẫu cốt hồn rỗng tít mù k mang mảy mai hạt bụi data vặt.
Nhưng dẫu vậy, trò nện penalty sấp ngửa cực hạn này minh chứng cho đạo phái System Arch hoàn mĩ nhất. Nó ném lựu đạn tử hình trọn móng những rìa biên intersection dẫm memory lở lói đâm xuyên của mấy hệ vô ngả C dạo trước (mảng `union` bọc chui C). Tuyệt đỉnh tấu chót, hội anh hào developer tự thân dắt ngực bí kiếp qua ải nhẹ tanh dẻo kẹo: chỉ nước lôi thằng Variant chúa quỷ khổng phình bọc tọng mẹ nó dẹp xuống nhốt ở cái Rương hòm sờ cọc Pointer trỏ xuống `Box` (`Err(Box<HugeHttpPayload>)`). Trò đập xẹp này ép teo đét sườn đo của cái phễu enum lụt mông chà bá biến tấu thon lỏn thu nhỏ bằng hạt bụi cái trỏ Pointer (vỏn vẹn 8 byte ngàm dính), chém cái nấc bloat bay sập sình, và cưu trọn linh hồn nguyên thủy bảo tàng vững như bàn thạch pattern-matching tuyệt sắc vô tiền khoáng hậu an ninh của đền Enum vĩ đại.
