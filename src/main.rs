mod day_1;

fn main() {
    let input = include_str!("../inputs/day-1-input.txt");
    let answer = day_1::part_1(input);
    let answer_part_2 = day_1::part_2(input);
    println!("The answer to day 1 part 1 is: {}", answer);
    println!("The answer to day 1 part 2 is: {}", answer_part_2);
}
