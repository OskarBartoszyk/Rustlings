// ╔══════════════════════════════════════════════════════════╗
// ║  32 - FINAL BOSS: THE CRAB CAFE INVENTORY 🏆             ║
// ╚══════════════════════════════════════════════════════════╝
//
// You made it! Ferris needs your help one last time. His inventory arrives as
// messy text lines like  "apple, 3, 0.5"  (name, quantity, unit price), and
// some lines are garbage. Time to combine EVERYTHING: structs, Result, `?`,
// Vec, iterators, Option, closures and slices.
//
// Tools you'll want:
//   line.split(',')                         pieces of the line
//   .map(|p| p.trim()).collect::<Vec<&str>>()    trimmed pieces in a Vec
//   parts[1].parse::<u32>()                 Result<u32, ParseIntError>
//   .map_err(|e| e.to_string())             turn the error into a String
//   a.partial_cmp(&b).unwrap()              compare floats (they're not fully ordered)

#[derive(Debug, PartialEq)]
struct Item {
    name: String,
    qty: u32,
    price: f64,
}

impl Item {
    fn value(&self) -> f64 {
        self.qty as f64 * self.price
    }
}

// 🎯 BOSS STAGE 1: PARSE A LINE
// Turn "apple, 3, 0.5" into Item { name: "apple", qty: 3, price: 0.5 }.
//   * Split on ',' and trim each part, collecting into a Vec<&str>.
//   * If there aren't exactly 3 parts:
//         return Err(format!("expected 3 fields, got {}", parts.len()));
//   * Parse qty as u32 and price as f64. If either fails, return the error as
//     a String: `.map_err(|e| e.to_string())?`
//   * Otherwise return Ok(Item { ... }).  (`name: parts[0].to_string()`)
fn parse_line(line: &str) -> Result<Item, String> {
    todo!() // <-- replace this with your code
}

// 🎯 BOSS STAGE 2: TOTAL VALUE
// What's the whole stock worth? Add up `value()` of every item.
// (`.iter().map(|i| i.value()).sum()`)
fn total_value(items: &[Item]) -> f64 {
    todo!() // <-- replace this with your code
}

// 🎯 BOSS STAGE 3: THE PRIZE ITEM
// Which item has the highest `value()`? Return None for an empty slice.
// Hint: `.iter().max_by(|a, b| ...)` where the closure compares two values
// with `a.value().partial_cmp(&b.value()).unwrap()`.
fn most_valuable(items: &[Item]) -> Option<&Item> {
    todo!() // <-- replace this with your code
}

// 🎯 BOSS STAGE 4: CLEAN UP THE MESS
// Parse every line, but silently SKIP the lines that fail. 🗑️
// Hint: `.filter_map(|l| parse_line(l).ok()).collect()` (`.ok()` turns a Result
// into an Option, and `filter_map` keeps only the `Some`s).
fn parse_all(lines: &[&str]) -> Vec<Item> {
    todo!() // <-- replace this with your code
}

fn demo() {
    let items = parse_all(&["apple, 3, 0.5", "bad line", "pear, 2, 1.25"]);
    println!("{:?}", items);
    println!("total = {}", total_value(&items));
}

fn main() {
    demo();
    assert_eq!(
        parse_line("apple, 3, 0.5"),
        Ok(Item { name: "apple".into(), qty: 3, price: 0.5 })
    );
    assert_eq!(parse_line("a,b"), Err("expected 3 fields, got 2".to_string()));
    assert!(parse_line("a, x, 1.0").is_err());
    assert!(parse_line("a, 1, y").is_err());

    let items = parse_all(&["apple, 3, 0.5", "oops", "pear, 2, 1.25"]);
    assert_eq!(items.len(), 2);
    assert_eq!(total_value(&items), 4.0);
    assert_eq!(most_valuable(&items).unwrap().name, "pear");
    assert!(most_valuable(&[]).is_none());

    println!("🎉🦀 BOSS DEFEATED! You finished the course! Ferris throws a party in your honour. 🦀🎉");
}
