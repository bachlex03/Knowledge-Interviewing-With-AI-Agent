# Shadowing in Rust - Bloom's Taxonomy

## Level 1: Remembering

**Question 1:**
en: What is shadowing in Rust?
vi: Tính năng che khuất (shadowing) trong Rust là gì?

**Answer 1:**
en: Shadowing is the ability to declare a new variable with the same name as a previous variable. The new variable "shadows" the previous one, meaning you can only access the new variable's value from that point forward within the scope.
vi: Tính năng che khuất (shadowing) là khả năng khai báo một biến mới có cùng tên với một biến trước đó. Biến mới "che khuất" (shadows) biến trước đó, nghĩa là bạn chỉ có thể truy cập giá trị của biến mới từ thời điểm đó thay vì nhìn thấy bên trong phạm vi.

**Question 2:**
en: Which keyword is mandatory to use when shadowing an existing variable?
vi: Từ khóa nào là bắt buộc phải sử dụng khi tiến hành che khuất (shadow) một biến đã tồn tại?

**Answer 2:**
en: The `let` keyword is mandatory. If you attempt to reassign the value without `let`, you are attempting mutation, not shadowing.
vi: Từ khóa `let` là bắt buộc. Nếu bạn cố gán lại giá trị mà không có `let`, bạn đang cố tình tái gán thay đổi (mutation), chứ không phải shadowing.

---

## Level 2: Understanding

**Question 1:**
en: Explain the key differences between Variable Shadowing (using `let` again) and Variable Mutability (using `mut`).
vi: Giải thích điểm khác biệt mấu chốt giữa che khuất biến (Variable Shadowing - dùng lại `let`) và Biến có tính đa biến đổi (Variable Mutability - dùng `mut`).

**Answer 1:**
en: 
- **Shadowing**: Creates a brand new variable. Because it's a new variable, you can change the underlying data type (e.g., from an integer to a string), while keeping the variable name exactly the same. The previous value still safely points to its memory chunk but gets detached from the name bindings going forward.
- **Mutability (`mut`)**: Modifies the value within the exact same memory space and variable. You **cannot** change the data type; you can only alter the value inside.
vi: 
- **Shadowing**: Tạo ta một object biến y mới mẻ tinh túy. Vì tính chất này, bạn có thể biến đổi hệ type (vd, từ int -> string) thoải mái mà cái tên tham chiếu không bị ảnh hưởng. Giá trị đằng trước vẫn ở đúng phần chunk memory, nó chỉ bị cắt đứt cái tên đối với người viết tiếp.
- **Mutability (`mut`)**: Gán trực tiếp data lên đúng ngay bộ ô nhớ đã tạo. Bạn **ko thể** đổi type được. Chỉ đổi giá trị mà thôi!

**Question 2:**
en: Describe what happens when a variable is shadowed inside an inner nested block scope (`{}`). Does the shadowing persist outside the block?
vi: Miêu tả chuyện gì xảy ra khi khai thác shadowing vào bên trong lớp function nhỏ chứa ngoặc nhọn `{}` tròng vào nhau. Phép gán che dấu có tiếp nối bay màu ra ngoài khu vực block ko?

**Answer 2:**
en: When a variable is shadowed inside an inner block, the new shadowed variable is exclusively active until the end of that inner scope. Once the inner scope ends (at the closing `}`), the shadowed variable drops, and the original variable (from the outer scope) comes back into context unchanged.
vi: Tại block scope cục bộ nhỏ (inner), khi tạo biến thay thế thì variable đó sống tại riêng mình block cho tới lúc dấu nháy khép lại `}`. Tại thềm tắt hoạt động, phần inner shadow biến mất, và giá trị biến trước đó tại điểm biên trả về nguyên gốc vạch ban đầu nguyên si.

---

## Level 3: Applying

**Question 1:**
en: Write a code example demonstrating how shadowing allows you to easily transform a string holding "    spaces    " into its length as a number `usize`, using the same variable name.
vi: Viết mã hiện trạng diễn phỏng về độ hiệu năng của shadow giải quyết một chuỗi mang tên "    spaces    " lùi đổi sang format kích thước `usize` của nó, áp đặt vẫn xài chung nguyên vẹn một danh tính biến.

**Answer 1:**
en: By leveraging shadowing, we can avoid naming the string `spaces_str` and the length `spaces_len`, and instead reuse the identifier `spaces` for differently typed concepts successively.
vi: Bằng cách mượn shadowing, bạn gạt bỏ hẳn việc đẻ ra dư `spaces_str` và `spaces_len`, thay cho tái ứng một mã danh xưng tên `spaces` với multi format loại data cách biệt hoàn toàn.

```rust
fn main() {
    let spaces = "   "; // Currently a string slice (&str)
    let spaces = spaces.len(); // Shadowing! Now an integer (usize)
    println!("Number of spaces: {}", spaces);
}
```

**Question 2:**
en: Suppose you have a mutable variable `let mut price = 100;`. Write code to apply a temporary discount in an inner scope using shadowing without modifying the original price permanently.
vi: Lường như bạn sở hữu định khoản có tính sửa liên tiếp là `let mut price = 100;`. Triển giao lệnh ứng dụng gán sale ngầm cục bộ trong inner scope trổ tài dựa dựa trên shadowing đảm bảo tài sản gốc giá bán cố định ko bị can hiệp lỗi nhai.

**Answer 2:**
en: We shadow `price` within an inner scope. The mutated state doesn't reflect because we used shadowing rather than reassignment.
vi: Xâu chuỗi trỏ gán shadow qua cục bộ bên trong phần vi phạm của scope kia. Sự tái diễn biến động sẽ k rọi ra biên ngoại do ứng dụg Shadow chứ ko phải Reassignment.

```rust
fn main() {
    let mut price = 100;
    
    {
        // Shadowing `price` inside this block
        let price = price - 20; 
        println!("Discounted price: {}", price); // Prints 80
    }
    
    println!("Original price globally: {}", price); // Still prints 100
}
```

---

## Level 4: Analyzing

**Question 1:**
en: Analyze why the compiler throws an error if you try to replicate the shadowing behavior with a mutable string type change. Provide the errant code and map the architectural issue.
vi: Giải thích mưu sự tại sao trình biên dịch mắng lỗi báo đỏ trong khi chọc ngoáy shadow bằng cách đổi định dạng type trên tham chiếu chuỗi bị đổi biến.

**Answer 1:**
en: 
Code:
```rust
let mut spaces = "   ";
spaces = spaces.len(); // Error: expected `&str`, found `usize`
```
Architecturally, variables declared in Rust have a fixed, immutable type determined at compile time to ensure type safety. Reassignment using `=` assumes mutating the memory contents of the *same* variable. Because a pointer object (`&str`) and a basic integer (`usize`) possess fundamentally unaligned memory geometries and type identities, Rust outlaws this implicitly. However, shadowing (`let spaces = ...`) bypasses this rule by completely abandoning the old memory slot and allocating a distinct variable tracking table item logically on the AST.
vi: 
Đoạn Lỗi: 
```rust
let mut spaces = "   ";
spaces = spaces.len(); // Error: expected `&str`, found `usize`
```
Về mặt hình kiến trúc, variables khai triển ở Rust sẽ dính chặt ghim tạc ở phần dạng dữ liệu (memory fixed data type). Xài phép thay hàm gán `=` trình compiler buộc ép thay data đè lên trên chính khung size biến số cũ kia. Tại vì Object string và hàm data số rời rạc không ăn nhập gì nhau bộ ô nhớ lẫn type danh tính, Rust khước từ ngầm hoàn toàn. Cơ mà shadow qua việc add chữ `let spaces` tháo rào cản gạt đi data memory trước rớt ra lề và khởi tạo 1 block biến hoàn toàn khác trên AST nhưng giữ nhãn mặt nạ tên để tương tác thôi!

**Question 2:**
en: Compare shadowing against naming conventions (e.g., `value_int`, `value_str`) in variables. How does shadowing lead to cleaner software design patterns in transformations pipeline implementations?
vi: So tỉa phép gán bóng gộp Shadowing vs các chiêu convention đặt gán tên biến dài lằn ngoằng (`value_int`, `value_str`). Đề đường ra tại sao kỹ nghệ kiến trúc shadow mở cõi cho nền thiết kế software sạch xịnh mịn với các chuỗi data transfer map nối tiếp?

**Answer 2:**
en: In sequential data transformation pipelines (like receiving a string from CLI, parsing it to integer, and stripping whitespace), standard strictly-typed languages force creating uniquely numbered aliases (e.g., `input`, `trimmed_input`, `parsed_input`). This fills the namespace with transient values, risking a developer mistakenly using `trimmed_input` instead of `parsed_input` later down the line. Shadowing solves this explicitly: by constantly rebinding the name `input`, the outdated state mappings become completely inaccessible to the developer. The compiler prohibits accessing the wrong pipeline transformation stage, effectively cementing type-state pattern paradigms right into the variable allocation table.
vi: Các chuỗi phân luồng chuyển giao (pipeline như tóm Input từ Terminal, Cắt khoảng trắng dòng đầu chữ, và xắn map chữ qua số Type Int), ở nền tảng ngôn ngữ gõ strict nghiêm ngặt khác buộc ép kĩ sư nhân bản vô giới hạn mớ tên (ví dụ: `input`, `trimmed_input`, `parsed_input`). Cảnh này làm ngập nát name variable block, kèm rủi ro con ng lọt hố do lỡ call đè tên cũ (vd xài `trimmed_input` thay cho bản chót `parsed`). Bóng che shadowing làm trùm cục này: Bởi vì nó thâu tóm nhãn lột xác danh tính `input` mãi, mớ code format ngả biến trước đó bay màu lập tức do đè. Rust compile triệt đường người lập trình viên xài lộn tên. Điều đó xây móng thiết kế Design paradigm type-state vĩnh cửu cứng ngắc ngay lên trên khung memory lưu trữ map của compiler.

---

## Level 5: Evaluating

**Question 1:**
en: Do you think Rust's decision to allow shadowing contributes to secure programming, or does it introduce a potential surface area for confusion and bugs? Support your stance.
vi: Thẩm vấn riêng luận điểm của bạn: Định luật Rust ban hành Shadow có thực sự tạo lợi thiết thực về mã lập trình bảo vệ an toàn (secure program) không, hay là rước dạo hoạ vào nhà làm lú lân và ngập bugs người theo sau? Bảo vệ quan điểm.

**Answer 1:**
en: Shadowing fundamentally enhances software security by minimizing state leaks. Although newcomers may mistakenly read a shadowed outer scope variable if they ignore indentation patterns, the overall utility prevents the far more dangerous risk of "stale data usage". In security-critical paths—like parsing untrusted user inputs—shadowing forces the replacement of an unvalidated token payload instance with securely sanitized verified wrapper instances of the exact identical name. Attackers or negligent developers are mechanically deterred from accessing the unvalidated root source, as the shadowing paradigm renders the unsanitized variable reference unreachable.
vi: Bóng quyền (Shadow) giải tỏa hoàn toàn tính an ninh lập trình, gỡ rủi ro memory leak thông qua state ảo. Đồng quy lúc lính mới vô nghề rớt bẫy ko đọc nổi mớ rào cản nội/ngoại vi scope gây mâu thuẫn, bản chất tiện lợi bao trùm của Shadow che triệt đi hiểm họa dã man là "xài nhầm rác data hết date". Ở ngưỡng hệ thống an ninh tối cao nhạy phảm (như phân tích user payload input ko rõ độ an toàn)—shadow gạt bỏ rác unvalidated đổi lấy chuỗi kén object verified (có bảo kê data check qua an ninh) mang trọn cái tên danh tính gốc. Tụi hack lẫn developer ẩu tả bị dập tắt tia vi phạm vào lõm bộ nhớ do rào cản này làm biến số gốc bay khỏi lưới radar compiler.

**Question 2:**
en: Evaluate the cost-benefit analysis of Variable Re-use (Shadowing) against Immutable Architectures enforced by pure functional languages like Haskell. Should modern memory-safe applications enforce zero-shadowing rules?
vi: Cân định lượng thiệt/hơn ở cách mượn dùng Tái sử chiết lọc Biến qua Shadowing so hạch với Lối Kiến Trúc Dữ liệu Kiên định (Immutable Architecture - Cấm biên) kiểu như trong dân hệ Pure Function (Haskell). Lệu dân chạy phần mềm memory-safe hiện hành có nên ra pháp lệnh cấm toàn phần sử dụng ngáo kỹ nghệ Shadowing ko?

**Answer 2:**
en: The paradigm in pure functional languages (Haskell) mandates mathematical immutability whereby any modification generates inherently new concepts and contexts without obfuscating the old bindings. While formally elegant, it demands intense mental overhead for system-level programming. Rust's Shadowing elegantly navigates a middle ground: it provides an immutable paradigm essentially (as the shadowed bindings are fully detached from mutations), yet it caters ergonomically to human readability in highly linear procedures. Enforcing a zero-shadowing rule via linting would devolve Rust code back into the boilerplate chaos of numbering values or aggressive unreadable method chaining. Thus, the practical engineering gains in cognitive clarity deeply outweigh the ideological purity of disjoint variable allocations.
vi: Thế giới ở cõi Functional (Hàm nguyên thủy - Haskell) thiết ban chuẩn hóa bất biến toán học tột đỉnh, hễ cứ đập xé type chẻ data đều bắt buộc quy lại một object tên mới tinh sạch, k bao giờ đụng che khuất cội nguồn cũ. Dẫu mỹ miều học thuật, nó áp khung ép kỹ sư hao mòn suy lý ở những phần mem system-level lõi. Shadow của Rust len được con ngõ luồng lách ăn trung hòa: nó giữ quy chuẩn tính bất biến cao (vì biến cũ rớt thẳng ra lề bị detach ko dính tới memory mutablity), lại vừa chuốc sướng cặp mắt ở mạch logic gõ dọc. Phát lệnh Cấm gõ zero-shadowing thông qua quy ước linting team sẽ tống cổ Rust lộn về cục rác C++ lặp mã hay nối chuỗi hạch toán chà chanh chain hàm khốn đốn mắt độc. Thành ra, độ rõ ràng ở chu trình tri giác cognitive người ghi chép code bỏ xa tính ảo vọng thuần túy lý thuyết định danh variable chia cắt.
