pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn part_1(input: &str) -> i32 {
    let elves = input
        .split("\n\n")
        .map(|group| {
            group
                .split_whitespace()
                .map(|line| line.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    // println!("groups: {:?}", elves);
    let elves_sum: Vec<i32> = elves.iter().map(|group| group.iter().sum()).collect();
    // println!("sum: {:?}", elves_sum);

    // find the largest number in the array and position
    let (_max, pos) = elves_sum
        .iter()
        .enumerate()
        .max_by_key(|&(_, x)| x)
        .unwrap();
    // println!("max: {:?}", max);
    // println!("pos: {:?}", pos);
    *pos
}

pub fn part_2(input: &str) -> i32 {
    let elves = input
    .split("\n\n")
    .map(|group| {
        group
            .split_whitespace()
            .map(|line| line.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    })
    .collect::<Vec<Vec<i32>>>();
    let elves_sum: Vec<i32> = elves.iter().map(|group| group.iter().sum()).collect();
    let mut top3 = elves_sum;
    top3.sort();
    top3.reverse();
    let top3 = top3[0..3].to_vec();
    // println!("top3: {:?}", top3);
    let top3_sum = top3.iter().sum::<i32>();
    // println!("top3_sum: {:?}", top3_sum);
    top3_sum

}
