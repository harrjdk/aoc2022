mod utils;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    println!("Advent of Code 2022!");
    println!("Day 1");
    day1::day1part1("input/day1.txt");
    day1::day1part2("input/day1.txt");
    println!("Day 2");
    day2::day2part1("input/day2.txt");
    day2::day2part2("input/day2.txt");
    println!("Day 3");
    day3::day3part1("input/day3.txt");
    day3::day3part2("input/day3.txt");
    println!("Day 4");
    day4::day4part1("input/day4.txt");
    day4::day4part2("input/day4.txt");
}
