// Exercise 29: Async/Await
//
// Demonstrates: `async fn`/`.await`, and — because `std` deliberately ships
// no executor — a ~30-line hand-rolled `block_on` that shows what a real
// runtime (tokio, async-std) does under the hood: poll a `Future`, and
// when it returns `Poll::Pending`, sleep the thread until a `Waker` says
// to try again. This is the one place the zero-dependency rule costs real
// convenience — real projects should use tokio rather than write this.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::{Duration, Instant};

// A Waker needs to know how to wake the executor back up. Here "the
// executor" is just whichever OS thread called block_on, parked and
// waiting — so waking means unpark()ing it.
struct ThreadWaker {
    thread: thread::Thread,
}

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.thread.unpark();
    }
}

// The entire executor: poll the future; if Pending, go to sleep until
// woken; if Ready, return the value. Real executors (tokio) additionally
// juggle many futures at once via an event loop — this one drives exactly one.
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = Box::pin(future);
    let waker = Waker::from(Arc::new(ThreadWaker {
        thread: thread::current(),
    }));
    let mut cx = Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value,
            Poll::Pending => thread::park(), // sleep until ThreadWaker::wake() unparks us
        }
    }
}

// A hand-written Future — this is what the compiler generates automatically
// for an `async fn` body, made explicit here. On first poll, it spawns a
// helper thread to sleep and then call the waker; on later polls, it
// checks whether the deadline has passed.
struct Delay {
    when: Instant,
    waker_registered: bool,
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut(); // Delay is Unpin — no self-referential fields
        if Instant::now() >= this.when {
            return Poll::Ready(());
        }
        if !this.waker_registered {
            let waker = cx.waker().clone();
            let when = this.when;
            thread::spawn(move || {
                let now = Instant::now();
                if when > now {
                    thread::sleep(when - now);
                }
                waker.wake(); // this is what turns the parked block_on thread back on
            });
            this.waker_registered = true;
        }
        Poll::Pending
    }
}

fn delay(duration: Duration) -> Delay {
    Delay {
        when: Instant::now() + duration,
        waker_registered: false,
    }
}

// A plain async fn — no real .await inside, just an expression. Compiles to
// a state machine with one state, immediately Ready on first poll.
async fn compute() -> i32 {
    let a = 5;
    let b = 10;
    a + b
}

// .await suspends this function's state machine at each point until the
// awaited future resolves — sequential here, since std has no join!/select!
// without pulling in the `futures` crate (intentionally not a dependency).
async fn delayed_greeting(name: &str) -> String {
    delay(Duration::from_millis(15)).await;
    format!("Hello, {name}, after a delay")
}

async fn two_sequential_delays() -> &'static str {
    delay(Duration::from_millis(5)).await;
    delay(Duration::from_millis(5)).await;
    "both delays complete, one after another"
}

fn main() {
    println!("=== Exercise 29: Async/Await ===");

    // Section 1: an async fn with no actual suspension
    println!("\n--- Section 1: async fn, no awaits ---");
    println!("compute() = {}", block_on(compute()));

    // Section 2: awaiting a custom Future that suspends and resumes
    println!("\n--- Section 2: awaiting a custom Future ---");
    println!("{}", block_on(delayed_greeting("Ferris")));

    // Section 3: sequential awaits within one async fn
    println!("\n--- Section 3: sequential awaits ---");
    println!("{}", block_on(two_sequential_delays()));

    println!("\nNotes:");
    println!("  - `async fn` compiles to a state machine implementing Future — .await is a suspend point.");
    println!("  - std ships Future/Context/Poll/Waker but deliberately NO executor — you must supply one.");
    println!(
        "  - Poll::Pending means 'not ready; I'll call your Waker when you should poll me again'."
    );
    println!("  - A real project should use tokio: multi-threaded reactor, real async I/O, join!/select!, timers.");
}
