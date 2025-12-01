mod day_1;

fn main() {
    let input = include_str!("../inputs/day-1-input.txt");
    let answer = day_1::part_1(input);
    println!("The answer to day 1 part 1 is: {}", answer);
}
