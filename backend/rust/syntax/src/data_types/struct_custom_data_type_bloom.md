# Structs in Rust - Bloom's Taxonomy

## Level 1: Remembering

**Question 1:**
en: What keyword is used to define a struct in Rust?
vi: Từ khóa nào được sử dụng để định nghĩa một struct trong Rust?

**Answer 1:**
en: The `struct` keyword is used.
vi: Từ khóa `struct` được sử dụng.

**Question 2:**
en: Rust features three kinds of structs. Name two of them.
vi: Rust cung cấp ba loại struct. Hãy kể tên hai trong số đó.

**Answer 2:**
en: 
1. Classic C-like structs (named fields).
2. Tuple structs (unnamed fields).
3. Unit-like structs (no fields).
vi: 
1. Struct cổ điển kiểu C (các trường được đặt tên).
2. Tuple struct (các trường không có tên).
3. Unit-like struct (không có trường nào).

---

## Level 2: Understanding

**Question 1:**
en: Differentiate between a standard struct and a tuple struct. When might you choose to use a tuple struct over a standard struct?
vi: Phân biệt giữa một struct tiêu chuẩn và một tuple struct. Khi nào bạn có thể chọn sử dụng tuple struct thay vì định hướng tạo struct tiêu chuẩn?

**Answer 1:**
en: A standard struct explicitly names each of its fields (e.g., `username: String`, `age: u32`), making data perfectly structured and self-documenting. A tuple struct behaves like a named tuple; it assigns a distinct type name to a tuple but does not name the individual fields, accessed simply via numeric indices like `my_tuple.0`. You use a tuple struct when you want to create a new robust type purely to differentiate it from other tuples or data, but naming the fields would be completely redundant (like `struct Color(i32, i32, i32)` instead of `struct Color { r: i32, g: i32, b: i32 }`).
vi: Ở struct dòng kinh điển phân luồng nhãn cho rạch ròi cục field riêng biệt (như `username: String`, `age: u32`), tạo tiền đề vững kiến trúc rõ nét định đoạt cấu trúc trần lấp lửng cho code dồn documentation tự thân sinh động. Còn thằng tuple struct là kiểu y đúc tuple thường; nó quăng một cái danh xưng riêng lên 1 chuỗi tuple vô danh để định tính lại thành format riêng biệt nhưng bưng bít tuyệt khẩu luôn danh sách field mảng liệt kê nội tạng, bới móc thông tin xài số móc `my_tuple.0`. Căn nguyên lôi Tuple struct xài là do ý niệm vạch trần ra 1 Type khác biệt cho tuple cơ bản cản trở chọc lẫn so những thằng tuple type nhãn danh chìm, nhưng ép đặt nhãn list cho nó đắp chăn thâm là dư thừa rác (chẳng hạn xài `struct Color(i32, i32, i32)` gọn hơn là bắt viết `struct Color { r: i32, g: i32, b: i32 }`).

**Question 2:**
en: If you instantiate an immutable struct `let user = User { ... }`, can you modify just one specific field inside it if the other fields stay the same?
vi: Giả trượng bạn bưng chậu phôi khai mở một struct tĩnh giới cấm (immutable) kiểu `let user = User { ... }`, cho hỏi ta có tài cán gì nhúng tay xẻ sửa được duy độc có mỗi mảng field lẻ của nó trong khi mấy ông field cọc còn lại nêm y nếp cũ không?

**Answer 2:**
en: No. In Rust, mutability is determined per structure instance, not per field. To change any single field, the entire struct instance must be declared as mutable using the `let mut` keyword.
vi: Chặn họng nói không. Cõi luật Rust định phong Mutablity tính đa biến cải là ban lệnh trên toàn bộ phận cấu kiện một Struct lớn instance duy nhất, chả có trù bị chi phân lô vạch ranh trên field. Muốn sờ nắn chọt dù là mảng gạch siêu nhỏ thì cần tuốt gươm kéo lệnh mutable vào bộ vỉa via `let mut` khi thai sinh nặn hình nguyên vẹn instance nọ.

---

## Level 3: Applying

**Question 1:**
en: Define a struct `Rectangle` with width and height (both `u32`). Then, write a function `area` that takes a reference to a `Rectangle` and returns its area.
vi: Dán chữ mở rập một struct `Rectangle` có width và height (rập khuôn type `u32`). Lặn hụp bơi lội viết ra function `area` mượn cống nạp nạp mượn một tham chiếu đến một `Rectangle` và phun thả nhả diện tích (area).

**Answer 1:**
en: 
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    let my_rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area is: {}", area(&my_rect));
}
```
vi: 
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    let my_rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area is: {}", area(&my_rect));
}
```


**Question 2:**
en: Instantiate a new struct `user2` by copying data from `user1` using the Struct Update Syntax, but with a different `username` string.
vi: Phá khuôn thai sinh khai rạch một struct con mới châm ngòi `user2` bằng cách rút tỉa hốt trọn code data qua `user1` với thần chú Struct Update Syntax, ngoại lệ loại trừ đi phần chuỗi khai thiên chọc phá `username` rẽ hướng qua string format khác.

**Answer 2:**
en: The Syntax `..user1` takes all fields from `user1` that were not explicitly set in the new instance and moves/copies them into `user2`.
vi: Cú nện Syntax `..user1` đoạt rút đong trọn mảng bao field kể lể thuộc địa `user1` chưa bị gọi hồn khai điểm trên cái ruột phôi vỏ instance và bế (move/copy) tất ném ùm hết ráo vào đít `user2`.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        username: String::from("new_user_name"),
        ..user1 // Struct Update Syntax
    };
}
```

---

## Level 4: Analyzing

**Question 1:**
en: Analyze the consequence of using the Struct Update Syntax (`..user1`) if `user1` contains types that do not implement the `Copy` trait (like `String`). Can `user1` still be completely used afterward?
vi: Đập chày phân giải những hệ lụy của cú pháp chĩa Update Struct kia (`..user1`) vướng vào trường hợp `user1` có tích hệ dạng hạch mà chả implement vạch `Copy` trait vướng (kê nhẩm như hàm `String`). Phân chia lằn ranh mổ não `user1` nó còn được nựng gọi dùng lành lặn không sau đó?

**Answer 1:**
en: If you use setup Struct Update Syntax and transfer fields that are moved (like types containing `String` as opposed to basic `i32` integers that implement `Copy`), the ownership of those specific fields is moved into the new struct instance. Therefore, the original struct (`user1`) becomes partially invalid. You can no longer access the moved fields in `user1` nor borrow `user1` as a complete entity. You can only safely access the fields of `user1` that were either `Copy` (like `sign_in_count`) or explicitly overwritten in the new instantiation.
vi: Nếu sờ cập đụng đến Struct Update Syntax, rớt dây nối cống luân hồi move đi các vùng bao field di dịch (kiểu trỏ như dính chật mớ `String` so với việc rút xẻ bầy rác integer lấp ló xài dc `Copy` trait), đặc quyền bá chủ ownership của dăm ba vùng field rạch gân bị bồng luân dịch sang mảng struct thể bọc mới toanh. Áp đảo hoàn kiếp, phôi struct nguồn nhung (`user1`) dính đòn tê liệt bại hoại một nửa (partially invalid). Kỹ sư hoàn cấm ko được bén vạch truy nạp đến mấy vùng field bồng đi lạc vào `user1` chả màng đến quyền gọi vay bợ vay mượn `user1` xem ra một cái cục tổng hành bọng kén y nguyên nữa. Luật chỉ xả ban xài chọc riêng lẻ mấy đống field của `user1` nếu loại đấy là `Copy` (chẳng hạn `sign_in_count`) với phần bị đè đầu overwrite ghi chèn rõ tại cái lố phôi kia thai vạch từ khâu khai báo mới.

**Question 2:**
en: A `Unit-like Struct` has no fields (e.g., `struct AlwaysEqual;`). Architecturally, why would a compiler construct provide and allow data structures that hold absolutely no data? Provide a trait-driven rationale.
vi: Phân cục hạch Unit-like Struct k rớt 1 hạt mảng lĩnh field ngòi nào (như này kìa: `struct AlwaysEqual;`). Chẻ mổ xương trụ hạ tầng bám trụ kiến trúc, lý nào trình Compiler chắp đẻ cống nạp hẵng cho phép tồn định thứ khung nạp kết cấu data rác vô duyên mà trong lòng chả ôm giữ tí máu data dữ nào trọc trơn cả? Chuốc biện cớ có màu mùi bám rễ Trait nện thẳng vạch phán.

**Answer 2:**
en: A unit-like struct takes up perfectly zero bytes of memory at runtime. They are essential for modeling pure behavioral traits. Sometimes you only need an empty struct to implement a specific `trait` (an interface defining behavior), especially for type-state programming or strategic mock objects. The struct acts merely as a compile-time "marker" or a namespace token to associate functions (methods without a state context `self`) upon, allowing generic architectures to dispatch correct algorithms strictly via zero-cost static type checking instead of dynamically querying variables at runtime.
vi: Thể rỗng ruột kén unit-like struct đớp ráo trọn trị giá không-tuyệt-đối gầm rỗng memory byte zero tại dòng xoay lúc chương trình thở (runtime). Chún chúa ngự bám gốc cốt trần định danh duy có phần nọc mô phỏng hình mẫu hành vi traits đính kèm. Đôi mốc ngoặt, dân thợ máy chỉ đần mặt ngập chỉ vào struct trắng nhẵn cốt là nhắm làm ngòi phôi đập implement nện vô cái mấu chốt trait Interface đặc chẻ lối hành xử (behaviors) nặn vào thớt type-state programming hay giăng mạng màng chằng lưới Mock object chiến thuật ảo. Struct mượn hồn kén như thứ điểm xuyết Marker phím danh chùm không gian hàm namespace gồng móc gán đống functions function (loại ngắt cái tã context rễ state của `self`) đan theo mạch liên hiệp kiến trúc tổng, rạp chiếu generic tha hồ rải binh phát điệu logic algorithms mà nạp thuật chỉ thông luồng thông hành Zero-cost thẩm tra loại biên dịch chứ bỏ qua khâu lục tìm rác biến runtime mỏ neo cực lực.

---

## Level 5: Evaluating

**Question 1:**
en: Object-Oriented Programming (OOP) languages bind state (fields) and behavior (methods) intrinsically natively inside a "Class" definition block. Rust deliberately separates the state (in `struct`) from the behavior (in an `impl` block). Defend this design partition. Does it result in superior codebase scalability or just disparate boilerplate?
vi: Quần hùng Ngôn ngữ Lập trình Hướng đối Tượng OOP (Object-Oriented Programming) trói ghịt cứng trạng thái (fields) và đòn thế thói hành vi (methods) móc dính vô nguyên rễ khăng khăng nương theo định danh Class. Rust lì lợm nạy chia tách bung bét trạng thái tống lùa trại (vô `struct`) nép thân lánh xa nhóm định đoạt hành vi bứt rứt tung mẻ (ngoài ô khối `impl`). Bào chữa cho cái thiết kế bẻ lái chia rẽ ngầm phá này. Liệu trò phân hóa phôi phân rã dạt cho ra mớ vướng bận lầm cùi chuối boilerplate rời rụng không gắn liền thay cho bộ scale nền cốt codebase xưng bá hùng mạnh không?

**Answer 1:**
en: 
Defending the partition: Rust's separation of `struct` and `impl` prevents the tightly-coupled monolithic "God object" anti-patterns often bred in traditional OOP.
- **Modularity via Multiple Impl logic:** You can partition functionality across multiple `impl` blocks logically, or even conditionally compile them. If behavior was baked into the `struct` itself, splitting it up across different files or crates would be syntactically restricted.
- **Trait Implementations:** Detaching behavior enables Rust's brilliant trait system. Because `impl` blocks are disjoint, you can augment *any* external structure created by *anyone else's* library by writing an `impl MyTrait for ExternalStruct` block in your code. With a unified `Class` file in OOP like Java, you cannot add intrinsic behavior natively or neatly without wrestling heavy inheritance mechanisms or wrapper decorators.
Thus, while it superficially feels like disjoint boilerplate initially, separating data layout from behavioral implementations results in massively superior scalability, fostering clean separation of concerns and limitless composability.
vi: 
Biện hộ bào chữa phe chia chẻ: Quy chiếu chia lối tẽ bẻ của Rust đục vỡ chia bôi 2 đường `struct` và `impl` chống lại hội chứng lây nhiễu bệnh bám trụ quái thủ chùm hạch thần anti-pattern "God object" kềnh oải chúa đẻ rác cõi môn truyền thống OOP thắt nòng.
- **Module Tính Phép thông qua Phù hiệu Multiple Impl**: User có phép bẻ xỏ mớ xài thuật dồn thắt đống đụn cho bung vào ti tỉ khối riêng `impl` định luận phân cách nhau logic, thả kịt Compile ép thắt lụi linh hoạt. Cặp chướng khi gom hạch nhồi behavior chết chùm vô cùng trong cấu block ruột `struct`, băm phá chia nhỏ theo vùng địa giới file lẫn crates ngoài thành thứ nát bét xiết rào ngữ pháp cực căng.
- **Phê phê Tự do Trait System**: Tha hóa tung rời thềm behavior dọn nền đẩy ngất ngây con ác mộng thần thánh trait siêu cấp. Tách tháo ra từng `impl` khối đan dập rời, Kĩ thuật gia rẽ ngòi thêu bạt mạng cài dặm vô tính năng lên *mỗi cấu hình trúc màng ngoài External Struct* do *tay mơ rác 1 thằng khứa nào khác chế tác library* mà ung dung đâm chêm 1 cú xọc ngoắc `impl MyTrait for ExternalStruct` block chấn động ngầm tại codebase cá thể. Chìm nổi nhúng bóng 1 mô phôi màng lưới Class đóng ghim như thể Java, trượt xa ko tài bơi thêm nổi hành vi nội ngầm cho sạch đẽ mà chẳng gồng kẹt vô ổ gỉ inheritance ức ép mòn hạch di truyền lẫn decorator wrapper phiền cấn đụng nóc.
Đụng nấc kết thì, tráng lớp nhìn ngó ban bề cho cái mác boilerplate phân rã tách đoạn rườm rịch, nứt phôi đoạn data gá layout hục hặc đâm ra các nhóm implementation gieo cấy lùa lại mang cho cả gia đồ mồi nhen codebase bộ bành scale dãn phóng khổng lồ tột vạn, vuốt châm mạch lằn ranh trách vụ separation of concerns rành mạch tạo lối composability ghép mồi vươn mầm ko điểm kết.
