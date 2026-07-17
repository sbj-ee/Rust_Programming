# Strings — Cheat Sheet

## `String` vs `&str`

```rust
let owned: String = String::from("hello");  // growable, heap-allocated, owns its buffer
let borrowed: &str = "hello";                 // a VIEW: (pointer, length) into UTF-8 bytes
let slice: &str = &owned[0..3];               // a &str borrowing INTO owned's buffer
```

This is exactly the `Vec<T>`/`&[T]` relationship from topics/11, applied to text: `String`
is the owning, growable buffer; `&str` is the borrowed, fixed view. A `&'static str` string
literal is baked directly into the binary.

## Everything Is UTF-8

```rust
let s = "héllo";           // valid UTF-8, non-ASCII bytes and all
// let bad = String::from_utf8(vec![0xFF]); // Err — not valid UTF-8, and it MUST be checked
```

A Rust `String`/`&str` is *guaranteed* valid UTF-8 — the type system enforces it. Building
one from raw bytes goes through `String::from_utf8`, which returns `Result` because the
bytes might not be valid UTF-8. C has no such guarantee at all (a `char*` is just bytes);
Go's `string` is also "usually UTF-8" but not enforced at the type level the way Rust's is.

## `char` vs `byte` — the Newcomer Trap

```rust
let s = "héllo";
s.len();                 // 6 — BYTE length (é is 2 bytes in UTF-8), not 5
s.chars().count();        // 5 — character count, walks the UTF-8 encoding
for c in s.chars() { }    // iterate Unicode scalar values (char, always 4 bytes)
for b in s.bytes()  { }   // iterate raw bytes (u8)
```

Indexing a `String` by integer (`s[2]`) **does not compile** — there is no `Index<usize>`
impl for `String`, specifically to stop you from silently landing mid-character. Slicing by
byte range (`&s[0..3]`) compiles but **panics at runtime** if the range doesn't fall on a
UTF-8 character boundary.

## Building Strings

```rust
let mut s = String::new();
s.push_str("hello");        // append a &str
s.push(' ');                  // append a single char
s.push_str("world");
let s2 = format!("{s}!");    // format! builds a new String, doesn't mutate
let s3 = s + "!";             // + consumes the left String, moves it, appends the right &str
```

`String::with_capacity(n)` pre-reserves a buffer, avoiding repeated reallocation — same idea
as Go's `strings.Builder` or C++'s `std::string::reserve`.

## Conversions

```rust
let n: i32 = "42".parse().unwrap();       // &str -> T via FromStr, returns Result
let s: String = 42.to_string();           // T -> String via Display
let s2: String = format!("{}", 42);        // equivalent, more general
let bytes: &[u8] = "hi".as_bytes();
let back: &str = std::str::from_utf8(bytes).unwrap();
```

## Common Operations

```rust
"  hi  ".trim();                          // "hi"
"a,b,,c".split(',').collect::<Vec<_>>();  // ["a", "b", "", "c"]
"hello".to_uppercase();                    // "HELLO" — allocates a new String
"hello".starts_with("he");                 // true
"hello".replace("l", "L");                 // "heLLo"
"hello world".split_whitespace().collect::<Vec<_>>();  // ["hello", "world"]
```

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Representation | `char*`, NUL-terminated | `std::string`, length-prefixed | `string`, (ptr, len), immutable | `String`/`&str`, (ptr, len[, cap]), UTF-8 enforced |
| Encoding guarantee | None — raw bytes | None — raw bytes (or `wchar_t`/`char16_t` variants) | "Usually UTF-8" by convention, not enforced | Enforced valid UTF-8 by the type system |
| Mutability | Mutable buffer, manual sizing | `std::string` mutable, manages its own buffer | Immutable; `strings.Builder` for building | `String` mutable and growable; `&str` immutable view |
| Indexing by position | Byte, always | Byte, always | Byte, always (`s[i]` is a byte, not a rune) | Not allowed by integer index; use `.chars()`/`.bytes()` |
| Ownership | Manual | RAII | GC | Borrow-checked, `String` (own) vs `&str` (borrow) |
