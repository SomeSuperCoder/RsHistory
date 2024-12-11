pub mod history;
pub mod hash;

use std::time::Instant;

use history::HistoryPartData;
use history::HistoryPart;

fn main() {
    let mut history: Vec<HistoryPart> = Vec::new();

    let start = Instant::now();

    for i in 0..200_000 {
        let prev_hash;

        if i == 0 {
            prev_hash = String::from("capybara");
        } else {
            prev_hash = history.get(history.len() - 1).unwrap().hash.clone();
        }

        let new_part = HistoryPart::new(
            HistoryPartData::new(
                prev_hash,
                String::new()
            )
        );

        println!("{}", new_part.hash);
        history.push(new_part);
    }

    println!("Time for 200_000 PoH hashes: {:?}", start.elapsed())
}
