// ╔══════════════════════════════════════════════════════════╗
// ║  12 - STRUCTS                                            ║
// ╚══════════════════════════════════════════════════════════╝
//
// Python: a class with __init__ that sets some attributes.
// Rust:   a `struct` is a bag of named fields. Just DATA. (Behavior comes
//         in the next lesson with `impl`.)
//
//     struct User { name: String, age: u32, active: bool }
//     let u = User { name: String::from("Ann"), age: 30, active: true };
//     println!("{}", u.name);
//
// * To change a field, the WHOLE variable must be `mut` (no per-field mut).
// * FIELD INIT SHORTHAND: if a variable has the same name as the field you
//   can skip the repetition:  User { name, age, active: true }
// * STRUCT UPDATE SYNTAX: "same as that other one, except..."
//       User { age: 31, ..old_user }
// * Tuple structs: `struct Meters(f64);`  -> access with `m.0`
// * `#[derive(Debug)]` lets you print a struct with {:?} (or pretty {:#?}).

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
    active: bool,
}

struct Meters(f64);

fn demo() {
    let mut u = User { name: String::from("Ann"), age: 30, active: true };
    u.age += 1;
    println!("{:?}", u);

    let name = String::from("Bob");
    let u2 = User { name, age: 20, active: false }; // shorthand!
    let u3 = User { age: 21, ..u2.clone() };
    println!("{:#?}", u3);

    let m = Meters(3.5);
    println!("{} m", m.0);
}

// 🎯 MISSION 1
// Ferris is registering new members for the Crab Club. 🦀 Every new member
// starts out ACTIVE. Oh no, `make_user` is empty!
// Build and return a User from the `name` and `age` arguments, using field
// init shorthand (no `name: name`!) and `active: true`.
// 📤 EXPECTED RESULT: Return a User with the given name and age and `active` set to true.
fn make_user(name: String, age: u32) -> User {
    todo!() // <-- replace this with your code
}

// 🎯 MISSION 2
// A member stopped paying dues. Return a copy of `user` that is identical
// except `active` is false. Use struct update syntax with `..user.clone()`
// to fill in the remaining fields.
// 📤 EXPECTED RESULT: Return a NEW User identical to `user` but with `active` set to false.
fn deactivate(user: &User) -> User {
    todo!() // <-- replace this with your code
}

fn main() {
    demo();
    let u = make_user(String::from("Cy"), 40);
    assert_eq!(u.name, "Cy");
    assert_eq!(u.age, 40);
    assert!(u.active);
    let d = deactivate(&u);
    assert!(!d.active);
    assert_eq!(d.name, "Cy");
    println!("✅ Exercise 12 passed! The Crab Club membership list is perfect. 🪪");
}
