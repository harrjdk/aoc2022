use crate::utils::read_lines;

pub fn day1part1(inputfilestr: &str) {
    if let Ok(lines) = read_lines(inputfilestr) {
        let mut sums = Vec::new();
        let mut sum = 0;
        for line in lines {
            if let Ok(text) = line {
                if text.is_empty() {
                    sums.push(sum);
                    sum = 0;
                } else {
                    sum += text.parse::<i32>().unwrap();
                }
            }
        }
        sums.sort();
        println!("Part 1: {}", sums.last().unwrap());
    } else {
        println!("Could not find day1.txt input file!");
    }
}

pub fn day1part2(inputfilestr: &str) {
    if let Ok(lines) = read_lines(inputfilestr) {
        let mut sums = Vec::new();
        let mut sum = 0;
        for line in lines {
            if let Ok(text) = line {
                if text.is_empty() {
                    sums.push(sum);
                    sum = 0;
                } else {
                    sum += text.parse::<i32>().unwrap();
                }
            }
        }
        sums.sort();
        let total:i32 = sums.iter().rev().take(3).sum();
        println!("Part 2: {}", total);
    } else {
        println!("Could not find day1.txt input file!");
    }
}