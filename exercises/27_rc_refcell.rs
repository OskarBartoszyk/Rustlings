// ╔══════════════════════════════════════════════════════════╗
// ║  27 - Rc AND RefCell (sharing, the Python way)           ║
// ╚══════════════════════════════════════════════════════════╝
//
// Ownership says ONE owner. But sometimes several things truly need to own the
// same data (e.g. many nodes pointing at one shared node). In Python that's
// the default; in Rust you opt in:
//
// * `Rc<T>` = Reference Counted pointer. 🧮
//       let b = Rc::clone(&a);        // NOT a deep copy: just count + 1
//       Rc::strong_count(&a)          // how many owners right now
//   The data is freed when the LAST owner is gone. (Single-threaded only.)
//
// * But an Rc only gives READ access. To mutate shared data use `RefCell<T>`,
//   which checks the borrowing rules at RUNTIME instead of compile time:
//       let c = RefCell::new(5);
//       *c.borrow_mut() += 1;         // temporary mutable borrow
//       println!("{}", c.borrow());   // temporary shared borrow
//   Break the rules (two borrow_mut alive at once) and it PANICS.
//
// `Rc<RefCell<T>>` = "shared AND mutable", the closest thing to Python's
// everything-is-a-shared-mutable-object. Use it sparingly: normal
// ownership and borrowing are usually cleaner.

use std::cell::RefCell;
use std::rc::Rc;

fn demo() {
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a);
    println!("{a} {b} count={}", Rc::strong_count(&a));
    {
        let _c = Rc::clone(&a);
        println!("count={}", Rc::strong_count(&a));
    }
    println!("count={}", Rc::strong_count(&a));
}

fn main() {
    demo();

    // The Crab Cafe has ONE tip jar 🫙 shared by three crabs. `shared`, `a`
    // and `b` all point to the SAME counter.
    let shared = Rc::new(RefCell::new(0));
    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    // 🎯 MISSION
    // Crab `a` adds 5 coins and crab `b` adds 10 coins to the jar.
    // Oh no, they forgot to do it!
    // Use `*a.borrow_mut() += 5;` and the same for `b` with 10.
    todo!() // <-- replace this with your code

    assert_eq!(*shared.borrow(), 15);
    assert_eq!(Rc::strong_count(&shared), 3);
    println!("✅ Exercise 27 passed! 15 coins in the tip jar. 💰");
}
