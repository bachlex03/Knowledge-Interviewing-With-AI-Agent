# String Types in Rust - Bloom's Taxonomy

## Level 1: Remembering

**Question 1:**
en: How do you create a new, empty `String`?
vi: Bạn tạo một `String` mới, rỗng như thế nào?

**Answer 1:**
en: You create a new, empty string by calling `String::new()`.
vi: Bạn tạo một chuỗi mới, rỗng bằng cách gọi `String::new()`.

**Question 2:**
en: State a common method to append a string slice (`&str`) to a `String`.
vi: Nêu một phương thức dùng chung để thêm (append) một string slice (`&str`) vào cuối của một `String`.

**Answer 2:**
en: The `push_str` method is used to append a string slice to a `String`.
vi: Phương thức `push_str` được sử dụng để nối thêm một string slice vào cuối của một `String`.

---

## Level 2: Understanding

**Question 1:**
en: Explain the difference between `String` and `&str` in terms of memory allocation.
vi: Hãy giải thích sự khác biệt giữa `String` và `&str` về mặt phân bổ/cấp phát bộ nhớ.

**Answer 1:**
en: A `String` is a growable, mutable, owned, UTF-8 encoded string type. Its text data is allocated dynamically on the heap. A `&str` (string slice) is an immutable reference pointing to a sequence of UTF-8 text somewhere in memory (which could be the heap, the stack, or the read-only data segment of the compiled binary).
vi: `String` là kiểu chuỗi sở hữu (owned), có thể thay đổi và mở rộng được, mã hóa chuẩn UTF-8. Chuỗi văn bản của nó được phân bổ memory động trên phần Heap. Type `&str` (lát cắt chuỗi) là loại tham chiếu không thể thay đổi mượn nhìn thẳng vào mảng phần tử text UTF-8 nằm đâu đó ngoài kia (chẳng hạn trên đoạn Heap, Stack, phân vùng binary cố định chỉ đọc Read-Only).

**Question 2:**
en: Why can't you index a `String` directly with an integer, like `s[0]`?
vi: Tại sao bạn không thể tác động Index duyệt vào một `String` trực tiếp bằng kiểu số nguyên đơn thuần ví dụ `s[0]`?

**Answer 2:**
en: Rust strings are UTF-8 encoded, meaning a single "character" might take anywhere from 1 to 4 bytes. Indexing with an integer (`s[0]`) would return the byte at that index, not necessarily the actual human-readable character. To prevent bugs and undefined behavior involving malformed characters, Rust prohibits direct integer indexing on strings, forcing developers to be explicit about whether they want iterating bytes (`.bytes()`) or characters (`.chars()`).
vi: Định luật Rust quy vào khung chuẩn UTF-8, mang nghĩa một "kí tự" có dao động tự định cỡ size ở khoảng 1 tới 4 byte. Đào Index số nguyên thô (kiểu `s[0]`) sẽ nôn ra byte lẻ ở con số đó chứ chưa chắc nó là kí tự thực ghép vô nghĩa đọc viết của loài người. Mượn để cản dòng dã tâm lỗi Bug và những chiêu thức undefined mông lung xoáy tại những text dị hợm, Rust ngắt đứt đường cho Index số vô luồng chuỗi, áp tròng bắt buộc Kỹ sư phải khai tính định dạng thẳng băng là đang chu du trên tầng bytes lẻ (`.bytes()`) hay là kí tự nguyên vẹn `.chars()`.

---

## Level 3: Applying

**Question 1:**
en: Write code that initializes a `String` with "foo", then appends "bar" using `push_str`, and lastly appends the single character "!" using `push`.
vi: Ghi đoạn code khởi hành tạo `String` mang "foo", rồi khép nối đuôi "bar" dùng `push_str`, và chấm chót đuôi đẩy chữ đơn "!" bằng `push`.

**Answer 1:**
en: Make sure the initial string is mutable.
vi: Đảm bảo chuỗi cấu thành lúc khởi tạo là mutable (có thể thay đổi).

```rust
fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("{}", s); // Prints "foobar!"
}
```

**Question 2:**
en: Demonstrate how to concatenate two `String` variables `s1` ("Hello, ") and `s2` ("world!") using the `+` operator. Explain what happens to the ownership of `s1`.
vi: Phô diễn kĩ thuật cách móc xích khâu hai `String` biến là `s1` ("Hello, ") và `s2` ("world!") nương tựa thủ thuật toán tử `+`. Giải trình đường đi cho bộ quy chế sở hữu owner của `s1`.

**Answer 2:**
en: When using the `+` operator, the first operand must be an owned `String` and the second must be a string slice `&str`. The `+` operator takes ownership of `s1`, appends a copy of the contents of `s2`, and then returns the ownership. After `s3` is created, `s1` is no longer valid.
vi: Phép gán cận `+` có một luật khắt khe, tham biến mở mào phải là sở hữu `String` và đứa theo hầu kề bên là gán `&str`. Toán toán tử `+` sẽ tịch thu thẳng cẳng sở hữu owner của `s1`, lùa chích thêm nội dung tóm chép của `s2`, rồi nôn trút ra trả về dạng biến mới. Hậu vận `s3` vừa mọc thì `s1` lăn đùng ra hóa thành rác cấm xài r lụi.

```rust
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used
    
    // println!("{}", s1); // This would result in an error
    println!("{}", s3); // Prints "Hello, world!"
}
```

---

## Level 4: Analyzing

**Question 1:**
en: Analyze the signature of the `add` method underlying the `+` operator for strings: `fn add(self, s: &str) -> String`. Why does it take `self` by value rather than by reference?
vi: Tra hỏi định kiến trúc phân rã (signature) cấu thành `add` cắm vô xương toán tử `+` áp cho loại string chuỗi: `fn add(self, s: &str) -> String`. Vấn đạo tại sao nó kẹt vô rút thẳng máu sinh thể `self` loại trực diện (by value) thay vì ngắm chơi như by reference?

**Answer 1:**
en: The `add` function takes `self` by value to claim ownership of the first string. This is highly efficient because it avoids allocating a brand-new chunk of memory on the heap. Instead, it takes the existing memory allocation of the first string (`self`), grows it if necessary, appends the contents of the second string slice onto the end of it, and returns ownership of that same chunk of memory back to the caller. If it took exactly `&self`, it would be forced to allocate completely new memory to hold the combined strings, making the `+` operator surprisingly expensive in loops.
vi: Biến cục hàm `add` đoạt quyền tóm gọn trực tiếp sinh tính của `self` để chiếm hữu triệt luôn bộ chuỗi một. Điều này thần tốc vô định vì né hẳn vụ phải cấp giấy mở sổ đất vùng mới trên Heap. Thay đấy, nó dùng lại cấu phần memory y chang của chuỗi một (`self`), trương phình to kéo dãn nếu cần thiết, vắt trọn nội dung cục slice chuỗi sau trát ngập chêm đít, trả mộc về ownership chính mảnh đất xưa cho thằng chúa ngự. Giả dụ nó mà tóm chốt điếm kiểu `&self`, buộc lòng xé rào khui bung mảng memory tinh sươm mới tinh để chứa đựng đống chuỗi chắp nối đó, thì phép `+` trong vòng xoay loop sẽ tốn phí đau đớn k tả kinh hãi.

**Question 2:**
en: Contrast formatting strings using the `format!` macro versus using the `+` operator. In what scenarios is one architecturally superior to the other?
vi: Luận báng cái thế thái định dạng chuỗi nương sào macro `format!` vs búa tạ toán tử `+`. Lật ván cờ, vị thế cục diện nào ngả nghiêng cho bên này mà loại bên kia k đặng xét về lõi architecture?

**Answer 2:**
en: 
- **The `+` Operator**: Forces strict ownership moves (`s1 + &s2`), makes sequential concatenation of many strings (`s1 + &s2 + &s3 + ...`) visually clunky, and alters the lifetimes of the variables involved.
- **The `format!` Macro**: Works by reading references implicitly, leaving the ownership of all original variables completely intact. It behaves similarly to `println!`.
**Superiority:** `format!` is structurally paramount when combining more than two variables or when you strictly need to reuse the input variables later in the block. `+` is only slightly superior when you want maximum performance by aggressively consuming the first string without allocating a fresh heap layout structure.
vi: 
- **Bóng tạ toán tử `+`**: Bắt người dùng trả giá nhường cống Move triệt để owner (`s1 + &s2`), phơi bày nối tiếp tản hệ đống string quá tải (`s1 + &s2 + &s3`) lòi mẻ trơ trẻn rệu rã thị giác, bẻ cong sinh thái vòng đời mấy biến liên can.
- **Dinh cơ Macro `format!`**: Tung chiêu duyệt tham chiếu (refs) kín đáo, giữ gin cốt vẹn nguyên nguyên lành hệ sinh đồ biến original khỏi vết xước owner dời dịch, xử thế kiểu giống y `println!`.
**Cái ghế bá vương:** `format!` ngồi chiếu ngai tối đa khi trộn lai ghép vượt ngưỡng 2 con object biến trở lên hay trúng khoảnh lúc ngặt bắt buộc giữ sống dàn input để luân hồi xài tiếp. Phép `+` chỉ vớt chóp một chút ít độ trội nếu nương nhờ vô năng suất thần tốc bằng cách hút khô tận diệt tài nguyên chuỗi mở đường mà không đào thêm đất rác heap sinh non.

---

## Level 5: Evaluating

**Question 1:**
en: Evaluate Rust's decision to expose developers to both bytes and scalar values (characters) explicitly when iterating over a string. Is this an over-complication compared to languages like Python, where a string is just an array of characters?
vi: Kê bình luận quyền ra luật của thủ ngự Rust tẩn phơi thẳng cẳng vào mặt Developers hai khối là byte với các biến lượng vô hướng Scalar (kí tự char) tường tận khi cho chạy tuần hoàn trên dây chuỗi (String loop). Thể trạng này có phải đánh vỡ bức tường đơn giản hóa quá lố hớp (over-complicate) lật kèo luôn ba ngôn ngữ nhè nhàng ngang hàng Python vốn dán cái mác String chỉ mảng chuỗi đơn tuần hoàn không?

**Answer 1:**
en: While distinguishing between `.bytes()` and `.chars()` increases cognitive load initially, it is a crucial design decision for a modern systems language. Python abstracts this away because it carries a heavy runtime VM that handles encoding translations implicitly, often penalizing performance and memory uniformly. Rust, without garbage collection or VM overhead, mandates developers to grapple with the reality of UTF-8 encoding (that logical characters map to variable-length bytes). Exposing this prevents hidden performance penalties traversing texts, safeguards against accidental out-of-bounds indexing in multi-byte symbols, and enforces globally correct localization processing from day one—something "simpler" languages often struggle to retrofit securely at scale.
vi: Cú đánh tách bóc hai phân khu `.bytes()` đi theo ngả đường `.chars()` kéo tụt não bộ não hao mòn công nhọc mới vô nhậm chức, nhưng đó lại đại cục tuyệt kĩ sinh tồn về quy chế design tối cao cho tầng ngôn ngữ phần mềm system hệ sát nhịp. Nhánh Python gạt cái khói phù du xám mặt đó do vì chở đèo ngênh ngang cái máy ảo runtime VM nhai lốn bộ encoding ngầm kín như bưng sau lưng, giáng phạt đòn vô hiệu suất và ngốn memory bằng. Rust, ở cảnh ngộ trống vắng gom rác tự do hay khuyết thiếu VM chống chống lưng, cưỡng chế developers ôm dằn xoắn vào đúng trọng tâm cội trần thực tế của khối UTF-8 (rằng các mảng mặt chữ logic tương ứng ánh xạ cho vô ván dãy bytes). Nâng tấm khiên tường minh này cắt triệt đường phạt penalty ngầm phá sập performance nếu truy dò trên list chữ, rào khiên che giáp chống tai điếc đâm mù mắt văng rớt chỉ mục đối nghịch vào loại đa-byte xé băm, và tống đè bộ khung phân quy mô format định danh chuẩn chỉnh toàn cầu từ rạng đông bình binh—thứ cõi huyền "nhẹ gọn ngon ăn" như ba cái ngôn ngữ kia vật vã xây ngược che rách cồng kềnh.
