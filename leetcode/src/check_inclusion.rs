pub struct Solution;

use std::cmp::min;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn test_check_inclusion () {
    println!("Check Inclusion (ab, aeibaoo) : {}",
             Solution::check_inclusion(String::from("ab"), String::from("aeibaoo")));
}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if (s2.len() < s1.len()) {
            false
        } else {
            // char -> current count, required, check_value
            let mut map :HashMap<char, [i32; 3]> = HashMap::new();
            for c in s1.chars() {
                let val = map.entry(c).or_insert([0, 0, 0]);
                val[1] += 1;
            }
            let mut ans = 0;
            let mut p1 = 0;
            let mut p2 = 0;
            let s2_chars :Vec<char> = s2.chars().collect();
            while p2 < s1.len() {
                let c = s2_chars[p2];
                Self::add_value(&mut map, c, &mut ans);
                p2 += 1;
            }

            if(ans as usize == s1.len()) {
                return true
            }

            while p2 < s2_chars.len() {
                let c1 = s2_chars[p1]; // Needs to be removed
                let c2 = s2_chars[p2]; // Needs to be added

                Self::remove_value(&mut map, c1, &mut ans);
                Self::add_value(&mut map, c2, &mut ans);

                if(ans as usize == s1.len()) {
                    return true
                }

                p1 += 1;
                p2 += 1;

            }
            false
        }
    }

    fn add_value(map :&mut HashMap<char, [i32; 3]>, c :char, ans :&mut i32) -> () {
        if let Entry::Occupied(mut entry) = map.entry(c) {
            let val = entry.get_mut();
            val[0] += 1; // current count
            *ans -= val[2];
            val[2] = min(val[0], val[1]);
            *ans += val[2];
        }
    }

    fn remove_value(map :&mut HashMap<char, [i32; 3]>, c :char, ans :&mut i32) -> () {
        if let Entry::Occupied(mut entry) = map.entry(c) {
            let val = entry.get_mut();
            val[0] -= 1; // current count
            *ans -= val[2];
            val[2] = min(val[0], val[1]);
            *ans += val[2];
        }
    }
}