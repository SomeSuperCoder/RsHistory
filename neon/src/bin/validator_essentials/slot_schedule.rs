pub const SLOT_SCHEDULE_LENGTH: usize = 5;

pub fn generate_slot_schedule(current_hieght: usize) -> [&'static str; SLOT_SCHEDULE_LENGTH] {
    let mut result: [&str; SLOT_SCHEDULE_LENGTH] = [""; SLOT_SCHEDULE_LENGTH];

    for slot in current_hieght+1..=current_hieght+5 {
        if slot % 2 == 0 {
            result[slot-current_hieght-1] = "1"
        } else {
            result[slot-current_hieght-1] = "2"
        }
    }

    result
}
