// ╔══════════════════════════════════════════════════════════╗
// ║  30 - Arc AND Mutex (sharing between threads)            ║
// ╚══════════════════════════════════════════════════════════╝
//
// How do several threads update ONE shared counter? You need two tools:
//
// * `Arc<T>`   = like Rc, but safe across threads (Atomic Reference Count).
//                `Arc::clone(&a)` gives each thread its own handle.
// * `Mutex<T>` = a lock around the data 🔐. Only one thread at a time:
//                    let mut guard = m.lock().unwrap();
//                    *guard += 1;
//                The lock is released automatically when `guard` goes out of scope.
//
// `Arc<Mutex<T>>` is THE pattern. Try using Rc inside a thread and the compiler
// says "`Rc` cannot be sent between threads safely": Rust's `Send` and `Sync`
// traits track thread-safety at compile time. Data races are impossible. 🎉

use std::sync::{Arc, Mutex};
use std::thread;

fn demo() {
    let m = Mutex::new(5);
    {
        let mut guard = m.lock().unwrap();
        *guard += 1;
    } // lock released here
    println!("{:?}", m.lock().unwrap());
}

fn main() {
    demo();

    // The cafe has ONE shared visitor counter 🔢 and 10 customers walk in at
    // the same time, each on their own thread!
    let counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    for _ in 0..10 {
        let c = Arc::clone(&counter); // this thread's own handle
        // 🎯 MISSION
        // Oh no! The customers walk in, but nobody clicks the counter. Spawn a
        // thread with a `move` closure which locks `c` and adds 1:
        //
        //     thread::spawn(move || { *c.lock().unwrap() += 1; })
        //
        // and push the JoinHandle into `handles`.
        // [[
        handles.push(thread::spawn(move || {
            *c.lock().unwrap() += 1;
        }));
        // ]]
    }
    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(*counter.lock().unwrap(), 10);
    println!("✅ Exercise 30 passed! Exactly 10 customers, no race conditions. 🚪");
}
