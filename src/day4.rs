use crate::utils::read_lines;

pub fn day4part1(inputfilestr: &str) {
    if let Ok(lines) = read_lines(inputfilestr) {
        let mut total = 0;
        for line in lines {
            if let Ok(text) = line {
                let pairs: Vec<&str> = text.split(",").into_iter().take(2).collect();
                let range1 = pair_str_to_pair(pairs.first().unwrap());
                let range2 = pair_str_to_pair(pairs.last().unwrap());
                if (range1.0 <= range2.0 && range1.1 >= range2.1)
                || (range2.0 <= range1.0 && range2.1 >= range1.1){
                    total+=1;
                }
            }
        }
        println!("Part 1: {}", total);
    }
}

pub fn day4part2(inputfilestr: &str) {
    if let Ok(lines) = read_lines(inputfilestr) {
        let mut total = 0;
        for line in lines {
            if let Ok(text) = line {
                let pairs: Vec<&str> = text.split(",").into_iter().take(2).collect();
                let range1 = pair_str_to_pair(pairs.first().unwrap());
                let range2 = pair_str_to_pair(pairs.last().unwrap());
                let mut discreats = Vec::new();
                discreats.push(range1.0);
                discreats.push(range1.1);
                discreats.push(range2.0);
                discreats.push(range2.1);
                discreats.sort();
                let max = discreats.last().unwrap();
                let min = discreats.first().unwrap();
                let d1 = distance(range1);
                let d2 = distance(range2);
                if (d1 + d2) >= (max - min) {
                    total += 1;
                }
            }
        }
        println!("Part 2: {}", total);
    }
}

fn pair_str_to_pair(pair_str: &str) -> (i32, i32) {
    let strs: Vec<&str> = pair_str.split("-").into_iter().take(2).collect();
    let (str_x, str_y): (&str, &str) = (strs.first().unwrap(), strs.last().unwrap());
    (str_x.parse::<i32>().unwrap(), str_y.parse::<i32>().unwrap())
}

fn distance(pair: (i32, i32)) -> i32 {
    (pair.1 - pair.0).abs()
}