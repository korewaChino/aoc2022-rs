//! Advent of Code 2022 - Day 6
// please send help

// Notes:
// We are recieving a signal stream as a string of characters
// we need to find a packet header in the stream
// a header is a set of 4 unique characters
// it needs to identify the first position where the four most recently received characters were all different.
// Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.

pub fn generator(input: &str) -> String {
    input.to_string()
}

fn index(v: &u8) -> usize {
    (v - b'a') as usize
}


// orlp solution
fn find_disjoint_window(s: &[u8], w: usize) -> Option<usize> {
    let mut last_known_position = [0; 256];
    let mut start_disjoint = 0;
    for i in 0..s.len() {
        start_disjoint = start_disjoint.max(last_known_position[s[i] as usize] + 1);
        last_known_position[s[i] as usize] = i;
        if i >= start_disjoint + w {
            return Some(i);
        }
    }
    None
}

pub fn part_1(input: &str) -> usize {
    let bytes = input.trim().as_bytes();

    // I can't figure out how to actually use this
    find_disjoint_window(bytes, 4).unwrap()
}


pub fn part_2(input: &str) -> usize {
    let bytes = input.trim().as_bytes();

    // wtf?
    find_disjoint_window(bytes, 13).unwrap() + 1
}