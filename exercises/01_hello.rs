// ╔══════════════════════════════════════════════════════════╗
// ║  01 - HELLO, RUST!                                       ║
// ╚══════════════════════════════════════════════════════════╝
//
// Meet FERRIS 🦀, the crab mascot of Rust. He will be your guide.
// Today is his first day at the Crab Cafe and he has to greet customers.
//
// Python:  print("Hi")
// Rust:    println!("Hi");
//
// What's new?
//  * `fn main()` is where every program starts. No `if __name__ == "__main__"`!
//  * Curly braces { } instead of indentation. Lines end with a semicolon `;`.
//  * `println!` has an exclamation mark because it is a MACRO (a function's
//    cooler cousin that can take any number of arguments).
//  * `{}` is a slot, like in an f-string:   println!("x = {}", x)
//    Or put the name right inside:           println!("x = {x}")
//  * `format!` is println!'s quiet sibling: it doesn't print, it RETURNS a String.
//
// 💡 Run me with:   python3 check.py 1

fn demo() {
    println!("Hello, world!");
    let name = "Ferris";
    println!("Hello, {}!", name);
    println!("Hello, {name}!");
    let s = format!("{} + {} = {}", 1, 2, 1 + 2);
    println!("{s}");
}

// 🎯 YOUR MISSION
// Oh no! Ferris is greeting every customer with... silence. 😶
// The `greeting` function should build the text "Hello, Ada!" for a customer
// named "Ada" (and "Hello, Bob!" for Bob, and so on).
//
// Don't print it - RETURN it. Use `format!` with a {} slot for the name.
fn greeting(name: &str) -> String {
    todo!() // <-- replace this with your code
}

fn main() {
    demo();
    assert_eq!(greeting("Ada"), "Hello, Ada!");
    assert_eq!(greeting("Bob"), "Hello, Bob!");
    println!("✅ Exercise 01 passed! Ferris waves his claws happily. 🦀");
}
