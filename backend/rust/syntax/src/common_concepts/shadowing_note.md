# Shadowing notes

## Shadowing vs Mutability (`mut` keyword)

The short answer is: **Shadowing usually doesn't move or change the data in memory; it just changes how the compiler lets you talk to it.**

In Rust, "shadowing" is when you declare a new variable with the same name as a previous one. It’s a bit of a magic trick—the old value still exists (usually), but you’ve blocked your own path to it.

---

## How It Works Under the Hood

When you shadow a variable, you aren't "overwriting" the old memory slot. Instead, you are telling the compiler, "From now on, when I say `x`, I mean this *new* thing, not the old one."

### 1. Same Type Shadowing

If you shadow a variable with the same type, the compiler might reuse the stack space if it's efficient, but conceptually, they are two different "bindings."

```rust
let x = 5; 
let x = x + 1; // Shadowing

```

### 2. Different Type Shadowing (The common use case)

This is where shadowing shines. You can change the type of a variable name without having to come up with names like `spaces_str` and `spaces_int`.

```rust
let spaces = "   ";       // A string slice
let spaces = spaces.len(); // Now an integer

```

In this case, the string slice and the integer exist in **different locations** on the stack. The first `spaces` is effectively "hidden" or made inaccessible for the rest of its scope.

---

## Memory and Scope

It is important to understand that shadowing **does not drop** the first variable immediately. Both exist in memory until they go out of scope.

| Feature | Shadowing | Mutability (`mut`) |
| --- | --- | --- |
| **Memory** | Creates a **new** binding/location. | Reuses the **same** memory location. |
| **Type** | Can change the data type. | Must keep the same data type. |
| **Access** | Old value becomes inaccessible. | Old value is overwritten/changed. |

### The "Scope" Catch

If you shadow a variable inside a nested block, the original variable "comes back to life" once that block ends.

```rust
let x = 10;
{
    let x = 20; // Shadows the outer x
    println!("{x}"); // Prints 20
}
println!("{x}"); // Prints 10! The original memory was never touched.

```

---

## Summary

Shadowing is a **compile-time** name management tool. It doesn't perform a "write" operation to the old memory; it simply allocates a new spot on the stack and binds the name to it, leaving the old data sitting there until the scope ends.

Would you like to see how shadowing interacts with **Ownership and Moves**, or are you more interested in how it compares to using `mut`?

 