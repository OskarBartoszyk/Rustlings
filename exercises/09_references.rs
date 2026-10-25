// ╔══════════════════════════════════════════════════════════╗
// ║  09 - REFERENCES AND BORROWING                           ║
// ╚══════════════════════════════════════════════════════════╝
//
// Giving away ownership for every little thing is exhausting. 😩
// Solution: LEND the value instead. That's a reference, written with `&`.
//
//     fn length(s: &String) -> usize { s.len() }
//
//     let s = String::from("hello");
//     let n = length(&s);        // "here, take a look, but give it back!"
//     println!("{s} {n}");       // ✅ still ours
//
// * `&T` is a SHARED, read-only reference (like a library book you can read
//   but not scribble in). Many people can read it at the same time.
// * The compiler guarantees a reference never outlives the thing it points
//   to. No dangling pointers, ever.
// * `*r` follows the reference to the value inside (the "dereference").
//   For method calls and printing Rust does this automatically.

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn demo() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s; // any number of readers is fine
    println!("{r1} {r2} len={}", calculate_length(&s));

    let x = 10;
    let rx = &x;
    println!("{}", *rx + 1); // math needs the explicit `*`
}

// 🎯 MISSION 1
// Ferris's mailbox: is the borrowed message empty? Return true if so.
// Hint: Strings have a method `.is_empty()`.
fn is_blank(s: &String) -> bool {
    todo!() // <-- replace this with your code
}

fn main() {
    demo();
    let s = String::from("hello");

    // 🎯 MISSION 2
    // Oh no, someone passed `s` BY VALUE to calculate_length in an earlier
    // version, and the String vanished (see the assert below: we still need it!).
    // Store the length in `n`, but LEND `s` with `&s` instead of giving it away.
    todo!() // <-- replace this with your code
    assert_eq!(n, 5);
    assert_eq!(s, "hello"); // still ours, because we only lent it
    assert!(is_blank(&String::new()));
    println!("✅ Exercise 09 passed! Borrowed and returned on time. 📚");
}
