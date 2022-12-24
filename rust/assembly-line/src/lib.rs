// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const PRODUCTION_RATE: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let total = speed as f64 * PRODUCTION_RATE;
    match speed{
        0..=4 => total,
        5..=8 => total * 0.9,
        9..=10 => total * 0.77,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
