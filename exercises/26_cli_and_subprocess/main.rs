// Exercise 26: Command-Line Args & Subprocesses
//
// Demonstrates: `std::env::args`, `std::env::var`, and
// `std::process::Command` — the Rust analog of Go's `os.Args`/`os.Getenv`
// and `os/exec`, and C's `argv`/`getenv`/`fork`+`exec`+`waitpid`.

use std::env;
use std::process::Command;

fn main() {
    println!("=== Exercise 26: Command-Line Args & Subprocesses ===");

    // Section 1: command-line arguments
    println!("\n--- Section 1: env::args ---");
    let args: Vec<String> = env::args().collect();
    println!("argv[0] (program path): {}", args[0]);
    println!("remaining args: {:?}", &args[1..]);
    println!("(run with `cargo run --bin 26_cli_and_subprocess -- foo bar` to pass args through)");

    // Section 2: environment variables
    println!("\n--- Section 2: env::var ---");
    match env::var("HOME") {
        Ok(home) => println!("HOME={home}"),
        Err(e) => println!("HOME not set: {e}"),
    }
    println!("PATH is set: {}", env::var("PATH").is_ok());

    // Section 3: running a subprocess and capturing output — the Rust
    // analog of fork()+exec()+waitpid() rolled into one call
    println!("\n--- Section 3: Command::output ---");
    let output = Command::new("echo")
        .arg("hello from a child process")
        .output()
        .expect("failed to run echo — is it on PATH?");
    println!("status: {}", output.status);
    println!(
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout).trim_end()
    );

    // Section 4: piping stdin to a child and reading its stdout
    println!("\n--- Section 4: piping stdin/stdout ---");
    use std::io::Write;
    use std::process::Stdio;
    let mut child = Command::new("wc")
        .arg("-l")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn wc — is it on PATH?");
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"line one\nline two\nline three\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    println!(
        "wc -l counted: {} lines",
        String::from_utf8_lossy(&output.stdout).trim()
    );

    // Section 5: checking exit status without capturing output
    println!("\n--- Section 5: exit status ---");
    let status = Command::new("true").status().expect("failed to run true");
    println!("`true` succeeded: {}", status.success());
    let status = Command::new("false").status().expect("failed to run false");
    println!(
        "`false` succeeded: {} (code: {:?})",
        status.success(),
        status.code()
    );

    println!("\nNotes:");
    println!("  - env::args() yields the program path as element 0, same convention as C's argv/Go's os.Args.");
    println!(
        "  - Command::output() is the one-shot 'run, wait, capture stdout+stderr+status' call."
    );
    println!("  - Command::spawn() + Stdio::piped() gives streaming control, needed to feed a child's stdin.");
    println!("  - No manual fork()/exec()/waitpid(); the child process is reaped when the Child value drops or waits.");
}
