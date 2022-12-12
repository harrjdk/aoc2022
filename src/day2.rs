use std::collections::HashMap;
use crate::utils::read_lines;
pub fn day2part1(inputfilestr: &str) {
    if let Ok(lines) = read_lines(inputfilestr) {
        let mut solutions = HashMap::new();
        solutions.insert("A X", 4);
        solutions.insert("A Y", 8);
        solutions.insert("A Z", 3);
        solutions.insert("B X", 1);
        solutions.insert("B Y", 5);
        solutions.insert("B Z", 9);
        solutions.insert("C X", 7);
        solutions.insert("C Y", 2);
        solutions.insert("C Z", 6);
        let mut sum = 0;
        for line in lines {
            if let Ok(text) = line {
                sum += solutions.get(&*text).unwrap();
            }
        }
        println!("Part 1: {}", sum);
    }
}
pub fn day2part2(inputfilestr: &str) {
    if let Ok(lines) = read_lines(inputfilestr) {
        let mut solutions = HashMap::new();
        solutions.insert("A X", 3);
        solutions.insert("A Y", 4);
        solutions.insert("A Z", 8);
        solutions.insert("B X", 1);
        solutions.insert("B Y", 5);
        solutions.insert("B Z", 9);
        solutions.insert("C X", 2);
        solutions.insert("C Y", 6);
        solutions.insert("C Z", 7);
        let mut sum = 0;
        for line in lines {
            if let Ok(text) = line {
                sum += solutions.get(&*text).unwrap();
            }
        }
        println!("Part 2: {}", sum);
    }
}