# Pattern Matching in Rust - Bloom's Taxonomy

## Level 1: Remembering

**Question 1:**
en: What control flow operator in Rust allows you to compare a value against a series of patterns and execute code based on which pattern matches?
vi: Toán tử điều khiển luồng (control flow operator) nào trong Rust cho phép bạn so sánh một giá trị với một chuỗi các mẫu (patterns) và thực thi mã dựa trên mẫu nào khớp?

**Answer 1:**
en: The `match` control flow operator.
vi: Toán tử điều khiển luồng `match`.

**Question 2:**
en: What is the special wildcard pattern symbol used in a `match` expression to catch all remaining unhandled cases?
vi: Ký hiệu mẫu ký tự thay thế (wildcard pattern symbol) đặc biệt nào được sử dụng trong một biểu thức `match` để "hốt" gộp chộp lại bắt gọn tất cả các trường hợp chưa được xử lý còn lại?

**Answer 2:**
en: The underscore `_` symbol.
vi: Ký hiệu dấu gạch dưới `_`.

---

## Level 2: Understanding

**Question 1:**
en: Why must `match` arms be exhaustive in Rust? Explain what occurs if they are not.
vi: Tại sao các nhánh (arms) `match` trong Rust bắt buộc phải vét cạn (exhaustive)? Giải thích điều gì xảy ra nếu chúng bị hở kẽ (thiếu xót / không exhaustive).

**Answer 1:**
en: `match` expressions must be exhaustive so that every possible value a given type can take is explicitly accounted for and handled. If they are not exhaustive (for example, missing a variant of an enum), the Rust compiler will throw an error at compile time, outright refusing to build the program. This completely prevents unhandled edge cases from causing runtime panics.
vi: Tầng kiểm soát biểu thức `match` ép uổng phải thầu gom vét cạn trọn túi (exhaustive) vì nhằm cản che mọi khoảng biên có thể xổ rớt giá trị ra khỏi phôi bao bọc. Hễ sảy chân hở sườn kẽ hở vạch ko trọn ngàm (Đơn ví dụ như sút rớt mất mẩu con Variant bên ổ nhà enum chả ngó ngàng), tay Thẩm Phán Trình Compiler Rust tức tốc giáng búa trảm tung lên lỗi khét lẹt ngay kì hạn thềm Compile, ngúng nguẩy cự tuyệt cấm biên chối chặn rào sinh code vĩnh hằng. Sải đòn đó càn quét tận diệt sạch hố phân bùn lầy nọc độc chứa mầm edge-cases (biên lỗi hở) thoát luồn vươn bòi cắn sập gài mìn nhịp chạy tàn sát cõi runtime chấn chát.

**Question 2:**
en: Explain the difference in purpose and brevity between using a `match` expression and using the `if let` syntax.
vi: Móc phôi chẻ băm giải mộng rõ sự nhị biệt ở cung ứng chức phận (purpose) và tốc độ rành mạch ngắn họn (brevity) tại phe cắm nòng rập `match` nhét biểu thức và nhóm xăm trổ chiêu mồi cú pháp tóm gọn `if let`.

**Answer 2:**
en: A `match` expression is designed for exhaustively branching multiple logic paths based on all possible shape variants of a value. It's rigid but incredibly secure. In contrast, `if let` is designed purely for brevity as syntactic sugar when developers explicitly only care about matching *one* specific pattern and completely ignoring all others. It trades off the forced exhaustiveness check in exchange for removing the verbose boilerplate of an empty catch-all `_ => ()` arm.
vi: Một nòng `match` tạc khuôn đâm trổ chĩa mũi nhánh tẽ cạn tàu ráo máng rập logic đường kẽ rãnh đè nắn sập trọn mọi gốc nghiêng biến ảo variant bao quanh 1 ngòi hạch data. Thằng múa này trịch thượng hà khắc nhưng bù bằng uy dũng bá chủ tường thành tối tăm siêu khủng bảo mật. Nghịch phe cắm dao, đòn vót nanh `if let` bưng bệ thiết kế nhả mồi rút gọn tinh tươm hệt viên kẹo đường (syntactic sugar) xỉa xực mụ mị vào gân óc khi cái bộ hội tay đấm thợ gõ code chỉ khăm khăm săm sắn vớt gọn nhổm duy bật *MỘT* mẫu (pattern) cá thể chộp dính trúng thầu và phớt mỏ hờ hững đống bồi rác các rập thừa thãi khác đằng sau. Trò quái gở này trao đổi ngầm sự kiểm duyệt ép gông "chiếu vét cạn" để thế lót rút lõi bay cục tạ boilerplate gánh xiếc báo danh vách mù gác cổng `_ => ()`.

---

## Level 3: Applying

**Question 1:**
en: Write a `match` expression that takes an `Option<i32>` variable named `dice_roll`. If it is `Some(6)`, print "Jackpot!". If it is `Some` with any other number, print the number. If it is `None`, print "No roll".
vi: Chém gõ ra luồng khai ấn `match` thồ luồng tóm xí nhận cái cục biến `Option<i32>` treo biển `dice_roll`. Nếu nó ngửa nhào trúng đít bọc cục nòng `Some(6)`, gào văng rống lên "Jackpot!". Cỡ chừng bọc `Some` nhưng tọng lấp số lẻ khác rác khác, phang print con số đó nhả text. Xui rủi kẹt vô `None`, la báng "No roll".

**Answer 1:**
en: We use pattern matching with a specific value block `Some(6)`, a generic binding bind `Some(other)`, and the `None` catch.
vi: Triển sải chắp ngàm pattern matching cùng lúc xọc ngoắc khoanh tay số thẳng tĩnh tĩnh `Some(6)`, vớt biến đệm trượt mảng gá ngầm `Some(other)` trút chữ, là phệt đuôi chặn bọc bẽ kẹt hẻm `None`.

```rust
fn main() {
    let dice_roll: Option<i32> = Some(6);

    match dice_roll {
        Some(6) => println!("Jackpot!"),
        Some(other) => println!("Rolled a {}", other),
        None => println!("No roll"),
    }
}
```

**Question 2:**
en: Convert the given verbose `match` block handling an `Option<u8>` into its concise `if let` equivalent.
vi: Phá khuôn rập tái nặn ép thu nhỏ khối kềnh cộm `match` xử 1 cái nọc `Option<u8>` dông dài nạp ở dưới chóp thành vóc kề nhỏ bé chắt ép rút nước lột vỏ biến xài thế mạng của món đồ chiêu `if let`.
Code to convert:
```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
```

**Answer 2:**
en: 
```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```
vi: 
```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

---

## Level 4: Analyzing

**Question 1:**
en: Examine the binding capabilities of `match` arms. How does a `match` expression handle variable shadowing when an overarching variable overlaps with a newly bound variable extracted from inside an Enum payload?
vi: Vạch mắt soi ngó độ cắn móc dính kết (binding capabilities) ở chấu cài `match` arm. Xướng giải cái quy trình rẽ nhánh `match` uốn xử vụ "Bóng đè hắc ám cắn tên" (variable shadowing) gập góc ngay đỉnh chóp kẽ biến tổng đè bóng phủ trùm lên gáy tên cái biến bóc rã đẻ trong nhụy tách ở bầu payload mang vác của Enum?

**Answer 1:**
en: 
Within a `match` arm, initializing a pattern variable introduces a new inner scope bindings block constraint. For instance, if you have an outer variable `let y = 10;`, and a match arm like `Some(y) => println!("{}", y)`, the `y` inside the `Some(y)` pattern is a brand newly introduced variable tracking the extracted enum payload. It successfully shadows the outer `y` for the localized duration of the single executing match arm expression. Once the arm concludes, the scope vanishes safely, and the outer `10` is fully restored unscathed. Understanding this scope wall boundary avoids crippling logic defects where a programmer mistakenly believed they were modifying or checking against the outer variable's content rather than capturing brand-new bounds bindings.
vi: 
Sa chân tiến mốc tròng lọt cục địa phận nhánh mảng `match` arm, cú khai hỏa báo danh thắt chằng pattern biến (variable) đẻ ra 1 cửa biên địa hạt (inner scope) gá mốc constraint mới keng. Minh họa nhẹ, như hồi có đính 1 con biến trùm ôm ngoài `let y = 10;`, móc thắt 1 tay arm match kiểu tọt vô `Some(y) => println!("{}", y)`, cục `y` bọc trong khe nách `Some(y)` đó mộc khai là bản vĩ nhân sơ sanh biến mới tinh khôi lần giấu bám túm bắt chặt cõi rỗng payload trong ruột enum chiết lột móc ruột rỉ ra. Nó cắn nát phủ bóng rợp đè gọn thằng trùm `y` outer nhốt bên ngoài xuyên suốt dặm đường nháy hoạt cục bộ vạch lằn sọc logic nhánh match tay kia. Khép hòm sập mâm kết cung kết ấn, màn trời luồng scope đó bay màu bốc khói tan nát, vạc lại nhường ngai đôn hoàng trả đủ con `10` outer vẹn cốt láng keng như chửa có dâu bể. Thấm rành chân chẻ vách ngàm nách phạm vi này né xẻng chôn hố bug thúi thủng chết người nơi bấy thợ gieo code ấu trĩ đắc chí mộng tưởng nhầm thọt mẹ chọc vô nội khối ngầm ngoài biến gốc ngoài biên ải thay cho khâu đoạt mẻ cắn biến bound thâu tóm lột nhụy giấu tận bên trong.

**Question 2:**
en: Analyze the architectural significance of Match Guards (e.g., `Some(x) if x % 2 == 0 => ...`). Why did language designers incorporate `if` conditions inside pattern matches instead of forcing developers to use standard nested `if` blocks inside the match arm resolution body?
vi: Tra não hạch định lượng phẩm vị kiết cấu thượng đẳng đỉnh móng về món đòn "Cấm vệ Canh Gác Mảng" (Match Guards) (vd `Some(x) if x % 2 == 0 => ...`). Dâng lời sớ mổ xẻ vì cớ nào băng đảng kiến phác ngôn ngữ lại ép tròng kẹp chèn kẽ cục cọc luồng `if` thòng rễ thẳng cẳng trong lõi kẹp hàm pattern match thế, đè lật đổ bầy chiêu cọc gá lũ lượt nhét lót tụi nested hầm nhú thọt `if` cản bên trong sườn xử lý giải mã cục bộ body match arm rập khuôn bọc truyền thống?

**Answer 2:**
en: 
Match guards combine structural layout inspection (the pattern unpacking the Enum) tightly embedded with state-value conditional analysis. While injecting a nested `if` block deeper inside the execute body achieves logical execution mirroring parity, it breaks the core requirement of Rust: `match` Exhaustiveness Fall-through evaluation sequence mapping.
If you use a nested `if`, the arm has irrevocably "caught" the enum. If the nested inner `if` evaluates to false, you are mathematically trapped in that local block—you *cannot* seamlessly fall through and let the overarching remaining match arms continue evaluating other patterns.
Conversely, a Match Guard `Some(x) if x % 2 == 0` evaluates atomically *before* the arm commits to trapping the state. If the `if` guard evaluates false, the sequence immediately drops to evaluate subsequent match pattern arms down below. This vastly simplifies architectures dealing with hyper-complex state machines needing distinct pathways for ranges of numerical subsets bundled in the same structural Enum token.
vi: 
Hệ đòn thòng Match Guards vơ chằng ráp nối cú kiểm kê gá vạch dọn lòng structural (nội phân mảng Enum lộ vỏ pattern bóc) thắt cà vạt bó gút chặt với mạch định phân giá trị kiểm trích rẽ nhánh dạo điều kiện state-value. Nén một hố nested thụt lõm tọt vào `if` nhét tọt trong hàm cục bộ dù hoàn trọn nợ vạch phẩy nhịp execution cho y xì tạc, cớ lỡ mà dính dẫm cái điều kiện lõi máu tủy rập chết bầm mẻ chót của Rust: Nền Định đoạt Trượt Lưới vớt ngã sụp Fall-through rà chiếu ngàm vạc vét cạn nhánh Exhaustiveness biểu thức Match.
Giải uế nếu bấu nested thọt lõm vô gá `if` trong trỏng, cánh tay arm đã túm cổ xiết nòng "bắt dính" nhai sống enum k nôn ngược ói thoái. Nhược có đâm bồi vấp inner `if` té rớt ngã false sấp mặt, mày lỳ đòn ôm rủi chết dí chôn não tịt nút trong cái ngõ ngách block hang cục bộ đó—kỹ sư *Bất Lực* sẩy sườn tuột phanh xuyên thấu rơi trượt lùi hụt chân vớt lại nhường dường đua xí tẩy cho dàn arm chốt chòm kẽ lá bên mép dưới mồm chồm hóp thử nuốt các patterns chưa chèn khấc nhích theo.
Trái quay đầu bo ngàm, thế cờ Match Guard `Some(x) if x % 2 == 0` nhai tính sổ 1 lèo thần tốc nguyên cục Atomics *trước lằn chỉ* khoảnh mạch bóp nòng súng cái tay arm tung cam kết phong rào ấn gông cái tròng nhà giam đó. Lỡ nắn `if` guard ngã rớt khậc thành mẻ false sấp lủng móng, dây rập liên hoàn tụt trượt băng xuyên biên bốc khói tuồn dọt xê dịch tấp vô vồ sạp nhai dằn mặt soi nhào các patterns khớp tay match arm phần mé đít dưới còn chực lóng đợi hốt. Sải mưu hèn này mài vót hóa siêu tinh xảo khối khung sườn quản ngục trạng thái dạo máy State Machine phức siêu hình ngụy hạch bùng đa phân mẻ rẽ nẻo tót ngõ, lúc lâm rập nắn xé phân bầy luồng cho các ngách dãy range cọng số kẹt gom bọc gán cùng 1 chùm thẻ Enum token sườn dính chùm khói mịt.

---

## Level 5: Evaluating

**Question 1:**
en: Defend or critique the rigid Exhaustiveness compile-time requirements of the `match` expression when implementing scalable evolving architectures (like maintaining HTTP Error Codes or an expanding Video Game Item system). Does the necessity to manually maintain wildcard arms (`_ => ()`) or refactor massive structs represent superior strict engineering discipline or a crippling drag on agile developer momentum?
vi: Vỗ ngực bảo tiêu hay đấm chê nát mảng hạch đòi hẹp trật cứng nhắc "Vét Váng Nồi Sạch Sẽ Tận Cùng" (Exhaustiveness) ở cõi thần mộc Compile-time khi lôi cổ biểu thức `match` trổ trần ra ứng dụng hòng xây móng gạch đúc bệ phóng (Architectures) vươn mình phình to xoay hóa như quái thú (thí mồi ngạch dàn cọc Code Lỗi Mạng HTTP chi chít găm đầu hay khối bầy đồ Item rơi lề mót dạo dãn nở sình chưởng tợ Video Game khổng lồ). Sự tình phải dọn rác cọ tay bò ra nhai bám cắm cừ chốt mấu Wildcard tay tẻ (`_ => ()`) ngập bờ ruộng hay đụng đục vỡ băm vằm Refactor tháo cốt cùi Structs, tất nảy tấu lên bài ngợi ca dũng mãnh phẩm kỉ cương quy pháp Strict Engineering thần oai chấn chát hay phủ chóp bọc đầu nện 1 đòn tã nát kéo ì cỗ máy phản đòn kéo giật thóp chân bành chướng động vận Agile nhảy cóc phóng lẹ thần tốc bóp chết developer momentum mạch thở ngục?

**Answer 1:**
en: 
In the short-term prototype phase, strict exhaustiveness feels like a jarring constraint, arguably slowing initial momentum where developers simply want a "switch statement" that safely ignores unknown variables natively.
However, for infinitely evolving scalable architectures (like HTTP API gateways or massive game state engines), Rust’s compile-time exhaustiveness serves as an indispensable mechanized CI/CD architectural safeguard that vastly outperforms the "agility" of non-exhaustive languages (like C/TS/Java). 
When an architect adds a new `VideoGameItem::EpicSword` to a core enum deeply shared across 50 project crates, loosely-typed languages will cleanly compile. The new item drops, but the loot-rendering UI, stat-calculator, and inventory-saver silently execute their default "fall-through" logic, fundamentally ignoring the new item and corrupting the gameplay loop with invisible runtime bugs spread globally across the repository.
With Rust's exhaustive `match`, adding that one enum variant forcefully breaks the compiler locally exactly at the 50 distinct isolated lines of code where that core enum was matched. The compiler provides a checklist to the developer: "Update your UI, update your calculator, update your saver". The strictness converts a multi-week catastrophic runtime logic search spanning 50 repos into a deterministic compile-time checklist handled in an hour. Manual wildcard `_` fallbacks should be heavily cautioned against in core expanding domain logic precisely because they suppress this beautiful architectural alarm bell. It enforces a philosophy where true developer momentum is defined not by how fast you compile once, but by how fast you can reliably refactor massive complex abstractions without fear.
vi: 
Chạm ngưỡng phôi sơ thai nhú nguyên sơ mẫu rập mồi lẹ (prototype phase), móc xích Vét ráo chạn exhaustiveness rạo rực thít bọng cắn một vòng đai ức hiếp sượng sùng chói mắt óc, nện bàn chùi ngáng đường chẻ lùi vận đà khói phóng initial momentum ở nơi anh thợ code chỉ thèm khát nạp 1 cú gạt trượt "switch statement" bẽn lẽn bơ phớt lờ mấy hạch biến quăng mốc cưa xỏ dị k ra dáng.
Thế nhưng, tột chót ôm cua vòng cho hạ tầng Architecture cựa phình quái gở tiến hóa vô luân hồi (áp mâm như cọc hạch HTTP API rào gác hay bộ Cỗ Máy engine lùi lẫy ngọa tàng cuộn vặn khổng trò chơi), màn vạch mặt dòm kề dao rập Compile-time đanh vót vét nát đáy exhaustiveness chễm trệ dâng bệ như 1 ngục thủ thiên thần cơ giới bảo ngọc chèn chắn gá ván mạng CI/CD an toàn thượng tuyệt đong gánh bóp nát xương đập tan cái chữ "ngáo linh hoạt rẻ rúng lẹ" của cái đám ngôn ngữ lỉnh lỗ hổng k vét tận đáy (thâm thù lề C/TS/Java). 
Điển tích 1 thợ mổ nặn cọc nhồi bơm tọng chép thêm rèn cọc đồ `VideoGameItem::EpicSword` cấy vào cội rễ sừng sỏ Enum cấy sâu luồn rãnh phủ vây chia nọc ở 50 đầm lầy project crates lân bang, hội Ngôn Ngữ Nới lỏng (loosely-typed) biên dịch xoắn thoát lẹ như chùi tót tướt tuột lọt thông hành sạch bẽn. Lưỡi kiếm tân binh rớt hố lộ mặt rơi bị động, bồi cơ cục dàn render giao diện UI, vòng băm chẻ bộ Calculator điểm số, và cúc chốt hòm kho tải Inventory câm mồm ngoác đuôi câm nín lủi cuộn ngầm xẹt qua rập xào điệu "rớt mâm nhồi lại - fall-through" chối thẳng mặt ngó lơ tột bậc gốc kiếm kia quẩn hạch làm ung hoại mạch dòng xoáy lặp trò chơi Gameplay bằng bầy lố Bug ngụy tạo khốc tiễn tàng hình runtime gieo phân bón vất rác rải đầm bầy ngập repo mả nguồn xê dịch khét tanh.
Ôm sát chân giáo Rust vót chẻ `match` vét xàng, dũi nêm vẩy vọt mọc thêm rễ 1 chổ ngách enum mảng đó liền kích gầm cuộn vốc thít sập cản đánh lủng phổi vệt compiler chùi nhủi sấp mặt chôn điểm tại đứt khúc gá đúng ghim 50 lằn chớp tẻ vệt mốc độc cỏi mà tay mã lệnh chui luồn chực hờ nuốt enum lõi rập kẽ. Lão làng Compiler tự vác nón chòi chìa tọt tập danh bìa lệnh kê sát mặt gầm gừ nắn thợ code: "Kê gạch update UI đi cháu, cập bản nhịp Calculator mài tiếp, nâng phôi cục kho Saver đi nhóc". Sự cực đoan máu lạnh gò đúc tạc hình hóa 1 công trường lặn ngập hố bùn rà cùm siêu bão Logic Bug chết tiệt kéo neo mục tủy nhiều tuần giăng khắp 50 tổ chùm mảng vung chài sập ngào xuống vỏn vẹn lật đật list biên chế chém lụi mục định hình compile-time vét check-list làm nhẹ oai ráo hoảnh 1 tiếng chuông dọn bãi cạn ráo. Ngàm cài núp xéo hất ngụy chèn Wildcard mờ bịt mắt bừa `_ => ()` nên bị dấn cùm dập dọa nạt kịch chát tẩy chay cách ly xa lãnh hạch cội nguồn lõi domain mở ngọa sưng do vì đích xác rắp trúng phóc cái điệu nó tịt ngòi ỉm diệt trùm bịt loa tiếng chuông báo động thần thánh kiêu sa nức lòng người đúc cọc sườn kia nòi ruột lú rớt rã. Triết gia đó đúc cọc phàm tư duy phán xét chân quang Tốc độ Thần Lực lập trình Momentum nhà kĩ thuật phơi mặt cân thước chẳng phải móc độ đua cuồng luồng compile cho lướt tít xẹt qua ải cho chập mặt lúc thưở 1, mà chính là lột độ đua phi phàm rẽ lượn bạn tay càn quắp đục vỡ bung bét rũ xương tháo khớp dập refactor hàng nùi bộ nòng ảo abstraction đè vặn đại mạo chằng chịt ngập trời siêu to tột cực bự mà thênh thang vô ưu thần thái đéo sợ ngả hố sợ sập.
