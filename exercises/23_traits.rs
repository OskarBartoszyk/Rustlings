// ╔══════════════════════════════════════════════════════════╗
// ║  23 - TRAITS                                             ║
// ╚══════════════════════════════════════════════════════════╝
//
// A TRAIT is a promise: "any type that wears this badge can do these things".
// It's Rust's version of an interface / abstract base class / Python Protocol.
//
//     trait Animal {
//         fn name(&self) -> String;                  // REQUIRED: you must write it
//         fn intro(&self) -> String {                // DEFAULT: you get it for free
//             format!("I am {}", self.name())
//         }
//     }
//
//     impl Animal for Dog {
//         fn name(&self) -> String { "Dog".to_string() }
//     }
//
// * There is NO inheritance in Rust. Instead, structs stay simple and you
//   attach behavior through traits. (Composition over inheritance, enforced!)
// * `impl Trait for Type` must provide every method without a default.
// * You've been using traits already: Debug, Clone, Copy, PartialEq, Iterator.
//   `#[derive(...)]` just auto-writes the `impl` for the common ones.

trait Animal {
    fn name(&self) -> String;
    fn speak(&self) -> String;

    // Default method: free for every implementer.
    fn intro(&self) -> String {
        format!("{} says {}", self.name(), self.speak())
    }
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn name(&self) -> String {
        "Dog".to_string()
    }
    fn speak(&self) -> String {
        "Woof".to_string()
    }
}

// 🎯 MISSION
// Ferris opened a pet corner in the cafe. 🐾 The dog is already working, but
// the CAT refuses to talk because nobody told the compiler she's an Animal!
// Fill in the `impl Animal for Cat` block:
//   name()  -> "Cat"
//   speak() -> "Meow"
// You only write those two methods; `intro()` comes for free from the trait.
impl Animal for Cat {
    // TODO: write your code here
}

fn demo() {
    println!("{}", Dog.intro());
}

fn main() {
    demo();
    assert_eq!(Dog.intro(), "Dog says Woof");
    assert_eq!(Cat.intro(), "Cat says Meow");
    println!("✅ Exercise 23 passed! The cat is talking (as usual, only when she wants). 🐱");
}
