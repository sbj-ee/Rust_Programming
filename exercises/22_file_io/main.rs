// Exercise 22: File I/O
//
// Demonstrates: `std::fs` for whole-file reads/writes, `std::io::{Read,
// Write}` traits for streaming, and `BufReader`/`BufWriter` for buffered
// access — the Rust analogs of Go's `os`/`io`/`bufio` trio.

use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

// RAII cleanup: the temp file is removed when this guard drops — on the
// normal path AND when a `?` below returns early with an error, which a
// trailing `fs::remove_file` at the end of main would miss.
struct TempFile {
    path: PathBuf,
}

impl TempFile {
    fn new(name: &str) -> Self {
        // Include the process id so two concurrent runs never share (and
        // clobber) the same file.
        let path = std::env::temp_dir().join(format!("{name}-{}.txt", std::process::id()));
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // Drop can't return an error; ignoring NotFound (nothing was written)
        // is the right call, and any other failure is worth a note on stderr.
        if let Err(e) = fs::remove_file(&self.path) {
            if e.kind() != io::ErrorKind::NotFound {
                eprintln!("warning: could not remove {}: {e}", self.path.display());
            }
        }
    }
}

fn main() -> io::Result<()> {
    println!("=== Exercise 22: File I/O ===");

    let temp = TempFile::new("rust_programming_exercise22");
    let path = temp.path();

    // Section 1: whole-file write and read — simplest possible API
    println!("\n--- Section 1: fs::write / fs::read_to_string ---");
    fs::write(path, "line one\nline two\nline three\n")?; // ? propagates io::Error
    let contents = fs::read_to_string(path)?;
    print!("{contents}");

    // Section 2: streaming writes with a BufWriter — fewer syscalls than
    // writing each piece directly, same idea as Go's bufio.Writer
    println!("--- Section 2: BufWriter ---");
    {
        let file = fs::File::create(path)?;
        let mut writer = io::BufWriter::new(file);
        for i in 1..=3 {
            writeln!(writer, "buffered line {i}")?; // writeln! works on anything implementing Write
        }
        // writer flushes on drop, but explicit flush() makes errors visible
        writer.flush()?;
    }

    // Section 3: streaming reads with a BufReader, line by line
    println!("--- Section 3: BufReader, line by line ---");
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = line?; // each line is io::Result<String>
        println!("  [{i}] {line}");
    }

    // Section 4: appending, and checking metadata
    println!("\n--- Section 4: append and metadata ---");
    {
        let mut file = fs::OpenOptions::new().append(true).open(path)?;
        writeln!(file, "appended line")?;
    }
    let metadata = fs::metadata(path)?;
    println!("file size after append: {} bytes", metadata.len());

    // Section 5: error handling — a missing file yields io::Error, not a panic
    println!("\n--- Section 5: errors on a missing file ---");
    let missing = std::env::temp_dir().join(format!(
        "rust_programming_exercise22-missing-{}/nope.txt",
        std::process::id()
    ));
    match fs::read_to_string(&missing) {
        Ok(_) => println!("surprise: {} exists", missing.display()),
        Err(e) => println!("expected error: {e} (kind: {:?})", e.kind()),
    }

    // No explicit remove_file: `temp` drops at the end of main and deletes it.

    println!("\nNotes:");
    println!("  - fs::read_to_string/fs::write cover the whole-file case in one call — no manual buffer sizing.");
    println!("  - BufReader/BufWriter wrap any Read/Write to batch syscalls — always prefer them for line-by-line work.");
    println!("  - `main() -> io::Result<()>` lets `?` propagate straight out of main; a Result Err prints and exits non-zero.");
    println!("  - io::ErrorKind (e.kind()) gives a portable way to branch on NotFound/PermissionDenied/etc.");

    Ok(())
}
