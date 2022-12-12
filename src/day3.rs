use std::collections::HashSet;
use crate::utils::read_lines;

static ASCII: [char; 52] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z', 'A', 'B', 'C', 'D',
    'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X',
    'Y', 'Z'
];

pub fn day3part1(inputfilestr: &str) {
    if let Ok(lines) = read_lines(inputfilestr) {
        let mut sum = 0;
        for line in lines {
            if let Ok(text) = line {
                let length = text.len();
                let (left, right) = text.split_at(length/2);
                let leftset: HashSet<char> = left.chars().collect();
                let rightset: HashSet<char> = right.chars().collect();
                let intersection: Vec<char> = leftset.intersection(&rightset).map(|x| *x).collect();
                let total: i32 = intersection.iter()
                    .map(|c| ASCII.iter().position(|&x| x == *c).unwrap() as i32 + 1).sum();
                sum += total;
            }
        }
        println!("Part 1: {}", sum);
    }
}

pub fn day3part2(inputfilestr: &str) {
    if let Ok(mut lines) = read_lines(inputfilestr) {
        let mut sum = 0;
        while let (Some(line1), Some(line2), Some(line3))
            = (lines.next(), lines.next(), lines.next()) {
            if let (Ok(text1), Ok(text2), Ok(text3)) = (line1, line2, line3) {
                let set1: HashSet<char> = text1.chars().collect();
                let set2: HashSet<char> = text2.chars().collect();
                let set3: HashSet<char> = text3.chars().collect();
                let mut sets = Vec::new();
                sets.push(set1);
                sets.push(set2);
                sets.push(set3);
                let overlap = intersection(sets);
                let total: i32 = overlap.iter()
                    .map(|c| ASCII.iter().position(|&x| x == *c).unwrap() as i32 + 1)
                    .sum();
                sum += total;
            }
        }
        println!("Part 2: {}", sum);
    }
}

fn intersection(mut sets: Vec<HashSet<char>>) -> HashSet<char> {
    if sets.is_empty() {
        return HashSet::new();
    }

    if sets.len() == 1 {
        return sets.pop().unwrap();
    }

    let mut result = sets.pop().unwrap();
    result.retain(|item| {
        sets.iter().all(|set| set.contains(item))
    });
    result
}