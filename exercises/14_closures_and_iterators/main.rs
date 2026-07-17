// Exercise 14: Closures & Iterators
//
// Demonstrates: the `Fn`/`FnMut`/`FnOnce` traits that classify closures by
// capture mode, and the iterator adapter chain (`map`/`filter`/`fold`/...)
// that replaces most hand-written loops — lazy, zero-cost, and composable.

// Accepts anything callable that borrows its captures immutably (or nothing).
fn call_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

// Accepts a closure that needs to mutate its captured state between calls.
fn call_n_times(mut f: impl FnMut() -> i32, n: usize) -> Vec<i32> {
    (0..n).map(|_| f()).collect()
}

// Accepts a closure that consumes something it captured — can only run once.
fn call_once(f: impl FnOnce() -> String) -> String {
    f()
}

fn main() {
    println!("=== Exercise 14: Closures & Iterators ===");

    // Section 1: Fn — captures by reference, callable repeatedly
    println!("\n--- Section 1: Fn ---");
    let double = |x: i32| x * 2;
    println!("call_twice(double, 3) = {}", call_twice(double, 3));

    // Section 2: FnMut — captures by mutable reference
    println!("\n--- Section 2: FnMut ---");
    let mut counter = 0;
    let next = || {
        counter += 1;
        counter
    };
    println!("call_n_times(next, 4) = {:?}", call_n_times(next, 4));

    // Section 3: FnOnce — captures by value, consumes the capture, one call only
    println!("\n--- Section 3: FnOnce ---");
    let owned = String::from("consumed");
    let take = move || owned; // `move` forces capture by value, not reference
    println!("call_once = {:?}", call_once(take));

    // Section 4: iterator adapters are LAZY — nothing runs until a consumer pulls
    println!("\n--- Section 4: laziness ---");
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
    let iter = numbers.iter().map(|n| {
        print!("(mapping {n}) ");
        n * n
    });
    println!("\n(iterator built, nothing printed yet above except this line)");
    let squares: Vec<i32> = iter.collect(); // NOW the closure actually runs, once per element
    println!("\nsquares={squares:?}");

    // Section 5: chaining filter/map/fold — no intermediate allocations between steps
    println!("\n--- Section 5: adapter chains ---");
    #[allow(clippy::unnecessary_fold)]
    // .sum() would be idiomatic here; fold is shown deliberately as the general form
    let sum_of_even_squares: i32 = numbers
        .iter()
        .filter(|&&n| n % 2 == 0) // keep evens
        .map(|&n| n * n) // square them
        .fold(0, |acc, n| acc + n); // reduce to a single total — .sum() is the specialized shortcut
    println!("sum_of_even_squares={sum_of_even_squares}");

    // Section 6: other common adapters
    println!("\n--- Section 6: more adapters ---");
    println!("any even? {}", numbers.iter().any(|&n| n % 2 == 0));
    println!("all positive? {}", numbers.iter().all(|&n| n > 0));
    println!("first > 5: {:?}", numbers.iter().find(|&&n| n > 5));
    println!("take(3): {:?}", numbers.iter().take(3).collect::<Vec<_>>());
    println!(
        "zip with letters: {:?}",
        numbers.iter().zip(['a', 'b', 'c']).collect::<Vec<_>>()
    );

    println!("\nNotes:");
    println!("  - Fn: borrows captures (&); FnMut: mutably borrows (&mut); FnOnce: takes ownership, one call.");
    println!("  - `move` forces a closure to capture by value even if a reference would compile.");
    println!("  - Iterator adapters build a lazy pipeline; nothing executes until collect/fold/for/sum/etc. drives it.");
    println!("  - The chain compiles down to a loop as tight as hand-written code — this costs nothing at runtime.");
}
