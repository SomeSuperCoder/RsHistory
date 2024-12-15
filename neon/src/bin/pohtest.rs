use std::time::Instant;
use neon::history::generate_slot;

fn main() {
    let start = Instant::now();
    generate_slot(String::from("capybara"));
    println!("Time for 350_000 PoH hashes: {:?}", start.elapsed())
}
