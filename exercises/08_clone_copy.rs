// ╔══════════════════════════════════════════════════════════╗
// ║  08 - CLONE AND COPY (when moving is annoying)           ║
// ╚══════════════════════════════════════════════════════════╝
//
// Sometimes you want to keep your sandwich AND give one away. Two ways:
//
// 1. `.clone()` - an explicit, full duplicate. 🧬
//        let a = String::from("x");
//        let b = a.clone();     // both a and b are valid
//    It can be slow for big data, so Rust makes you ask for it out loud.
//    (Python copies references behind your back; Rust is honest about cost.)
//
// 2. `Copy` types - tiny stack-only values (integers, floats, bool, char,
//    and tuples of those) are copied automatically instead of moved.
//        let x = 5;  let y = x;   // x is still fine!
//    A String is NOT Copy, because it owns memory on the heap.
//
// Your own types can opt in with a #[derive(...)] line, which asks the
// compiler to write the boring code for you:
//     Debug      -> printable with {:?}
//     Clone      -> .clone()
//     Copy       -> copied instead of moved (needs Clone too)
//     PartialEq  -> can be compared with ==

fn demo() {
    let a = String::from("x");
    let b = a.clone();
    println!("{a} {b}");

    let x = 5;
    let y = x;
    println!("{x} {y}");
}

// 🎯 MISSION 1
// This `Point` is a pair of tiny numbers. It should behave like a number:
// assigning it somewhere must COPY it, not move it, and we need to compare
// and print points in the tests.
// Oh no, someone forgot the magic stickers! Add a derive line right above
// `struct Point` listing: Debug, Clone, Copy, PartialEq
//   -> #[derive(Debug, Clone, Copy, PartialEq)]
// 📤 EXPECTED RESULT: Only the derive line is needed - no variables or return values. The tests then compile and compare/print/copy points.
// TODO: write your code here
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    demo();

    // 🎯 MISSION 2
    // Ferris wants to keep a backup of his secret recipe before giving the
    // original away. The line below `moved = original` MOVES the recipe.
    // Create `copy` BEFORE that line, as an independent duplicate of `original`.
    // 📤 EXPECTED RESULT: Create a new variable `copy` of type String with the same text as `original` ("rust"), declared BEFORE the line `let moved = original;`.
    let original = String::from("rust");
    todo!() // <-- replace this with your code
    let moved = original; // `original` is gone after this line
    assert_eq!(copy, "rust");
    assert_eq!(moved, "rust");

    let a = Point { x: 1, y: 2 };
    let b = a; // a Copy type: `a` is still alive afterwards (needs MISSION 1)
    assert_eq!(a, b);

    println!("✅ Exercise 08 passed! The recipe is safe. 📜");
}
