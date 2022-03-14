// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let production_rate: u32 = 221 * speed as u32;

    match speed {
        0 => 0_f64,
        1..=4 => production_rate as f64,
        5..=8 => 0.9 * production_rate as f64,
        9 | 10 => 0.77 * production_rate as f64,
        _ => panic!("bad speed value. Expect integer between 0 and 10"),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
