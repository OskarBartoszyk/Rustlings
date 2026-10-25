// ╔══════════════════════════════════════════════════════════╗
// ║  24 - TRAITS IN SIGNATURES: impl, dyn and Display        ║
// ╚══════════════════════════════════════════════════════════╝
//
// How do you ask for "anything that implements Trait" in a function?
//
// 1. STATIC dispatch: decided at compile time, as fast as it gets.
//        fn show<T: Display>(x: &T)
//        fn show(x: &impl Display)        // same thing, shorter
//
// 2. DYNAMIC dispatch: decided at runtime (Python's duck typing, with rules).
//        Vec<Box<dyn Animal>>
//    A Vec holds ONE type, so how do you store a Dog and a Cat together?
//    As trait objects: "some type that implements Animal, don't ask which".
//    They must sit behind a pointer: `Box<dyn Animal>` or `&dyn Animal`.
//
// BONUS: `std::fmt::Display` is the trait behind `{}`. It's Python's __str__.
// Implement it and `println!("{}", my_thing)` just works.

use std::fmt;

trait Animal {
    fn name(&self) -> String;
    fn speak(&self) -> String;
    fn intro(&self) -> String {
        format!("{} says {}", self.name(), self.speak())
    }
}
struct Dog;
struct Cat;
impl Animal for Dog {
    fn name(&self) -> String { "Dog".into() }
    fn speak(&self) -> String { "Woof".into() }
}
impl Animal for Cat {
    fn name(&self) -> String { "Cat".into() }
    fn speak(&self) -> String { "Meow".into() }
}

struct Point {
    x: i32,
    y: i32,
}

// 🎯 MISSION 1
// Ferris wants `println!("{}", point)` to print a nice "(1, 2)" instead of
// the compiler complaining "Point doesn't implement Display". 🗺️
// `write!(f, "...", args...)` is like println!, but it writes into `f`.
// Make it produce "(x, y)", e.g. "(1, 2)".
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!() // <-- replace this with your code
    }
}

// Works for ANYTHING printable with {}. Already written for you:
fn bracketed(item: &impl fmt::Display) -> String {
    format!("<{}>", item)
}

// 🎯 MISSION 2
// The pet corner has a Dog AND a Cat in one list (`Box<dyn Animal>` makes that
// possible!). Collect every animal's `intro()` into a Vec<String>, in order.
// Hint: `animals.iter().map(|a| a.intro()).collect()`
fn describe_all(animals: &[Box<dyn Animal>]) -> Vec<String> {
    todo!() // <-- replace this with your code
}

fn demo() {
    println!("{}", bracketed(&42));
    println!("{}", bracketed(&"text"));
}

fn main() {
    demo();
    assert_eq!(format!("{}", Point { x: 1, y: 2 }), "(1, 2)");
    assert_eq!(bracketed(&Point { x: 3, y: 4 }), "<(3, 4)>");

    let zoo: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    assert_eq!(describe_all(&zoo), vec!["Dog says Woof", "Cat says Meow"]);
    println!("✅ Exercise 24 passed! A zoo of different types, one list. 🦁");
}
