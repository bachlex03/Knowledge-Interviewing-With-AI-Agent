# String Slices (&str) in Rust - Bloom's Taxonomy

## Level 1: Remembering

**Question 1:**
en: What is the data type for a string slice in Rust?
vi: Kiểu dữ liệu cho một string slice (phần cắt chuỗi) trong Rust là gì?

**Answer 1:**
en: The data type for a string slice is `&str`.
vi: Kiểu dữ liệu cho một string slice là `&str`.

**Question 2:**
en: How do you create a string slice from a `String` variable `s` starting at index 0 and ending at index 4 (exclusive)?
vi: Làm thế nào để bạn tạo một string slice từ một biến `String` `s`, bắt đầu tại chỉ mục 0 và kết thúc tại chỉ mục 4 (không bao gồm 4)?

**Answer 2:**
en: You create it using the range syntax: `&s[0..4]`.
vi: Bạn tạo nó bằng cách sử dụng cú pháp phạm vi: `&s[0..4]`.

---

## Level 2: Understanding

**Question 1:**
en: Explain structurally what a string slice actually is under the hood. Does it own the string data?
vi: Giải thích cấu trúc thực sự của một string slice ở bên dưới. Nó có sở hữu (own) dữ liệu chuỗi không?

**Answer 1:**
en: Under the hood, a string slice (`&str`) is a "fat pointer" consisting of two things: a pointer to the starting byte of the string data in memory, and the length of the slice. It does not own the data; it only borrows a view into a portion of a `String` or static memory.
vi: Dưới màn hình, một string slice (`&str`) là một "fat pointer" mập mạp bao gồm hai yếu tố: một con trỏ trỏ đến byte bắt đầu của dữ liệu chuỗi trong bộ nhớ, và chiều dài của slice đó. Nó không sở hữu (own) dữ liệu; nó chỉ mượn (borrow) một góc nhìn vào một phần của `String` hoặc bộ nhớ tĩnh.

**Question 2:**
en: Why are string literals automatically of type `&str` instead of `String`?
vi: Tại sao các hằng chuỗi (string literals) tự động có kiểu `&str` thay vì `String`?

**Answer 2:**
en: String literals are hardcoded directly into the compiled binary of the program. Since the binary is loaded into read-only memory, the program simply needs a pointer and a length to read that text. Therefore, string literals are immutable slices (`&'static str`) pointing to that specific location in the binary, rather than a heap-allocated `String`.
vi: Các hằng chuỗi (string literals) được mã hóa cứng trực tiếp vào tập tin thực thi nhị phân của chương trình. Vì tập tin nhị phân được tải vào bộ nhớ chỉ đọc, chương trình chỉ cần một con trỏ và độ dài để đọc văn bản đó. Do đó, các hằng chuỗi là các slice không thể thay đổi (`&'static str`) trỏ tới vị trí cụ thể đó trong tập tin nhị phân, chứ không phải là một `String` được cấp phát trên bộ nhớ heap.

---

## Level 3: Applying

**Question 1:**
en: Write a function `first_word` that takes a string slice `&str` as a parameter and returns a string slice `&str` representing the first word (up to the first space).
vi: Viết một hàm `first_word` nhận một string slice `&str` làm tham số và trả về một string slice `&str` đại diện cho từ đầu tiên (cho đến khoảng trắng đầu tiên).

**Answer 1:**
en: We can convert the string slice to bytes and iterate through it to find the index of the space character. Then, we return a slice of the string up to that index.
vi: Chúng ta có thể chuyển string slice đó thành các byte và lặp qua nó để tìm chỉ mục của ký tự khoảng trắng. Sau đó, chúng ta trả về một lát cắt của chuỗi cho đến chỉ mục đó.

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

**Question 2:**
en: Given a `String` `text = String::from("Rust Lang");`, call the `first_word` function from the previous question and print the result.
vi: Cho một `String` `text = String::from("Rust Lang");`, hãy gọi hàm `first_word` từ câu hỏi trước đó và in kết quả ra màn hình.

**Answer 2:**
en: You can pass a `String` object to a function expecting `&str` by creating a slice of the entire `String` using `&text` or `&text[..]`. This leverages Rust's Deref coercion.
vi: Bạn có thể truyền một đối tượng `String` cho một hàm đang mong đợi `&str` bằng cách tạo một slice của toàn bộ `String` bằng cách dùng `&text` hoặc `&text[..]`. Điều này tận dụng tính năng Deref coercion của Rust.

```rust
fn main() {
    let text = String::from("Rust Lang");
    let word = first_word(&text);
    println!("The first word is: {}", word);
}
```

---

## Level 4: Analyzing

**Question 1:**
en: Analyze the compilation error in the following code. Connect the error back to the relationship between the `String` owner and the `&str` slice borrow.
vi: Phân tích lỗi biên dịch trong đoạn mã sau. Kết nối lỗi này trở lại với mối quan hệ giữa chủ sở hữu `String` và sự vay mượn của slice `&str`.
```rust
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); 
    s.clear(); // This empties the String
    println!("the first word is: {}", word); // Error!
}
```

**Answer 1:**
en: The compiler errors out because `first_word` borrows an immutable reference (`&str`) from the mutable `String` `s`. `word` holds this slice. Calling `s.clear()` requires a mutable borrow of `s`. Rust's rules state you cannot have a mutable borrow while an immutable borrow is currently active (the `word` slice is used later in the `println!`). If `s.clear()` were allowed to empty the string, the `word` slice would become a dangling reference pointing to freed or invalid memory.
vi: Trình biên dịch báo lỗi vì `first_word` vay mượn một tham chiếu không thể thay đổi (`&str`) từ `String` `s` có thể thay đổi. `word` lưu giữ slice này. Việc gọi `s.clear()` đòi hỏi một lần mượn mutable của `s`. Các luật của Rust phát biểu rằng bạn không thể có một lần mượn thay đổi trong khi vẫn đang có một thiết lập mượn không thay đổi đang hoạt động (`word` lát cắt lại được xài tại `println!`). Nếu `s.clear()` được phép dọn sạch chuỗi, slice `word` sẽ bị biến thành tham chiếu lơ lửng trỏ vào memory đã xóa hoang.

**Question 2:**
en: Compare indexing a Rust `String` or `&str` by individual characters versus taking a slice byte-by-byte (e.g., `&s[0..2]`). What is the primary risk associated with slicing, given Rust strings are UTF-8 encoded?
vi: Đối chiếu việc duyệt chỉ mục lấy kí tự lẻ tẻ trong Rust `String` / `&str` với thao tác tạo các phần cắt dựa trên byte (`&s[0..2]`). Lường định hiểm họa cốt lõi gắn liền với thao tác lấy phần cắt slice, do chuỗi Rust định chuẩn theo UTF-8?

**Answer 2:**
en: Rust does not allow simple integer indexing (like `s[0]`) because UTF-8 characters can take up more than 1 byte (from 1 to 4 bytes). If you take a slice via byte ranges like `&s[0..2]`, you risk slicing right through the middle of a multi-byte character (like an emoji or Cyrillic character). If a byte slice doesn't fall on a valid UTF-8 character boundary, your program will completely panic and crash at runtime.
vi: Rust chặn bắt lập chỉ mục số nguyên đơn lẻ (cỡ `s[0]`) do cấu hình kí tự UTF-8 có lúc vượt quyền đòi đến hơn 1 byte (từ 1 đến 4 bytes lớn bé). Nếu bạn lấy phần cắt nhờ khoảng byte định lượng kiểu `&s[0..2]`, bạn dính lựu đạn chặt chém ở nửa chừng phần xác kí tự đa byte liền (như mấy cái hình mặt emoji hay kí tự Slavic/Tiếng Việt). Giả thiết khoảng byte đem chia ko đáp cánh sát biên giới của UTF-8 đúng chuẩn, luồng máy sẽ cuống lệnh panic xập nguồn ứng dụng tắp lự.

---

## Level 5: Evaluating

**Question 1:**
en: Defend the design choice in Rust to use both `String` and `&str`. Would the language be simpler or better if everything was just a heap-allocated `String`?
vi: Biện hộ kiến trúc trong Rust xài hai luồng `String` và `&str`. Có phải cấu hình language sẽ gọn gàng đơn giản hóa sạch nếu tất tật mọi con text chữ đêu chỉ là `String` phân bổ ở Heap thôi k?

**Answer 1:**
en: Using only a heap-allocated `String` would drastically simplify the learning curve, mimicking languages like Java or Python. However, it would destroy Rust's core advantage: high-performance systems programming. If every string was a `String`, every subroutine extracting logic words would demand its own costly heap allocation and subsequent deallocation. `&str` enables "zero-copy" string manipulation. You can parse a gigabyte-sized log file, extracting thousands of sub-strings (`&str`), without making a single heap allocation, simply by pointing to subsections of the original memory buffer block. That tradeoff defines Rust's zero-cost abstractions perfectly.
vi: Có, xài đặc dính một `String` cấp nằm ở Heap sẽ kéo thấp độ xoắn đường cong học hỏi vô biên, y như copy model bên ba ngôn ngữ Java với vPython. Cớ mà làm vỡ nát hoàn toàn siêu năng lực vô cực: tốc lực lập trình hệ thống phần cứng siêu hạng. Cứ mường tượng tất tần tật chữ đều chui qua `String`, nguyên thung lũng logic thuật toán mổ xẻ nội dung sẽ đòi phải bung ngân quỹ cấp phát Heap tàn khốc rồi hốt dọn rác đi chừng nấy thời gian. `&str` kích hoạt thao tác chặt chuỗi theo phép "zero-copy" - phân thân không tốn giọt tinh huyết. Kĩ sư có quyền scan 1 cục dữ liệu GB khổng lồ, băm vằm chia tỉ hệ thành hàng ngàn lát phần tử nhỏ (`&str`), không kích hoạt lệnh call Heap nào, mà đơn thuần dùng định vị tham chiếu góc nhìn tọa độ vào đúng khu Buffer lõi nguyên thủy. Ranh giới trao đổi này tạo hệ "Zero-cost Abstraction" hoàn hảo ko tì vết.
