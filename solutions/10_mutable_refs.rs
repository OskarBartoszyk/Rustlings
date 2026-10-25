// ╔══════════════════════════════════════════════════════════╗
// ║  10 - MUTABLE REFERENCES                                 ║
// ╚══════════════════════════════════════════════════════════╝
//
// A `&T` lets you READ. A `&mut T` lets you READ AND WRITE. ✏️
//
//     fn add_bang(s: &mut String) { s.push('!'); }
//
//     let mut s = String::from("hi");   // the variable itself must be `mut`
//     add_bang(&mut s);                 // lend it out for editing
//
// THE GOLDEN RULE OF BORROWING 🏆
//   At any moment you can have EITHER
//      - as many shared refs (&T) as you like,   OR
//      - exactly ONE mutable ref (&mut T).
//   Never both at once.
//
//     let r1 = &mut s;
//     let r2 = &mut s;   // ❌ "cannot borrow `s` as mutable more than once"
//
// Think of a Google Doc where only ONE person may type at a time, and nobody
// reads while someone is typing. That rule kills a whole family of bugs, like
// changing a list while you loop over it.
//
// To WRITE through a `&mut i32` you must follow the pointer: `*x += 1;`

fn add_bang(s: &mut String) {
    s.push('!');
}

fn demo() {
    let mut s = String::from("hi");
    add_bang(&mut s);
    println!("{s}");

    let mut n = 1;
    {
        let r = &mut n;
        *r += 1;
    } // `r` is gone, so we're allowed to use `n` again
    println!("{n}");
}

// 🎯 MISSION 1
// Ferris's lucky counter lives somewhere else, and we only got a `&mut` to it.
// Oh no, nobody presses the button! Make the number behind `x` go up by one.
// Hint: write through the reference with `*x += 1;`
fn increment(x: &mut i32) {
    // [[
    *x += 1;
    // ]]
}

// 🎯 MISSION 2
// A basket of fish prices must be DOUBLED because of inflation. 📈 You get a
// mutable slice, so change the elements IN PLACE (don't create a new Vec).
// Hint: `for item in v.iter_mut()` gives you `&mut i32` items. Then `*item *= 2;`
fn double_all(v: &mut [i32]) {
    // [[
    for item in v.iter_mut() {
        *item *= 2;
    }
    // ]]
}

fn main() {
    demo();
    let mut n = 41;
    increment(&mut n);
    assert_eq!(n, 42);

    let mut arr = [1, 2, 3];
    double_all(&mut arr);
    assert_eq!(arr, [2, 4, 6]);
    println!("✅ Exercise 10 passed! The answer is 42, as usual. 🌌");
}
