// Advent of Code 2022, Day 1, Part 1

fn main() {
    // read input.txt
    let input = include_str!("day1.txt");

    // format is newline separated list of numbers, an array of array of numbers
    // each group is delimited by a blank line
    let elves = input
        .split("\n\n")
        .map(|group| {
            group
                .split_whitespace()
                .map(|line| line.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    println!("groups: {:?}", elves);

    let elves_sum: Vec<i32> = elves.iter().map(|group| group.iter().sum()).collect();

    println!("sum: {:?}", elves_sum);

    // find the largest number in the array and position
    let (max, pos) = elves_sum
        .iter()
        .enumerate()
        .max_by_key(|&(_, x)| x)
        .unwrap();
    println!("max: {:?}", max);
    println!("pos: {:?}", pos);
    // part 2


    // Now we get the top 3 numbers, then make a sum of those numbers
    let mut top3 = elves_sum.clone();
    top3.sort();
    top3.reverse();
    let top3 = top3[0..3].to_vec();
    println!("top3: {:?}", top3);
    let top3_sum = top3.iter().sum::<i32>();
    println!("top3_sum: {:?}", top3_sum);
}
