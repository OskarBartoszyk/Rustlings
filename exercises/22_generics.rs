// ╔══════════════════════════════════════════════════════════╗
// ║  22 - GENERICS                                           ║
// ╚══════════════════════════════════════════════════════════╝
//
// In Python a function works with any type because nothing is checked until
// runtime (duck typing 🦆). Rust checks everything up front, so to write ONE
// function for MANY types you use a type placeholder, conventionally `T`:
//
//     fn first<T>(items: &[T]) -> &T { &items[0] }
//
// The compiler stamps out a specialised copy for each real type you use, so
// generics cost NOTHING at runtime.
//
// But hold on: if T could be anything, what can you DO with it? Only what you
// promise through TRAIT BOUNDS (next lesson explains traits):
//
//     fn largest<T: PartialOrd + Copy>(items: &[T]) -> T
//   = "T must be comparable with > and copyable".
//
// Structs and impls can be generic too:
//     struct Wrapper<T> { value: T }
//     impl<T> Wrapper<T> { ... }
//
// You already know generic types: Vec<T>, Option<T>, Result<T, E>, HashMap<K, V>.

fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut m = items[0];
    for &i in items {
        if i > m {
            m = i;
        }
    }
    m
}

struct Wrapper<T> {
    value: T,
}

fn demo() {
    println!("{}", largest(&[3, 9, 2]));
    println!("{}", largest(&[1.5, 0.2]));
    println!("{}", largest(&['a', 'z', 'c']));
    let w = Wrapper { value: "hi" };
    println!("{}", w.value);
}

// Ferris found a magic box 📦 that can hold ANYTHING: a number, a string, a
// whole other crab. Time to build it as a generic `Wrapper<T>`.
impl<T> Wrapper<T> {
    // 🎯 MISSION 1: the constructor. Put `value` inside the box.
    // 📤 EXPECTED RESULT: Return a Wrapper (Self) that stores `value`.
    fn new(value: T) -> Self {
        todo!() // <-- replace this with your code
    }

    // 🎯 MISSION 2: peek inside without taking the contents out.
    // Return a reference to the stored value (`&self.value`).
    // 📤 EXPECTED RESULT: Return a reference &T to the stored value (does not move it out).
    fn get(&self) -> &T {
        todo!() // <-- replace this with your code
    }
}

// 🎯 MISSION 3
// A pair of items (any two types, A and B) needs to be flipped around.
// Oh no, the types can be different! (1, "a") must become ("a", 1).
// Return a tuple with the two parts in reverse order. Hint: `pair.0` / `pair.1`.
// 📤 EXPECTED RESULT: Return a tuple (B, A) with the two parts reversed. swap_pair((1, "a")) -> ("a", 1).
fn swap_pair<A, B>(pair: (A, B)) -> (B, A) {
    todo!() // <-- replace this with your code
}

fn main() {
    demo();
    let w = Wrapper::new(42);
    assert_eq!(*w.get(), 42);
    let s = Wrapper::new(String::from("x"));
    assert_eq!(s.get(), "x");
    assert_eq!(swap_pair((1, "a")), ("a", 1));
    println!("✅ Exercise 22 passed! The magic box holds everything. 🎩");
}
