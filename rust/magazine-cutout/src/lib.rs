// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {

    let mut magazine_map: HashMap<&str, u32> = HashMap::new();
    let mut note_map: HashMap<&str, u32> = HashMap::new();

    for &word in magazine {
        let count = magazine_map.entry(word).or_insert(0);
        *count += 1;
    }

    for &word in note {
        let count = note_map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, val) in note_map {
        if magazine_map.get(key).unwrap_or(&0) < &val {
            return false;
        }
    }

    true
}
