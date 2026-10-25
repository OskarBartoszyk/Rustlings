// ╔══════════════════════════════════════════════════════════╗
// ║  31 - WRITING YOUR OWN Iterator                          ║
// ╚══════════════════════════════════════════════════════════╝
//
// In Python you write __iter__ and __next__ (or use `yield`). In Rust you
// implement the `Iterator` trait for your type, and you only need ONE method:
// `next`. Every adaptor (map, filter, sum, collect...) comes for free!
//
//     impl Iterator for Counter {
//         type Item = u32;                       // what each step yields
//         fn next(&mut self) -> Option<u32> {    // Some(x) = here's an item
//             ...                                //  None    = I'm finished
//         }
//     }
//
// `type Item` is an "associated type": a type slot that every implementer fills in.

struct Countdown(u32);

// 🎯 MISSION
// A rocket launch countdown! 🚀 `Countdown(3)` should produce 3, 2, 1 and
// then stop. Oh no, `next` is empty, so the rocket never leaves the pad!
//
// `self.0` holds the NEXT number to hand out.
//   - if it is 0 -> we're done: return None
//   - otherwise remember the current number, decrease `self.0` by one,
//     and return Some(the number you remembered)
impl Iterator for Countdown {
    type Item = u32;

    // 📤 EXPECTED RESULT: Return Option<u32>: Some(current number) while counting down, None when finished. Countdown(3) yields 3, 2, 1, then None.
    fn next(&mut self) -> Option<u32> {
        todo!() // <-- replace this with your code
    }
}

fn demo() {
    for n in Countdown(3) {
        println!("{n}...");
    }
}

fn main() {
    demo();
    let v: Vec<u32> = Countdown(3).collect();
    assert_eq!(v, vec![3, 2, 1]);
    let total: u32 = Countdown(4).map(|x| x * 10).sum();
    assert_eq!(total, 100);
    assert_eq!(Countdown(0).count(), 0);
    println!("✅ Exercise 31 passed! Liftoff! 🚀");
}
