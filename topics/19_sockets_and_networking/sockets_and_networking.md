# Sockets & Networking — Cheat Sheet

## TCP Server

```rust
use std::net::TcpListener;

let listener = TcpListener::bind("127.0.0.1:0")?;   // ":0" asks the OS to pick a free port
let addr = listener.local_addr()?;                    // find out which port it chose

for stream in listener.incoming() {
    let stream = stream?;
    std::thread::spawn(move || handle_client(stream));  // one thread per connection
}
```

`TcpListener::bind` folds together what C does as three separate syscalls — `socket()`,
`bind()`, `listen()` — into one call, and `.incoming()` wraps `accept()` in an iterator.

## TCP Client

```rust
use std::net::TcpStream;

let mut stream = TcpStream::connect("127.0.0.1:8080")?;
stream.write_all(b"hello\n")?;
let mut response = String::new();
stream.read_to_string(&mut response)?;
```

`TcpStream` implements both `Read` and `Write` (topics/10), so every buffered-I/O technique
that works on a file works on a socket unchanged.

## Independent Read/Write Halves

```rust
let mut writer = stream.try_clone()?;   // an independent handle to the same underlying socket
let mut reader = std::io::BufReader::new(stream);
```

Needed when one thread/loop needs to write while another reads on the same connection — the
Rust analog of splitting a Go `net.Conn` isn't needed (Go lets one `Conn` be used from
multiple goroutines directly), but the *pattern* of a dedicated reader/writer pair per
connection is the same.

## Resource Cleanup — No Leaked File Descriptors

```rust
{
    let stream = TcpStream::connect(addr)?;
    // ... use it ...
}   // socket closes here automatically, when `stream` drops
```

Unlike raw BSD sockets in C (a leaked `close(fd)` call leaks a file descriptor until process
exit) or having to remember `conn.Close()` in Go, the socket's underlying file descriptor is
tied to ownership (topics/02) — it closes exactly once, deterministically, when the owning
value drops. No `defer`, no `finally`, needed for this specific concern.

## UDP (for Reference — Not Used in This Project's Exercises)

```rust
use std::net::UdpSocket;
let socket = UdpSocket::bind("127.0.0.1:0")?;
socket.send_to(b"hello", "127.0.0.1:9000")?;
let mut buf = [0u8; 1024];
let (n, src) = socket.recv_from(&mut buf)?;
```

## Error Handling

```rust
match TcpStream::connect("127.0.0.1:1") {
    Ok(_) => {}
    Err(e) => println!("connect failed: {e} (kind: {:?})", e.kind()),
}
```

Same `io::Error` + `ErrorKind` story as file I/O (topics/10) — `ConnectionRefused`,
`TimedOut`, and friends are portable across platforms without parsing errno strings.

## No Built-In HTTP

`std::net` stops at TCP/UDP — there's no `net/http` equivalent in the standard library.
Exercise 25 builds a line-oriented echo protocol directly on `TcpStream` to demonstrate
framing without one; a real HTTP server should use `axum`/`actix-web`/`hyper`, and a real
HTTP client should use `reqwest` (none used here, in keeping with the zero-dependency rule).

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Setup | `socket()`+`bind()`+`listen()`+`accept()`, four calls | Same as C, or an RAII wrapper class | `net.Listen()` + `.Accept()` | `TcpListener::bind()` + `.incoming()` |
| Cleanup | Manual `close(fd)` | RAII wrapper (if you wrote one) | `defer conn.Close()` | Automatic on `Drop` |
| Read/Write interface | Raw `read()`/`write()` on an fd | `<sys/socket.h>` calls, or an abstraction you build | `io.Reader`/`io.Writer` (Conn implements both) | `Read`/`Write` traits (topics/10) |
| Built-in HTTP | No | No | Yes — `net/http` in std | No — reach for `hyper`/`axum`/`reqwest` |
| Concurrency model per connection | Manual (thread/fork/select per conn) | Manual, or a library (Boost.Asio) | One goroutine per `Accept()`ed conn, idiomatically | One OS thread per connection, idiomatically (topics/07) |
