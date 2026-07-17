// Exercise 22: File I/O
//
// Demonstrates: `std::fs` for whole-file reads/writes, `std::io::{Read,
// Write}` traits for streaming, and `BufReader`/`BufWriter` for buffered
// access — the Rust analogs of Go's `os`/`io`/`bufio` trio.

use std::fs;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    println!("=== Exercise 22: File I/O ===");

    let path = std::env::temp_dir().join("rust_programming_exercise22.txt");

    // Section 1: whole-file write and read — simplest possible API
    println!("\n--- Section 1: fs::write / fs::read_to_string ---");
    fs::write(&path, "line one\nline two\nline three\n")?; // ? propagates io::Error
    let contents = fs::read_to_string(&path)?;
    print!("{contents}");

    // Section 2: streaming writes with a BufWriter — fewer syscalls than
    // writing each piece directly, same idea as Go's bufio.Writer
    println!("--- Section 2: BufWriter ---");
    {
        let file = fs::File::create(&path)?;
        let mut writer = io::BufWriter::new(file);
        for i in 1..=3 {
            writeln!(writer, "buffered line {i}")?; // writeln! works on anything implementing Write
        }
        // writer flushes on drop, but explicit flush() makes errors visible
        writer.flush()?;
    }

    // Section 3: streaming reads with a BufReader, line by line
    println!("--- Section 3: BufReader, line by line ---");
    let file = fs::File::open(&path)?;
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = line?; // each line is io::Result<String>
        println!("  [{i}] {line}");
    }

    // Section 4: appending, and checking metadata
    println!("\n--- Section 4: append and metadata ---");
    {
        let mut file = fs::OpenOptions::new().append(true).open(&path)?;
        writeln!(file, "appended line")?;
    }
    let metadata = fs::metadata(&path)?;
    println!("file size after append: {} bytes", metadata.len());

    // Section 5: error handling — a missing file yields io::Error, not a panic
    println!("\n--- Section 5: errors on a missing file ---");
    match fs::read_to_string("/does/not/exist/anywhere") {
        Ok(_) => unreachable!(),
        Err(e) => println!("expected error: {e} (kind: {:?})", e.kind()),
    }

    fs::remove_file(&path)?; // clean up the temp file

    println!("\nNotes:");
    println!("  - fs::read_to_string/fs::write cover the whole-file case in one call — no manual buffer sizing.");
    println!("  - BufReader/BufWriter wrap any Read/Write to batch syscalls — always prefer them for line-by-line work.");
    println!("  - `main() -> io::Result<()>` lets `?` propagate straight out of main; a Result Err prints and exits non-zero.");
    println!("  - io::ErrorKind (e.kind()) gives a portable way to branch on NotFound/PermissionDenied/etc.");

    Ok(())
}
