// ╔══════════════════════════════════════════════════════════╗
// ║  07 - OWNERSHIP  (the big one!)                          ║
// ╚══════════════════════════════════════════════════════════╝
//
// Python has a garbage collector: a janitor 🧹 who walks around and cleans up
// memory nobody uses any more. Rust has NO janitor. Instead it has 3 simple rules:
//
//   1. Every value has exactly ONE owner (a variable).
//   2. When the owner goes out of scope (`}`), the value is thrown away.
//   3. Assigning a value or passing it to a function MOVES it to the new owner.
//
// Think of it like a physical object. If Ferris hands you his sandwich 🥪,
// Ferris no longer has a sandwich.
//
//     let s1 = String::from("hi");
//     let s2 = s1;           // the String moves into s2
//     println!("{s1}");      // ❌ "borrow of moved value: s1"  (s1 has no sandwich!)
//
// Same thing when passing it to a function: the function now owns it, and
// it's dropped at the end of that function unless you hand it back (return it).
//
// Why is this awesome? Memory is freed at an exact, predictable moment,
// with ZERO garbage-collector pauses, and the compiler proves you never
// use freed memory.
//
// (Small simple types like numbers are just copied instead - next lesson.)

fn takes_ownership(s: String) {
    println!("I own '{s}' now");
} // `s` is dropped here. Bye, string!

fn gives_back(s: String) -> String {
    s // handing the sandwich back to the caller
}

fn demo() {
    let s1 = String::from("hello");
    let s2 = s1; // moved!
    println!("{s2}");

    takes_ownership(s2);
    // println!("{s2}"); // ❌ would not compile, the function ate it

    let s3 = String::from("again");
    let s3 = gives_back(s3); // went in, came back out
    println!("{s3}");
}

// 🎯 MISSION
// Ferris owns a String and passes it to `append_world`. The function now owns
// it (and it's declared `mut`, so it may change it). The text should become
// "hello" + " world".
//   1. Add " world" to the end with `s.push_str(" world");`
//   2. Hand the String back by making `s` the last line (no semicolon!).
// Otherwise the string would be dropped at the end of the function. 💀
fn append_world(mut s: String) -> String {
    // [[
    s.push_str(" world");
    s
    // ]]
}

fn main() {
    demo();
    let greeting = String::from("hello");
    let result = append_world(greeting);
    // `greeting` was moved away; only `result` is usable now.
    assert_eq!(result, "hello world");
    println!("✅ Exercise 07 passed! No sandwiches were lost. 🥪");
}
