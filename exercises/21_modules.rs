// ╔══════════════════════════════════════════════════════════╗
// ║  21 - MODULES AND PRIVACY                                ║
// ╚══════════════════════════════════════════════════════════╝
//
// Python: everything is public, you just politely name private things `_x`.
// Rust:   EVERYTHING IS PRIVATE BY DEFAULT. 🔒 Slap a `pub` on whatever you
//         want the outside world to see.
//
//     mod math {
//         pub fn add(a: i32, b: i32) -> i32 { a + b }   // visible outside
//         fn secret() {}                                 // only inside `math`
//     }
//     math::add(1, 2)
//
// * Paths use `::`.  `use` imports a name (like Python's `from x import y`):
//        use std::collections::HashMap;
// * Struct FIELDS are private too unless marked `pub`.
// * In real projects `mod foo;` loads the file foo.rs. Here we use modules
//   written inline, so everything fits in one file.
// * `crate::` is your project's root, `super::` is the parent module.

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[allow(dead_code)]
    fn secret() -> i32 {
        42
    }

    // 🎯 MISSION
    // Ferris is organising his maths toolbox. 🧰 `math::cube(3)` is called in
    // `main`, but nobody wrote it! Add a function INSIDE this module:
    //
    //     pub fn cube(x: i32) -> i32 { ... }       (x * x * x)
    //
    // Don't forget `pub`, or `main` will get "function `cube` is private". 🙈
    // 📤 EXPECTED RESULT: Add a public function `cube(x: i32) -> i32` returning x * x * x. math::cube(3) must return 27.
    // TODO: write your code here
}

mod shapes {
    pub struct Square {
        pub side: u32,
        id: u32, // private field!
    }

    impl Square {
        pub fn new(side: u32) -> Self {
            Square { side, id: 1 }
        }
        pub fn id(&self) -> u32 {
            self.id
        }
    }
}

use shapes::Square;

fn demo() {
    println!("{}", math::add(1, 2));
    let s = Square::new(3);
    println!("{} {}", s.side, s.id());
    // println!("{}", s.id); // ❌ field `id` is private. Use the `id()` method.
}

fn main() {
    demo();
    assert_eq!(math::cube(3), 27);
    println!("✅ Exercise 21 passed! Privacy respected. 🕵️");
}
