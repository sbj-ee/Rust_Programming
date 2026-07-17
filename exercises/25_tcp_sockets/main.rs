// Exercise 25: TCP Sockets
//
// Demonstrates: `std::net::{TcpListener, TcpStream}` — a concurrent echo
// server and a client talking to it over loopback, entirely in one
// process. This is the Rust analog of Go's `net.Listen`/`net.Dial` and
// C's raw BSD sockets, minus the manual `bind`/`listen`/`accept` dance and
// the risk of leaking a file descriptor (the socket closes on `Drop`).

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: TcpStream) {
    let peer = stream.peer_addr().unwrap();
    let reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream;
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line == "quit" {
            break;
        }
        let reply = format!("echo: {line}\n");
        if writer.write_all(reply.as_bytes()).is_err() {
            break;
        }
    }
    println!("  [server] connection from {peer} closed");
}

fn main() -> std::io::Result<()> {
    println!("=== Exercise 25: TCP Sockets ===");

    // Section 1: bind a listener on an OS-assigned port (":0")
    println!("\n--- Section 1: starting the server ---");
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;
    println!("listening on {addr}");

    // Section 2: accept connections on a background thread, one thread per client
    println!("\n--- Section 2: accept loop (background thread) ---");
    let server = thread::spawn(move || {
        // accept exactly 2 connections for this exercise, then stop
        for stream in listener.incoming().take(2) {
            match stream {
                Ok(s) => {
                    thread::spawn(move || handle_client(s));
                }
                Err(e) => eprintln!("accept error: {e}"),
            }
        }
    });

    // Section 3: a client connecting and exchanging lines
    println!("\n--- Section 3: client one ---");
    {
        let stream = TcpStream::connect(addr)?;
        let mut writer = stream.try_clone()?;
        let mut reader = BufReader::new(stream);
        for msg in ["hello", "world"] {
            writer.write_all(format!("{msg}\n").as_bytes())?;
            let mut reply = String::new();
            reader.read_line(&mut reply)?;
            print!("  [client1] server said: {reply}");
        }
        writer.write_all(b"quit\n")?;
    }

    // Section 4: a second, independent client
    println!("\n--- Section 4: client two ---");
    {
        let stream = TcpStream::connect(addr)?;
        let mut writer = stream.try_clone()?;
        let mut reader = BufReader::new(stream);
        writer.write_all(b"second connection\n")?;
        let mut reply = String::new();
        reader.read_line(&mut reply)?;
        print!("  [client2] server said: {reply}");
        writer.write_all(b"quit\n")?;
    }

    server.join().unwrap();

    println!("\nNotes:");
    println!("  - TcpListener::bind + .incoming() replaces bind()/listen()/accept() from raw BSD sockets.");
    println!("  - The socket closes automatically when the TcpStream/TcpListener is dropped — no leaked fds.");
    println!("  - try_clone() gives an independent handle to the same socket for simultaneous read+write halves.");
    println!("  - `:0` asks the OS to pick a free port — local_addr() reports which one it chose.");

    Ok(())
}
