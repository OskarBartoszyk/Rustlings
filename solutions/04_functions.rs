// ╔══════════════════════════════════════════════════════════╗
// ║  04 - FUNCTIONS (and the sneaky semicolon)               ║
// ╚══════════════════════════════════════════════════════════╝
//
// Python:  def add(a, b): return a + b
// Rust:    fn add(a: i32, b: i32) -> i32 { a + b }
//
// * Every parameter needs a type. The return type goes after `->`.
// * Rust is EXPRESSION-based: the LAST line of a block, written WITHOUT a
//   semicolon, is its value. That's the return value. No `return` needed!
//
//        fn five() -> i32 { 5 }     // ✅ returns 5
//        fn oops() -> i32 { 5; }    // ❌ the `;` swallows the value. Returns nothing!
//
//   That tiny `;` is the #1 beginner trap. If you ever see
//   "expected i32, found ()", look for a stray semicolon on the last line. 🕵️
//
// * `return x;` still exists - handy for leaving early.
// * A function with no `->` returns `()` ("unit"), which means "nothing useful".

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn abs_diff(a: i32, b: i32) -> i32 {
    if a > b {
        return a - b; // early exit
    }
    b - a // last line, no semicolon = the result
}

fn say_hi() {
    println!("hi");
}

fn demo() {
    say_hi();
    println!("{}", add(2, 3));
    println!("{}", abs_diff(3, 10));
}

// 🎯 MISSION 1
// Ferris is tiling the cafe floor. He needs the area of a square tile,
// given its side `x`. Return x * x.
// (Remember: NO semicolon on the last line!)
fn square(x: i32) -> i32 {
    // [[
    x * x
    // ]]
}

// 🎯 MISSION 2
// Table service! Customers with EVEN numbers sit on the left side of the cafe.
// Return `true` if `n` is even, `false` otherwise.
// Hint: `%` is the remainder operator, and `==` compares. A comparison is
// already a bool, so you don't need an `if` at all.
fn is_even(n: i32) -> bool {
    // [[
    n % 2 == 0
    // ]]
}

fn main() {
    demo();
    assert_eq!(square(7), 49);
    assert!(is_even(4));
    assert!(!is_even(7));
    println!("✅ Exercise 04 passed! Tiles laid, tables sorted. 🧱");
}
