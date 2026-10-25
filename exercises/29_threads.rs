// ╔══════════════════════════════════════════════════════════╗
// ║  29 - THREADS                                            ║
// ╚══════════════════════════════════════════════════════════╝
//
// Python's GIL means threads take turns. Rust threads are REAL OS threads that
// run in parallel, and the ownership rules turn data races (the scariest bug
// in concurrency) into COMPILE ERRORS. "Fearless concurrency"! 💪
//
//     let handle = thread::spawn(move || { 1 + 2 });
//     let result = handle.join().unwrap();    // wait for it, get its return value
//
// * The thread runs the closure you give it. Use `move` so the closure takes
//   OWNERSHIP of the variables it uses (the thread may outlive the function).
// * `join()` waits for the thread and returns a Result (Err if it panicked).
//
// CHANNELS: threads can talk by sending messages. 📬
//     let (tx, rx) = mpsc::channel();         // transmitter, receiver
//     thread::spawn(move || tx.send(42).unwrap());
//     rx.recv().unwrap();                     // waits for a message
//   Clone `tx` to have several senders.

use std::sync::mpsc;
use std::thread;

fn demo() {
    let h = thread::spawn(|| 1 + 2);
    println!("thread returned {}", h.join().unwrap());

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("hello from a thread").unwrap();
    });
    println!("{}", rx.recv().unwrap());
}

fn main() {
    demo();

    // 🎯 MISSION
    // Ferris hired 4 helper crabs, one per number i in 1..=4. 🦀🦀🦀🦀
    // Each crab computes i * i in its OWN thread. Oh no, nobody started them!
    // Inside the loop, spawn a thread with a `move` closure that returns
    // `i * i`, and push the returned JoinHandle into `handles`:
    //
    //     handles.push(thread::spawn(move || i * i));
    //
    // After the loop we `join` all of them and add up the answers: 1+4+9+16 = 30.
    let mut handles: Vec<thread::JoinHandle<i32>> = Vec::new();
    for i in 1..=4 {
        todo!() // <-- replace this with your code
    }
    let total: i32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    assert_eq!(total, 30);
    println!("✅ Exercise 29 passed! Four crabs worked in parallel. 🧵");
}
