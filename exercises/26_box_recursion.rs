// ╔══════════════════════════════════════════════════════════╗
// ║  26 - BOX AND RECURSIVE TYPES                            ║
// ╚══════════════════════════════════════════════════════════╝
//
// Rust must know the exact SIZE of every type at compile time. Look at this:
//
//     enum List { Cons(i32, List), Nil }   // ❌ "recursive type has infinite size"
//
// A List contains a List contains a List... how big is that? Infinitely big!
// 🪆 (Like a Russian doll with no smallest doll.)
//
// `Box<T>` is a pointer to data on the HEAP. A pointer always has the same
// size, no matter what it points to. So:
//
//     enum List { Cons(i32, Box<List>), Nil }   // ✅ the doll is now a pointer
//
// * `Box::new(value)` puts something on the heap. It is freed when the Box
//   goes out of scope (ownership, remember?).
// * `*boxed` gets the value back out.
// * Use Box for: recursive types (lists, trees), huge values, and
//   `Box<dyn Trait>`.

use List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn demo() {
    let b = Box::new(5);
    println!("{}", *b + 1);

    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    if let Cons(first, _) = &list {
        println!("first = {first}");
    }
}

// 🎯 MISSION 1
// Ferris built a train of wagons 🚂: each `Cons(number, rest)` is a wagon
// carrying a number and the rest of the train, and `Nil` is the end.
// How many fish in total? Write it RECURSIVELY with `match`:
//   Cons(v, rest) -> v + sum(rest)
//   Nil           -> 0
// 📤 EXPECTED RESULT: Return an i32: the total of all numbers in the list (Nil -> 0).
fn sum(list: &List) -> i32 {
    todo!() // <-- replace this with your code
}

// 🎯 MISSION 2
// How many wagons does the train have? Same idea:
//   Cons(_, rest) -> 1 + length(rest)       (`_` = "I don't care about the number")
//   Nil           -> 0
// 📤 EXPECTED RESULT: Return a usize: the number of wagons in the list (Nil -> 0).
fn length(list: &List) -> usize {
    todo!() // <-- replace this with your code
}

fn main() {
    demo();
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    assert_eq!(sum(&list), 6);
    assert_eq!(length(&list), 3);
    assert_eq!(sum(&Nil), 0);
    println!("✅ Exercise 26 passed! Choo choo! 🚂");
}
