// ╔══════════════════════════════════════════════════════════╗
// ║  25 - LIFETIMES (don't panic, it's just bookkeeping)     ║
// ╚══════════════════════════════════════════════════════════╝
//
// Every reference has a LIFETIME: how long the thing it points to is alive.
// Most of the time the compiler figures this out alone. But when a function
// takes TWO references and returns one, Rust can't guess which input the
// result is borrowed from. So you label them with a name like 'a:
//
//     fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
//
// Read it as: "the result is valid for as long as BOTH inputs are valid".
//
// Lifetimes never keep data alive longer. They just let the compiler PROVE
// that no reference outlives its data. Without them this would be a classic
// use-after-free bug:
//
//     let r;
//     { let x = 5; r = &x; }   // ❌ `x` does not live long enough
//     println!("{r}");         //    (x was thrown away at the `}`)
//
// * A struct that stores a reference needs a lifetime:
//       struct Excerpt<'a> { part: &'a str }
// * `'static` = lives for the whole program (string literals!).
// * Survival tip: if lifetimes fight you, return an owned `String` instead of
//   a `&str`. That is a perfectly respectable solution. 🏳️

struct Excerpt<'a> {
    part: &'a str,
}

fn demo() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first = novel.split('.').next().unwrap();
    let e = Excerpt { part: first };
    println!("{}", e.part);
}

// 🎯 MISSION 1
// Two crabs argue about who has the LONGER name. 🦀⚔️🦀 Return the longer
// string (if equal, return `a`). The signature with `'a` is already written;
// you just compare `.len()`s and return one of the two references.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    // [[
    if a.len() >= b.len() { a } else { b }
    // ]]
}

// 🎯 MISSION 2
// Ferris collects book quotes. 📖 Return an `Excerpt` whose `part` is the text
// up to the FIRST '.' (or the whole text if there's no dot).
// Hint: `text.split('.').next().unwrap_or(text)` gives that piece.
fn first_sentence<'a>(text: &'a str) -> Excerpt<'a> {
    // [[
    Excerpt { part: text.split('.').next().unwrap_or(text) }
    // ]]
}

fn main() {
    demo();
    assert_eq!(longest("apple", "fig"), "apple");
    assert_eq!(longest("a", "bb"), "bb");
    assert_eq!(first_sentence("Hi there. Bye.").part, "Hi there");
    assert_eq!(first_sentence("no dot").part, "no dot");
    println!("✅ Exercise 25 passed! No dangling references today. 🎈");
}
