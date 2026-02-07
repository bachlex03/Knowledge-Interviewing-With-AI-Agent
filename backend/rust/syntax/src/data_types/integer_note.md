# Integer Types in Rust (Notes)

- Rust infernce i32 by default

## OS related

It’s a classic "off-by-one" logic puzzle! The reason it stops at **255** is that we have a limited number of "patterns" available, and we use one of those patterns to represent **0**.

Think of it like a car's odometer with only three digits. If you can only show numbers from `000` to `999`, you can show **1,000** distinct values total—but the highest number you can see is **999**.

---

### The Math of Bits

Each bit is a switch that can be either **0** or **1**. With 8 bits, the total number of unique combinations is calculated as:


If we started counting at 1, we could reach 256. However, computers are designed to start counting at **0**.

### Let's visualize the count:

| Sequence | Binary | Decimal Value |
| --- | --- | --- |
| 1st combination | `00000000` | **0** |
| 2nd combination | `00000001` | **1** |
| ... | ... | ... |
| 255th combination | `11111110` | **254** |
| 256th combination | `11111111` | **255** |

If you tried to represent **256**, you would need a **9th bit**: `1 00000000`. Since a byte only has 8 "slots," that 9th bit has nowhere to go. This is what programmers call an **integer overflow**.

---

### The "Odometer" Visual

Imagine a physical counter:

1. You start at `00000000`.
2. You keep adding 1 until you hit `11111111` (which is 255).
3. If you add **one more**, the whole thing flips back to `00000000` because there is no room for the carry-over digit.

> **Fun Fact:** This is why original Pokémon games (like Red/Blue) often had items cap at 255, or why certain glitches happened at level 256—the memory literally couldn't hold a higher number in that single byte!

Would you like to see what happens in Rust code when you try to force a variable to go from 255 to 256?

---

## Handling Overflow with `Option`

When using methods like `checked_add`, Rust returns an `Option<T>`. You must handle the `None` case (overflow) to extract the value.

### 1. `unwrap()` / `expect()`
en: Use when you want to panic on overflow.
vi: Sử dụng khi bạn muốn chương trình dừng lại (panic) nếu bị tràn số.
```rust
let x = 255u8.checked_add(1).unwrap();
let x = 255u8.checked_add(1).expect("Overflow!");
```

### 2. `unwrap_or()`
en: Provide a default value on overflow.
vi: Cung cấp một giá trị mặc định nếu bị tràn số.
```rust
let x = 255u8.checked_add(1).unwrap_or(0);
```

### 3. `match` Expression
en: Explicitly handle both cases.
vi: Xử lý tường minh cả hai trường hợp.
```rust
let x = match 255u8.checked_add(1) {
    Some(val) => val,
    None => 0,
};
```

### 4. `if let`
en: Execute code only if no overflow occurs.
vi: Chỉ thực thi mã nếu không xảy ra tràn số.
```rust
if let Some(val) = 255u8.checked_add(1) {
    println!("Result: {}", val);
}
```