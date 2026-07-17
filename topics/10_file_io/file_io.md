# File I/O — Cheat Sheet

## Whole-File Operations

```rust
use std::fs;

fs::write("out.txt", "hello\n")?;              // create/truncate + write, one call
let contents = fs::read_to_string("out.txt")?;  // read entire file as a UTF-8 String
let bytes = fs::read("out.txt")?;               // read entire file as Vec<u8>
```

For the common case ("just get me the whole file"), `std::fs` needs no manual buffer sizing
or loop — closer to Go's `os.ReadFile`/`os.WriteFile` than to C's `fopen`/`fread`/`fclose`
dance.

## Streaming Reads: `BufReader`

```rust
use std::io::{BufRead, BufReader};

let file = fs::File::open("big.txt")?;
let reader = BufReader::new(file);
for line in reader.lines() {
    let line = line?;   // each line is io::Result<String>
    println!("{line}");
}
```

`BufReader` wraps anything implementing `Read` and batches syscalls into internally
buffered reads — always prefer it over calling `.read()` directly in a loop, same reasoning
as Go's `bufio.Scanner`/`bufio.Reader`.

## Streaming Writes: `BufWriter`

```rust
use std::io::{BufWriter, Write};

let file = fs::File::create("out.txt")?;
let mut writer = BufWriter::new(file);
for i in 1..=3 {
    writeln!(writer, "line {i}")?;   // writeln!/write! work on anything implementing Write
}
writer.flush()?;                       // flushes on drop too, but this surfaces I/O errors explicitly
```

## `Read` and `Write` — the Composable Traits

```rust
fn copy_all(mut r: impl std::io::Read, mut w: impl std::io::Write) -> std::io::Result<u64> {
    std::io::copy(&mut r, &mut w)
}
```

Same design as Go's `io.Reader`/`io.Writer`: a small trait, implemented by files, network
sockets (topics/19), `Vec<u8>`, `Stdin`/`Stdout`, and anything else that moves bytes — code
written against the trait works with all of them, no matter the concrete source/sink.

## Opening With Specific Modes

```rust
use std::fs::OpenOptions;

let file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("log.txt")?;
```

## Error Handling

```rust
match fs::read_to_string("missing.txt") {
    Ok(s) => println!("{s}"),
    Err(e) => println!("error: {e} (kind: {:?})", e.kind()),  // e.g. ErrorKind::NotFound
}
```

`std::io::Error` carries a portable `ErrorKind` (`NotFound`, `PermissionDenied`,
`AlreadyExists`, ...) you can match on without parsing an OS-specific errno string — similar
intent to Go's `errors.Is(err, os.ErrNotExist)`.

## Metadata and Directory Listing

```rust
let meta = fs::metadata("out.txt")?;
println!("{} bytes, is_dir={}", meta.len(), meta.is_dir());

for entry in fs::read_dir(".")? {
    let entry = entry?;
    println!("{}", entry.path().display());
}
```

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Whole-file read | Manual `fopen`/`fread` loop + buffer sizing | `ifstream` + `stringstream`, or manual loop | `os.ReadFile` | `fs::read_to_string`/`fs::read` |
| Buffered I/O | Manual (or `setvbuf`) | `fstream` is buffered by default | `bufio.Reader`/`bufio.Scanner` | `BufReader`/`BufWriter` |
| Resource cleanup | Manual `fclose` (leaks if you forget) | RAII (`fstream` destructor) | `defer f.Close()` | Automatic — `File` closes on `Drop` |
| Error reporting | `errno` + `strerror` | Exceptions, or `std::error_code` | `error` with `os.IsNotExist` etc. | `io::Error` with a portable `ErrorKind` |
