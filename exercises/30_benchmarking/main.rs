// Exercise 30: Benchmarking & Profiling
//
// Demonstrates: a hand-rolled `std::time::Instant`-based micro-benchmark
// harness. `cargo bench` itself works on stable Rust — it builds the
// `benches/` targets with optimizations and runs them. What is nightly-only
// is the built-in libtest `#[bench]` attribute (and `test::Bencher`), so on
// stable, bench targets set `harness = false` and use a library such as
// `criterion` (not a dependency here, in keeping with the zero-dependency
// rule). This exercise shows what criterion automates: warmup, repetition,
// averaging.
//
// RUN IT OPTIMIZED: `cargo run --release --bin 30_benchmarking`. A plain
// `cargo run` measures an unoptimized debug build, whose numbers say little
// about real performance (the program prints a warning in that case).

use std::time::{Duration, Instant};

// A naive O(2^n) recursive Fibonacci — deliberately slow, to have something
// worth measuring.
fn fib_recursive(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fib_recursive(n - 1) + fib_recursive(n - 2)
    }
}

// An O(n) iterative version — the comparison point.
fn fib_iterative(n: u64) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    a
}

// A minimal benchmark harness: run `f` `iterations` times after a warmup,
// return the mean duration per call. This is a simplified version of what
// criterion does with statistical rigor (outlier detection, confidence
// intervals) that this exercise skips for clarity.
fn bench(label: &str, iterations: u32, mut f: impl FnMut()) -> Duration {
    // Warmup — let caches, the branch predictor, and CPU frequency scaling
    // reach a steady state before timing. (There's no JIT to warm up, unlike
    // the JVM or JavaScript, but the hardware still has state.)
    for _ in 0..iterations / 10 {
        f();
    }

    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let elapsed = start.elapsed();
    let per_call = elapsed / iterations;
    println!("  {label}: {per_call:?}/call over {iterations} iterations (total {elapsed:?})");
    per_call
}

fn main() {
    println!("=== Exercise 30: Benchmarking & Profiling ===");
    if cfg!(debug_assertions) {
        println!("WARNING: this is a debug (unoptimized) build — timings are not representative.");
        println!("         Re-run with: cargo run --release --bin 30_benchmarking");
    }

    // Section 1: comparing two implementations of the same function
    println!("\n--- Section 1: fib_recursive vs fib_iterative ---");
    let recursive_time = bench("fib_recursive(25)", 100, || {
        std::hint::black_box(fib_recursive(std::hint::black_box(25)));
    });
    let iterative_time = bench("fib_iterative(25)", 100, || {
        std::hint::black_box(fib_iterative(std::hint::black_box(25)));
    });
    let speedup = recursive_time.as_secs_f64() / iterative_time.as_secs_f64().max(1e-12);
    println!("  iterative is ~{speedup:.0}x faster than recursive at n=25");

    // Section 2: why std::hint::black_box matters
    println!("\n--- Section 2: preventing the optimizer from cheating ---");
    println!("  Without black_box, an optimizing compiler can prove fib_iterative(25) is constant");
    println!(
        "  and either precompute it or delete the 'unused' result entirely — measuring nothing."
    );
    println!(
        "  black_box (stable since Rust 1.66) is an opaque-to-the-optimizer identity function."
    );

    // Section 3: comparing allocation strategies
    println!("\n--- Section 3: String building strategies ---");
    bench("push_str in a loop", 1000, || {
        let mut s = String::new();
        for _ in 0..50 {
            s.push_str("xy");
        }
        std::hint::black_box(s);
    });
    bench("+= in a loop", 1000, || {
        let mut s = String::new();
        for _ in 0..50 {
            s += "xy";
        }
        std::hint::black_box(s);
    });
    bench("String::with_capacity then push_str", 1000, || {
        let mut s = String::with_capacity(100);
        for _ in 0..50 {
            s.push_str("xy");
        }
        std::hint::black_box(s);
    });

    println!("\nNotes:");
    println!("  - `cargo bench` runs on stable; only libtest's #[bench] attribute is nightly — use `criterion`");
    println!("    (with `harness = false` in a [[bench]] target) in real projects.");
    println!("  - Always benchmark a --release build; debug builds skip the optimizations you're measuring.");
    println!("  - criterion adds statistical rigor this harness skips: outlier rejection, variance, HTML reports.");
    println!("  - std::hint::black_box stops the optimizer from deleting or constant-folding 'unused' benchmark work.");
    println!("  - For CPU profiling beyond timing (call graphs, flamegraphs): `cargo flamegraph` or `perf record` +");
    println!("    `perf report` on Linux, or Instruments.app on macOS — all external to std, unlike Go's built-in pprof.");
}
