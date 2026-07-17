# Types — Cheat Sheet

## Scalar Types

```rust
let a: i8/i16/i32/i64/i128 = -1;   // signed, explicit width; i32 is the default
let b: u8/u16/u32/u64/u128 = 1;    // unsigned, explicit width
let c: isize/usize = 1;             // pointer-width — used for indexing/lengths
let d: f32/f64 = 1.0;               // f64 is the default float
let e: bool = true;
let f: char = 'R';                  // ALWAYS 4 bytes, a Unicode scalar value, not a byte
```

No implicit narrowing/widening, ever. Every conversion is an explicit `as`:

```rust
let big: i64 = 300;
let truncated = big as u8;   // 300 % 256 = 44 — silent wraparound, same risk as a C cast
let f = 3.9_f64 as i32;      // 4? No — truncates toward zero: 3
```

## Overflow Behavior

```rust
let x: u8 = 255;
// x + 1            // debug build: PANICS ("attempt to add with overflow")
x.wrapping_add(1)    // 0 — explicit wraparound
x.checked_add(1)     // None — explicit Option-returning check
x.saturating_add(1)  // 255 — clamps at the type's max
```

Release builds (`--release`) silently wrap by default, same as C — but debug builds panic,
catching overflow bugs long before they'd ship. This is a deliberate middle ground between
C (always silent) and a checked-arithmetic-everywhere language (always slow).

## Compound Types

```rust
let tuple: (i32, f64, &str) = (1, 2.0, "three");
tuple.0; tuple.1; tuple.2;                 // access by position
let (a, b, c) = tuple;                      // destructure

let arr: [i32; 4] = [1, 2, 3, 4];           // fixed size, part of the TYPE
let zeros = [0; 5];                         // [0, 0, 0, 0, 0]
```

`[i32; 4]` and `[i32; 5]` are different types — array length is compile-time, not a runtime
field the way it effectively is for a C array-decayed-to-pointer.

## `const` vs `static`

```rust
const MAX: u32 = 100;         // inlined at every use site; no fixed address
static NAME: &str = "rustd";  // one fixed memory location for the whole program
static mut COUNTER: i32 = 0;  // mutation requires `unsafe` — see topics/16_unsafe_rust
```

## Type Inference

```rust
let x = 5;          // inferred i32 (the default integer type)
let y = 5.0;         // inferred f64 (the default float type)
let v = Vec::new();  // needs a hint — often from later usage or an annotation
let v: Vec<i32> = Vec::new();
```

Inference is local — Rust never infers a function's parameter or return types from its body
or call sites, unlike closures (topics/15) or some ML-family languages.

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Default int width | `int` (usually 32-bit, platform-defined) | same as C | `int` (32 or 64-bit, platform-defined) | `i32`, always exactly 32 bits |
| Implicit conversions | Yes (dangerous) | Yes (dangerous, `-Wconversion` helps) | No | Never — always explicit `as` |
| Overflow | UB (signed) / wraps (unsigned) | same as C | wraps silently | panics in debug, wraps in release |
| `char` | 1 byte | 1 byte (`char8_t`/`wchar_t` for Unicode) | `rune` = `int32`, separate from `byte` | 4 bytes, always a Unicode scalar value |
| Array length | not part of the type after decay | `std::array<T,N>` encodes it | not part of the type (slices instead) | part of the type: `[T; N]` |
