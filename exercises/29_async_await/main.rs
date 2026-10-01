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
use std::sync::{Arc, Mutex};
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

// A hand-written LEAF future — the kind an `async fn` ultimately awaits.
// The compiler turns an `async fn` body into a state machine that polls
// futures like this one; it never writes a timer for you. Real runtimes
// provide leaf futures for timers and I/O (tokio::time::sleep, sockets);
// here we build a timer by hand.
//
// The Future contract: when returning Pending, arrange for the waker from
// the MOST RECENT poll to be woken. A future can be moved between tasks or
// executors, and each poll may carry a different waker, so storing only the
// first one would wake the wrong task. So the timer thread shares a slot
// with the future, and every poll refreshes the waker in that slot.
struct Delay {
    when: Instant,
    waker: Arc<Mutex<Option<Waker>>>,
    timer_started: bool,
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut(); // Delay is Unpin — no self-referential fields
        if Instant::now() >= this.when {
            return Poll::Ready(());
        }
        // Store (or replace) the current waker on EVERY poll, skipping the
        // clone when it would wake the same task anyway.
        {
            let mut slot = this.waker.lock().unwrap();
            match slot.as_ref() {
                Some(existing) if existing.will_wake(cx.waker()) => {}
                _ => *slot = Some(cx.waker().clone()),
            }
        }
        if !this.timer_started {
            // One helper thread per Delay keeps the example std-only; a real
            // runtime multiplexes every timer onto a single timer wheel/thread.
            let slot = Arc::clone(&this.waker);
            let when = this.when;
            thread::spawn(move || {
                let now = Instant::now();
                if when > now {
                    thread::sleep(when - now);
                }
                // Wake whichever waker the latest poll left in the slot.
                if let Some(waker) = slot.lock().unwrap().take() {
                    waker.wake(); // this is what turns the parked block_on thread back on
                }
            });
            this.timer_started = true;
        }
        Poll::Pending
    }
}

fn delay(duration: Duration) -> Delay {
    Delay {
        when: Instant::now() + duration,
        waker: Arc::new(Mutex::new(None)),
        timer_started: false,
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
