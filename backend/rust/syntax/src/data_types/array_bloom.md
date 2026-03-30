# Arrays in Rust - Bloom's Taxonomy

## Level 1: Remembering

**Question 1:**
en: What is the defining characteristic of a tuple compared to an array in Rust?
vi: Đặc điểm cấu thành quan trọng nhất của Tuple so găng với Array (Mảng) trong lãnh giới Rust là gì?

**Answer 1:**
en: An array must contain elements of the exact same data type and has a fixed length. A tuple also has a fixed length, but it can contain a mixture of completely different data types.
vi: Một Mảng (array) buộc kẹt nhét cứng các thành viên chỉ với một hệ mã type data chung duy nhất đồng thời đóng cứng cọc chiều dài độ lớn. Một ống Tuple (Tuple) cũng bị bóp hẹp chặt chiều dài k cho phình to, cơ mờ nó có khiếu nạp nhồi lộn xộn các khúc type data nhão nhẹt sai pha khác bè với nhau rầm rập k cấm cản.

**Question 2:**
en: How do you declare an array of type `i32` with exactly 5 elements? What is the type signature syntax?
vi: Bạn gọi trận báo danh list mảng Array theo khuôn phép type `i32` ốp chặt đóng cứng 5 phần tử con? Hình nét cú pháp chữ ký type là sao?

**Answer 2:**
en: You specify the type signature inside square brackets containing the type, a semicolon, and the length: `[i32; 5]`.
vi: Trổ cú pháp ấn định định tính ngầm trong cái lồng ngoặc vuông vức gồm cụm type, theo sau chèn chốt dấu chấm phẩy kéo số đo length của mảng: `[i32; 5]`.

---

## Level 2: Understanding

**Question 1:**
en: Explain where arrays are allocated in memory compared to standard Collections like Vectors (`Vec<T>`).
vi: Lý giải vùng căn cơ tọa lạc cắm rễ phân chia cấp phát nơi chốn mảng Array đóng băng dạt lề so sánh với phân loại Collections tiêu biểu Vector (`Vec<T>`).

**Answer 1:**
en: Because arrays have a completely fixed, known size at compile time, their memory is allocated directly on the Stack rather than the Heap. This makes array access extremely fast. Vectors, conversely, are allowed to grow or shrink dynamically, so their varying data must be allocated on the Heap.
vi: Chính hỡi việc đám mảng array bị đóng ngoàm bộ kích cỡ đo kẹp số cứng lúc châm mồi Compile, cục đất memory của mảng quăng trực tiếp lên phễu Stack cọc móng lẹ làng chứ ko chui rúc vào vùng Heap xôi thịt. Ngón bợ này vút phi thân luồng truy sát vô memory nhanh tẹt ga. Phía chống cự Vector được gỡ kim cô rào kẽm cho thoải tay bơm mập hay xẹp luống tự do theo thời gian dạo Runtime, cho nên đống phôi chứa mảng biến động của nó tấp bãi bị ném ngả rầm chôn xuống hầm Heap để chờ cấp phát.

**Question 2:**
en: What is the syntax trick for initializing an array with the same repeated value (e.g., creating an array of 100 zeroes)?
vi: Trò tà thuật Syntax nảy số khai mạc mảng Array ốp đầy nghẹt một giá trị lập lặp theo khuôn (vd như khui ra dàn 100 con số 0 ghim trọn)?

**Answer 2:**
en: You use square brackets, put the repeated initial value first, followed by a semicolon, and then the total number of elements desired: `let arr = [0; 100];`.
vi: Nhấm thả mở khung Bracket Vuông `[]`, móc thả số giá trị sẽ sao bản nhân lặp ra đặt chỗ trước thềm, lật đật theo đuôi chốt dấu phẩy chấm `;`, và trút con số lượng tổng các phần tử nhãn khao khát ra sau điếm cùng: `let arr = [0; 100];`.

---

## Level 3: Applying

**Question 1:**
en: Write code to create an array holding the names of the four seasons as strings, and then safely use safe matching to access an element without risking an out-of-bounds panic crash.
vi: Viết xả code chuốc lập ra một array mảng quây quần tóm tên nhãn của 4 mùa thời tiết (Seasons) đúc làm chuỗi String, rồi nắn gân xài mẹo safe-matching an toàn thò tay nặn moi thử phần tử mà ko bị lụi nguy cơ ôm boom panic vỡ màn do thọt chỉ mục Out-of-bounds quá đà biên nhám.

**Answer 1:**
en: We can use `.get(index)` which returns an `Option<&T>`. This allows us to handle out-of-bounds indices securely via a `match` or `if let` block without crashing.
vi: Tạt tấp sử dụng phay ngàm hàm `.get(index)` nôn oẹ ra lẫy bọc `Option<&T>`. Tùy bút này cho chúng dân nắn nắn các ngón chỉ mục đánh chệch thọt ra ngoài lề rào một cách an tâm nhờ móc thắt màng bọc `match` hay nhịp `if let` khối khỏi lo đứt nhịp đứt bóng rập màn.

```rust
fn main() {
    let seasons: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"];

    // Accessing index 2 (Autumn)
    let index = 2; // Try changing to 10
    match seasons.get(index) {
        Some(season) => println!("The season at index {} is {}", index, season),
        None => println!("Index {} is securely handled as out of bounds!", index),
    }
}
```

**Question 2:**
en: Destructure (unpack) the first two elements of a 4-element integer array into individual variables, and explicitly ignore the remaining two using a wildcard symbol.
vi: Chiết bẻ gãy rã (Destructure / Unpack) cặp mở hàng hai phần tử nòng cốt của một mảng integer ôm 4 cụm phần chui rúc vào biến đơn tách bầy cá nhân hóa, và bỏ bom công khai gạch sọc phớt lờ rũ mạt 2 đồng đội cặn đáy xui dạt nhờ ký tự thay thế wildcard mờ.

**Answer 2:**
en: You can use array pattern matching and the double-dot `..` wildcard to ignore the suffix portion of the array.
vi: Cào nháp bung hệ Pattern Matching mảng kết nối đôi chấm cặn `..` là ký hiệu Wildcard hất bụi lờ lấp nguyên đoạn đuôi lằng ngoằng của tổ đội đó.

```rust
fn main() {
    let numbers = [10, 20, 30, 40];
    let [first, second, ..] = numbers;
    println!("Extracted: first is {}, second is {}", first, second);
}
```

---

## Level 4: Analyzing

**Question 1:**
en: Analyze the core compile-time guard rails Rust provides when indexing standard arrays (e.g., `arr[10]`). How does the compiler respond if you hardcode an index that exceeds the array's defined length, compared to using dynamic variables to index them incorrectly at runtime?
vi: Vạch mặt điểm dò xét nòng cốt bảo hộ tường chắn biên dịch (Compile-time guard rails) mà tay thợ Rust cung hiến khi truy lần theo chỉ mục mảng thường (kiểu nhảy `arr[10]`). Phương trình cỗ máy Compiler lật bài lật mâm báo hiệu sao nếu bạn chốt cứng (hardcode) con index vượt cọc chiều đo mảng định sẵn đã ghim, đem tạc với hình xài cục biến số lơ lửng bồi chọc thủng sai trái chệch choạng khi ván bài rẽ khúc chạy ngầm đoạn Runtime?

**Answer 1:**
en: 
If you perform out-of-bounds indexing with a literal or constant (e.g., `arr[5]` on an array of length 3), the Rust compiler evaluates it at compile time and deliberately aborts compilation, halting the bug before you can even run the program.
However, if you index via a dynamic run-time variable (e.g., `let i = get_user_input(); arr[i];`), the compiler cannot statically predict the value of `i`. In this scenario, Rust injects a runtime boundary check. If `i` exceeds the array's length during execution, the program triggers an immediate panic (an intentional crash), neutralizing undefined behavior, whereas a language like C might blindly sweep into the adjacent memory and read foreign garbage data or compromise security.
vi: 
Hễ trượt tay mò chỉ mục vượt rào cùm bằng phím gõ chết giá trị tĩnh Constant (nthe kiểu ép `arr[5]` xộc hông cái mảng kẹp độ dài có 3 mẩu), vị Thẩm phán Compiler định vị nhổ sào ngay đoạn compile biên dịch và lập án chặt ngang khựng sập biên mã, tóm rệp đoạt lệnh ngắt ngay khi game còn chưa vô màn khởi chạy test.
Quay đầu, nhược bằng tóm index qua trung gian biến biến hoán run-time chuyển động liên thanh (đơn cử `let i = get_user_input(); arr[i];`), nòng tĩnh Compiler tịt ngòi bó tay k phán đoán đc thần chú lõi của cục `i` chứa cái mộng số gì. Ngay nấc kịch bản này, Rust lén nhét cấy nặn một cổng dò check Runtime ranh giới canh cửa chặn bắt. Lỡ khúc vận hành mà biến `i` cuộn tít quá mốc mảng cọc rào, đứt cầu dao cho ghim nổ Panic crash thảm cảnh cố kình tức lự (đánh rớt toàn cục), thủ tiêu rủi ro tàng hình undefined behavior độc tính, trong khi đó rẽ nhánh cái lũ Code C cũ mèm thì mụ mị đâm trượt luồn vô sâu lề memory đất kế cận bốc mẻ rác thối data nước lân bang hay lủng mạng thủng rào security phá hoại sạch.

**Question 2:**
en: Contrast the strict type signature `[T; N]` against slices `&[T]`. How does moving from a hard array `[i32; 5]` to a slice `&[i32]` architecturally impact function reusability?
vi: Vặc nhau nẹt nòng chữ ký rập cọc Strict Type `[T; N]` nghênh so găng với hàng họ lũ lát Slices Sắt `&[T]`. Kể sự tình lúc dời đòn bứt khỏi con Mảng cứng băng `[i32; 5]` vọt trườn tạt qua bộ Slice trượt `&[i32]` thì làm rêm cục diện ảnh chấn hạ tầng tái xài tái mượn Functions của chu trình lập trình Architecture nhường nào?

**Answer 2:**
en: The array signature `[T; N]` encodes the physical length `N` directly into the actual Type definition. Consequently, a function taking `[i32; 5]` cannot mathematically accept an array of `[i32; 6]`; they are completely divergent disjoint types architecturally. This kills function reusability for processing different-sized arrays. By exposing a slice signature `&[T]` instead, you drop the physical length bound. A slice dynamically pairs a memory pointer with length metadata as a "fat pointer". A function accepting `&[i32]` can now process subsets or complete references to arrays of *any* arbitrary length generically, profoundly increasing structural versatility without sacrificing any speed.
vi: Mộc triện nhãn hiệu Array `[T; N]` ghim tạc đục nguyên khối chiều đo xác thịt `N` nhét thẳng tận bộ nòng màng lọc bảng định type phân loại. Nghịch chướng kéo đến, một khối hàm xin sỏ nhai vào cái `[i32; 5]` đố cha toán học mà nó nuốt lọt thêm được mảng đúc `[i32; 6]`; về mặt hạ tầng loại hình đúc tụi nó phân mảnh lìa rời cách cõi hoàn toàn. Món cục súc này chém ngang cổ chặt đường sống thói reusability tái sử dụng viết rút vòng hàm định vị xử hàng mảng phình xẹp tạp nham lúc cạn lúc đầy. Khi vén màn rút quân gá vào dùng bộ chữ ký lát cắt Slices Sắt `&[T]` mượn dòm, thanh thoát cởi trói thả dây gánh nặng rào chắn chiều dài. Một Slice đu bám liên hồi khâu ghép một móc chỉ điểm cọc Pointer và đè cục Metadata chứa chiều mảng ráp nên "Phễu Mỡ Mập Fat Pointer". Function nghênh tiếp `&[i32]` bỗng chốc đớp sướng đớp mượt rập mảng gom chùm của bất kể mọi chuỗi tham chiếu lớn nhỏ size *Bất Cứ* một độ dài mảng vặn vẹo nào quy theo tổng quát (generically), khuếch đại nảy lửa tính đàn hồi cấu trúc mà khỏi xén một xíu điểm hụt nhịp vận cước tốc độ nào cả.
